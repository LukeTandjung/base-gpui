use std::{cell::RefCell, ops::Range, rc::Rc, sync::Arc, time::Duration};

use gpui::{
    point, AnyElement, App, Bounds, ElementId, Entity, FocusHandle, IntoElement, Modifiers,
    ParentElement as _, Pixels, ScrollStrategy, SharedString, Styled as _, UniformListScrollHandle,
    Window,
};

use super::{
    drag_and_drop::{
        drag_edge_scroll_delta, FileTreeDragAndDropConfig, FileTreeDropTarget,
        FileTreeDropTargetKind,
    },
    git_status::{
        apply_file_tree_git_status_patch, resolve_file_tree_git_status_state,
        FileTreeGitStatusEntry, FileTreeGitStatusPatch, FileTreeGitStatusState,
    },
    interaction::{compute_file_tree_row_click_plan, FileTreeRowClickMode},
    presentation::{
        FileTreeBuiltInFileIcon, FileTreeBuiltInIconSet, FileTreeComposition,
        FileTreeCompositionItem, FileTreeContextMenuOpenContext, FileTreeContextMenuSource,
        FileTreeDensityPreset, FileTreeDropState, FileTreeIconRenderContext, FileTreeIconResolver,
        FileTreeIconSlot, FileTreeIcons, FileTreeItemKind, FileTreeResolvedIconSource,
        FileTreeRowDecorationRenderer, FileTreeRowStyleWithState, FileTreeStyleWithState,
        FileTreeVisibleRow,
    },
    runtime::{FileTreeRuntime, FileTreeRuntimeUpdate},
    sticky::FileTreeStickyRow,
    FileTreeInitialExpansion, FileTreePreparedInput, FileTreeRenamingConfig, FileTreeSearchMode,
};

pub type FileTreeSelectionChangeHandler = Rc<dyn Fn(&[String], &mut Window, &mut App) + 'static>;
pub type FileTreeSearchChangeHandler = Rc<dyn Fn(Option<&str>, &mut Window, &mut App) + 'static>;
type FileTreeStickyRefreshHandler = Rc<dyn Fn(&mut App) + 'static>;

#[derive(Clone)]
struct FileTreeOpenContextMenu {
    row: FileTreeVisibleRow,
    anchor_bounds: Bounds<Pixels>,
    source: FileTreeContextMenuSource,
}

#[derive(Default)]
struct FileTreePresentationRuntimeState {
    git_status: Option<Arc<FileTreeGitStatusState>>,
    input_git_status: Vec<FileTreeGitStatusEntry>,
    hovered_path: Option<String>,
    pressed_path: Option<String>,
    context_menu: Option<FileTreeOpenContextMenu>,
}

pub struct FileTreeContext {
    runtime: Entity<FileTreeRuntime>,
    focus_handle: FocusHandle,
    search_focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    on_selection_change: Option<FileTreeSelectionChangeHandler>,
    on_search_change: Option<FileTreeSearchChangeHandler>,
    renaming: Option<FileTreeRenamingConfig>,
    drag_and_drop: Option<FileTreeDragAndDropConfig>,
    sticky_folders: bool,
    row_height: Pixels,
    sticky_refresh: Rc<RefCell<Option<FileTreeStickyRefreshHandler>>>,
    presentation_state: Rc<RefCell<FileTreePresentationRuntimeState>>,
    density: FileTreeDensityPreset,
    icons: FileTreeIconResolver,
    composition: FileTreeComposition,
    row_decoration: Option<FileTreeRowDecorationRenderer>,
    style_with_state: Option<FileTreeStyleWithState>,
    row_style_with_state: Option<FileTreeRowStyleWithState>,
    filename_max_chars: usize,
}

impl Clone for FileTreeContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            focus_handle: self.focus_handle.clone(),
            search_focus_handle: self.search_focus_handle.clone(),
            scroll_handle: self.scroll_handle.clone(),
            on_selection_change: self.on_selection_change.clone(),
            on_search_change: self.on_search_change.clone(),
            renaming: self.renaming.clone(),
            drag_and_drop: self.drag_and_drop.clone(),
            sticky_folders: self.sticky_folders,
            row_height: self.row_height,
            sticky_refresh: Rc::clone(&self.sticky_refresh),
            presentation_state: Rc::clone(&self.presentation_state),
            density: self.density,
            icons: self.icons.clone(),
            composition: self.composition.clone(),
            row_decoration: self.row_decoration.clone(),
            style_with_state: self.style_with_state.clone(),
            row_style_with_state: self.row_style_with_state.clone(),
            filename_max_chars: self.filename_max_chars,
        }
    }
}

impl FileTreeContext {
    pub fn new(
        id: impl Into<ElementId>,
        input: FileTreePreparedInput,
        initial_expansion: FileTreeInitialExpansion,
        search_mode: FileTreeSearchMode,
        flatten_empty_directories: bool,
        initial_search_query: Option<String>,
        initial_expanded_paths: Vec<String>,
        initial_selected_paths: Vec<String>,
        on_selection_change: Option<FileTreeSelectionChangeHandler>,
        on_search_change: Option<FileTreeSearchChangeHandler>,
        renaming: Option<FileTreeRenamingConfig>,
        drag_and_drop: Option<FileTreeDragAndDropConfig>,
        sticky_folders: bool,
        row_height: Pixels,
        density: FileTreeDensityPreset,
        icons: Option<FileTreeIcons>,
        git_status: Vec<FileTreeGitStatusEntry>,
        composition: FileTreeComposition,
        row_decoration: Option<FileTreeRowDecorationRenderer>,
        style_with_state: Option<FileTreeStyleWithState>,
        row_style_with_state: Option<FileTreeRowStyleWithState>,
        filename_max_chars: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let id = id.into();
        let initial_input = input.clone();
        let initial_expanded = initial_expanded_paths.clone();
        let initial_selected = initial_selected_paths.clone();
        let initial_search = initial_search_query.clone();
        let runtime = window.use_keyed_state(id.clone(), cx, move |_, _| {
            let mut runtime = FileTreeRuntime::new_with_configuration_and_flattening(
                &initial_input,
                initial_expansion,
                &initial_expanded,
                &initial_selected,
                search_mode,
                flatten_empty_directories,
            );
            if let Some(query) = initial_search.as_deref() {
                runtime.set_search(Some(query));
            }
            runtime
        });
        let update = runtime.update(cx, |runtime, cx| {
            let update = runtime.reconcile_with_flattening(
                &input,
                initial_expansion,
                &initial_expanded_paths,
                search_mode,
                flatten_empty_directories,
            );
            if update.state_changed {
                cx.notify();
            }
            update
        });
        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let search_focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("search-focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let scroll_handle_entity: Entity<UniformListScrollHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("scroll")),
            cx,
            |_, _| UniformListScrollHandle::new(),
        );
        let sticky_refresh_entity: Entity<Rc<RefCell<Option<FileTreeStickyRefreshHandler>>>> =
            window.use_keyed_state(
                ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("sticky-refresh")),
                cx,
                |_, _| Rc::new(RefCell::new(None)),
            );
        let initial_git_status = git_status.clone();
        let presentation_state_entity: Entity<Rc<RefCell<FileTreePresentationRuntimeState>>> =
            window.use_keyed_state(
                ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("presentation")),
                cx,
                move |_, _| {
                    Rc::new(RefCell::new(FileTreePresentationRuntimeState {
                        git_status: resolve_file_tree_git_status_state(&initial_git_status, None),
                        input_git_status: initial_git_status,
                        ..Default::default()
                    }))
                },
            );
        let presentation_state = presentation_state_entity.read(cx).clone();
        {
            let mut state = presentation_state.borrow_mut();
            if state.input_git_status != git_status {
                state.git_status =
                    resolve_file_tree_git_status_state(&git_status, state.git_status.as_ref());
                state.input_git_status = git_status;
            }
        }
        let context = Self {
            runtime,
            focus_handle: focus_handle_entity.read(cx).clone(),
            search_focus_handle: search_focus_handle_entity.read(cx).clone(),
            scroll_handle: scroll_handle_entity.read(cx).clone(),
            on_selection_change,
            on_search_change,
            renaming,
            drag_and_drop,
            sticky_folders,
            row_height,
            sticky_refresh: sticky_refresh_entity.read(cx).clone(),
            presentation_state,
            density,
            icons: FileTreeIconResolver::new(icons.as_ref()),
            composition,
            row_decoration,
            style_with_state,
            row_style_with_state,
            filename_max_chars,
        };
        context.emit_selection_change(update, window, cx);
        context
    }

    pub fn focus_handle(&self) -> &FocusHandle {
        &self.focus_handle
    }

    pub fn search_focus_handle(&self) -> &FocusHandle {
        &self.search_focus_handle
    }

    pub fn scroll_handle(&self) -> &UniformListScrollHandle {
        &self.scroll_handle
    }

    pub fn density(&self) -> FileTreeDensityPreset {
        self.density
    }

    pub fn render_icon(
        &self,
        slot: FileTreeIconSlot,
        path: Option<&str>,
        file_name: Option<&str>,
        item_kind: Option<FileTreeItemKind>,
        expanded: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let resolved = self.icons.resolve(slot, path);
        let (glyph, built_in) = match resolved.source {
            FileTreeResolvedIconSource::Render(render) => {
                return render(
                    FileTreeIconRenderContext {
                        slot,
                        path: path.map(str::to_owned),
                        file_name: file_name.map(str::to_owned),
                        item_kind,
                        expanded,
                    },
                    window,
                    cx,
                );
            }
            FileTreeResolvedIconSource::BuiltIn(icon) => {
                (built_in_file_icon_glyph(icon), Some(icon))
            }
            FileTreeResolvedIconSource::Slot(slot) => (semantic_icon_glyph(slot), None),
        };
        let icon = gpui::div()
            .w(gpui::px(16.0))
            .flex()
            .justify_center()
            .child(glyph);
        let icon = match built_in {
            Some(built_in_icon)
                if self.icons.config().colored
                    && self.icons.config().set == Some(FileTreeBuiltInIconSet::Complete) =>
            {
                icon.text_color(built_in_file_icon_color(built_in_icon))
            }
            _ => icon,
        };
        icon.into_any_element()
    }

    pub fn row_decoration(&self) -> Option<FileTreeRowDecorationRenderer> {
        self.row_decoration.clone()
    }

    pub fn style_with_state(&self) -> Option<FileTreeStyleWithState> {
        self.style_with_state.clone()
    }

    pub fn row_style_with_state(&self) -> Option<FileTreeRowStyleWithState> {
        self.row_style_with_state.clone()
    }

    pub fn filename_max_chars(&self) -> usize {
        self.filename_max_chars
    }

    pub fn header(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        self.composition
            .header
            .as_ref()
            .and_then(|header| header.render.as_ref())
            .and_then(|render| render(window, cx))
    }

    pub fn context_menu_open(&self) -> bool {
        self.presentation_state.borrow().context_menu.is_some()
    }

    pub fn context_menu_open_for_path(&self, path: &str) -> bool {
        self.presentation_state
            .borrow()
            .context_menu
            .as_ref()
            .is_some_and(|menu| menu.row.path == path)
    }

    pub fn hovered_path(&self) -> Option<String> {
        self.presentation_state.borrow().hovered_path.clone()
    }

    pub fn pressed_path(&self) -> Option<String> {
        self.presentation_state.borrow().pressed_path.clone()
    }

    pub fn set_hovered_path(&self, path: Option<String>, window: &mut Window, cx: &mut App) {
        if self.presentation_state.borrow().hovered_path == path {
            return;
        }
        self.presentation_state.borrow_mut().hovered_path = path;
        self.refresh_presentation(window, cx);
    }

    pub fn set_pressed_path(&self, path: Option<String>, window: &mut Window, cx: &mut App) {
        if self.presentation_state.borrow().pressed_path == path {
            return;
        }
        self.presentation_state.borrow_mut().pressed_path = path;
        self.refresh_presentation(window, cx);
    }

    pub fn drop_state_for_path(&self, path: &str, cx: &App) -> FileTreeDropState {
        self.read_runtime(cx, |runtime| {
            let Some(session) = runtime.drag_session() else {
                return FileTreeDropState::None;
            };
            if session.dragged_paths.iter().any(|dragged| dragged == path) {
                return FileTreeDropState::Dragging;
            }
            let Some(target) = session.target.as_ref() else {
                return FileTreeDropState::None;
            };
            if target.flattened_segment_path.as_deref() == Some(path) {
                return FileTreeDropState::FlattenedSegmentTarget {
                    path: path.to_owned(),
                };
            }
            if target.hovered_path.as_deref() == Some(path) {
                return match target.kind {
                    FileTreeDropTargetKind::Root => FileTreeDropState::RootTarget,
                    FileTreeDropTargetKind::Directory => FileTreeDropState::RowTarget {
                        path: path.to_owned(),
                    },
                };
            }
            FileTreeDropState::None
        })
    }

    pub fn git_status_for_path(&self, path: &str) -> Option<super::FileTreeGitStatus> {
        self.presentation_state
            .borrow()
            .git_status
            .as_ref()
            .and_then(|state| state.resolved_status_for_path(path))
    }

    pub fn has_git_status(&self) -> bool {
        self.presentation_state.borrow().git_status.is_some()
    }

    pub fn directory_has_git_changes(&self, path: &str) -> bool {
        self.presentation_state
            .borrow()
            .git_status
            .as_ref()
            .is_some_and(|state| state.directory_has_changes(path))
    }

    pub fn set_git_status(
        &self,
        entries: &[FileTreeGitStatusEntry],
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let previous = self.presentation_state.borrow().git_status.clone();
        let next = resolve_file_tree_git_status_state(entries, previous.as_ref());
        if same_optional_arc(&previous, &next) {
            return false;
        }
        self.presentation_state.borrow_mut().git_status = next;
        self.refresh_presentation(window, cx);
        true
    }

    pub fn apply_git_status_patch(
        &self,
        patch: &FileTreeGitStatusPatch,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let previous = self.presentation_state.borrow().git_status.clone();
        let next = apply_file_tree_git_status_patch(previous.as_ref(), patch);
        if same_optional_arc(&previous, &next) {
            return false;
        }
        self.presentation_state.borrow_mut().git_status = next;
        self.refresh_presentation(window, cx);
        true
    }

    pub fn context_menu_trigger_mode(&self) -> Option<super::FileTreeContextMenuTriggerMode> {
        self.composition
            .context_menu
            .as_ref()
            .filter(|menu| menu.enabled)
            .map(|menu| menu.trigger_mode)
    }

    pub fn context_menu_button_visibility(
        &self,
    ) -> Option<super::FileTreeContextMenuButtonVisibility> {
        self.composition
            .context_menu
            .as_ref()
            .filter(|menu| menu.enabled)
            .map(|menu| menu.button_visibility)
    }

    pub fn open_context_menu(
        &self,
        row: FileTreeVisibleRow,
        anchor_bounds: Bounds<Pixels>,
        source: FileTreeContextMenuSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(menu) = self
            .composition
            .context_menu
            .as_ref()
            .filter(|menu| menu.enabled)
            .cloned()
        else {
            return false;
        };
        self.context_click(&row.path, window, cx);
        let open = FileTreeOpenContextMenu {
            row,
            anchor_bounds,
            source,
        };
        self.presentation_state.borrow_mut().context_menu = Some(open.clone());
        let open_context = self.context_menu_context(open, cx);
        if let Some(on_open) = menu.on_open {
            on_open(open_context, window, cx);
        }
        self.refresh_presentation(window, cx);
        true
    }

    pub fn close_context_menu(
        &self,
        restore_focus: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if self
            .presentation_state
            .borrow_mut()
            .context_menu
            .take()
            .is_none()
        {
            return false;
        }
        if let Some(on_close) = self
            .composition
            .context_menu
            .as_ref()
            .and_then(|menu| menu.on_close.clone())
        {
            on_close(window, cx);
        }
        if restore_focus {
            self.focus_tree(window, cx);
        }
        self.refresh_presentation(window, cx);
        true
    }

    pub fn context_menu_surface(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let open = self.presentation_state.borrow().context_menu.clone()?;
        let render = self
            .composition
            .context_menu
            .as_ref()
            .filter(|menu| menu.enabled)
            .and_then(|menu| menu.render.clone())?;
        render(self.context_menu_context(open, cx), window, cx)
    }

    fn context_menu_context(
        &self,
        open: FileTreeOpenContextMenu,
        cx: &App,
    ) -> FileTreeContextMenuOpenContext {
        let close_context = self.clone();
        let drop_state = self.drop_state_for_path(&open.row.path, cx);
        FileTreeContextMenuOpenContext {
            item: FileTreeCompositionItem::from(&open.row),
            row: open.row,
            anchor_bounds: open.anchor_bounds,
            source: open.source,
            drop_state,
            close: Rc::new(move |restore_focus, window, cx| {
                close_context.close_context_menu(restore_focus, window, cx);
            }),
        }
    }

    fn refresh_presentation(&self, window: &mut Window, cx: &mut App) {
        self.runtime.update(cx, |_, cx| cx.notify());
        window.refresh();
    }

    pub fn focus_tree(&self, window: &mut Window, cx: &mut App) {
        self.focus_handle.focus(window, cx);
    }

    pub fn renaming_config(&self) -> Option<FileTreeRenamingConfig> {
        self.renaming.clone()
    }

    pub fn drag_and_drop_config(&self) -> Option<FileTreeDragAndDropConfig> {
        self.drag_and_drop.clone()
    }

    pub fn schedule_drag_hover_open(
        &self,
        generation: u64,
        target: FileTreeDropTarget,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(config) = self.drag_and_drop.as_ref() else {
            return;
        };
        let delay = config.open_on_drop_delay_value();
        if delay.is_zero() {
            return;
        }
        let runtime = self.runtime.clone();
        window
            .spawn(cx, async move |cx| {
                cx.background_executor().timer(delay).await;
                cx.update(|window, cx| {
                    runtime.update(cx, |runtime, cx| {
                        let update = runtime.tick_drag_hover(generation, &target);
                        if update.state_changed {
                            cx.notify();
                        }
                    });
                    window.refresh();
                })
                .ok();
            })
            .detach();
    }

    pub fn drag_auto_scroll(
        &self,
        pointer_y: Pixels,
        bounds: Bounds<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let point = (
            f32::from(pointer_y),
            f32::from(bounds.top()),
            f32::from(bounds.bottom()),
        );
        let Some(generation) = self.runtime.update(cx, |runtime, _| {
            runtime.arm_drag_auto_scroll(point.0, point.1, point.2)
        }) else {
            return false;
        };
        if !apply_drag_auto_scroll(&self.scroll_handle, point, window) {
            self.runtime
                .update(cx, |runtime, _| runtime.stop_drag_auto_scroll());
            return false;
        }
        self.schedule_sticky_refresh(window, cx);

        let runtime = self.runtime.clone();
        let scroll_handle = self.scroll_handle.clone();
        let sticky_folders = self.sticky_folders;
        let sticky_refresh = Rc::clone(&self.sticky_refresh);
        window
            .spawn(cx, async move |cx| loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                let keep_scrolling = cx
                    .update(|window, cx| {
                        let Some(point) = runtime.read(cx).drag_auto_scroll_point(generation)
                        else {
                            return false;
                        };
                        let keep_scrolling = apply_drag_auto_scroll(&scroll_handle, point, window);
                        if keep_scrolling && sticky_folders {
                            if let Some(refresh) = sticky_refresh.borrow().as_ref() {
                                refresh(cx);
                            }
                        }
                        keep_scrolling
                    })
                    .unwrap_or(false);
                if !keep_scrolling {
                    break;
                }
            })
            .detach();
        true
    }

    pub fn stop_drag_auto_scroll(&self, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _| runtime.stop_drag_auto_scroll());
    }

    pub fn begin_drag_target_capture(&self, cx: &mut App) -> Option<u64> {
        self.runtime
            .update(cx, |runtime, _| runtime.begin_drag_target_capture())
    }

    pub fn clear_unresolved_drag_target_capture(
        &self,
        generation: u64,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.runtime.update(cx, |runtime, cx| {
            let update = runtime.clear_unresolved_drag_target_capture(generation);
            if update.state_changed {
                cx.notify();
            }
        });
        window.refresh();
    }

    pub fn renaming_path(&self, cx: &App) -> Option<String> {
        self.runtime.read(cx).renaming_path().map(str::to_owned)
    }

    pub fn renaming_value(&self, cx: &App) -> String {
        self.runtime.read(cx).renaming_value().to_owned()
    }

    pub fn renaming_generation(&self, cx: &App) -> u64 {
        self.runtime.read(cx).renaming_generation()
    }

    pub fn visible_count(&self, cx: &App) -> usize {
        self.runtime.read(cx).visible_count()
    }

    pub fn visible_rows(&self, range: Range<usize>, cx: &App) -> Vec<FileTreeVisibleRow> {
        self.runtime.read(cx).visible_rows(range)
    }

    pub fn focused_row(&self, cx: &mut App) -> Option<FileTreeVisibleRow> {
        let index = self.update_runtime_cache(cx, |runtime| runtime.focused_index())?;
        self.visible_rows(index..index.saturating_add(1), cx)
            .into_iter()
            .next()
    }

    pub fn tree_drop_state(&self, cx: &App) -> FileTreeDropState {
        self.read_runtime(cx, |runtime| {
            let Some(session) = runtime.drag_session() else {
                return FileTreeDropState::None;
            };
            match session.target.as_ref() {
                Some(target) if target.kind == FileTreeDropTargetKind::Root => {
                    FileTreeDropState::RootTarget
                }
                Some(target) => FileTreeDropState::RowTarget {
                    path: target
                        .directory_path
                        .clone()
                        .or_else(|| target.hovered_path.clone())
                        .unwrap_or_default(),
                },
                None => FileTreeDropState::Dragging,
            }
        })
    }

    pub fn sticky_rows(&self, cx: &mut App) -> Vec<FileTreeStickyRow> {
        if !self.sticky_folders {
            return Vec::new();
        }
        let scroll_top = self.sticky_scroll_top(cx);
        let row_height = f32::from(self.row_height);
        self.update_runtime_cache(cx, |runtime| runtime.sticky_rows(scroll_top, row_height))
    }

    pub fn sticky_overlay_rows(&self, cx: &mut App) -> (Vec<FileTreeStickyRow>, bool) {
        if !self.sticky_folders {
            return (Vec::new(), false);
        }
        let scroll_top = self.sticky_scroll_top(cx);
        let visible = scroll_top > 0.0;
        let layout_scroll_top = if visible { scroll_top } else { 1.0 };
        let row_height = f32::from(self.row_height);
        let rows = self.update_runtime_cache(cx, |runtime| {
            runtime.sticky_rows(layout_scroll_top, row_height)
        });
        (rows, visible)
    }

    #[cfg(test)]
    pub fn scroll_top(&self) -> f32 {
        let base_handle = self.scroll_handle.0.borrow().base_handle.clone();
        f32::from((-base_handle.offset().y).max(Pixels::ZERO))
    }

    pub fn sticky_scroll_top(&self, cx: &App) -> f32 {
        let visible_count = self.visible_count(cx);
        let (actual_scroll_top, request, item_size) = {
            let state = self.scroll_handle.0.borrow();
            (
                (-state.base_handle.offset().y).max(Pixels::ZERO),
                state.deferred_scroll_to_item,
                state.last_item_size,
            )
        };
        let Some(request) = request else {
            return f32::from(actual_scroll_top);
        };
        let Some(item_size) = item_size else {
            return f32::from(actual_scroll_top);
        };
        if visible_count == 0 || self.row_height <= Pixels::ZERO {
            return 0.0;
        }

        let item_index = request.item_index.min(visible_count - 1);
        let item_top = self.row_height * item_index;
        let item_bottom = item_top + self.row_height;
        let offset = self.row_height * request.offset;
        let is_above = item_top < actual_scroll_top + offset;
        let is_below = item_bottom > actual_scroll_top + item_size.item.height;
        if !request.scroll_strict && !is_above && !is_below {
            return f32::from(actual_scroll_top);
        }

        let strategy = if request.strategy == ScrollStrategy::Nearest {
            if is_above {
                ScrollStrategy::Top
            } else if is_below {
                ScrollStrategy::Bottom
            } else {
                return f32::from(actual_scroll_top);
            }
        } else {
            request.strategy
        };
        let max_scroll_top =
            (self.row_height * visible_count - item_size.item.height).max(Pixels::ZERO);
        let target = match strategy {
            ScrollStrategy::Top => item_top - offset,
            ScrollStrategy::Center => {
                let item_center = item_top + self.row_height / 2.0;
                let viewport_height = item_size.item.height - offset;
                let viewport_center = offset + viewport_height / 2.0;
                item_center - viewport_center
            }
            ScrollStrategy::Bottom => item_bottom - item_size.item.height,
            ScrollStrategy::Nearest => actual_scroll_top,
        };
        f32::from(target.clamp(Pixels::ZERO, max_scroll_top))
    }

    pub fn scroll_to_visible_index(
        &self,
        index: usize,
        strategy: ScrollStrategy,
        strict: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let offset = self.sticky_inset_rows(index, cx);
        match (strict, offset) {
            (true, 0) => self.scroll_handle.scroll_to_item_strict(index, strategy),
            (true, offset) => self
                .scroll_handle
                .scroll_to_item_strict_with_offset(index, strategy, offset),
            (false, 0) => self.scroll_handle.scroll_to_item(index, strategy),
            (false, offset) => self
                .scroll_handle
                .scroll_to_item_with_offset(index, strategy, offset),
        }
        self.schedule_sticky_refresh(window, cx);
    }

    pub fn bind_sticky_refresh(&self, handler: FileTreeStickyRefreshHandler) {
        *self.sticky_refresh.borrow_mut() = Some(handler);
    }

    pub fn schedule_sticky_refresh(&self, window: &mut Window, cx: &mut App) {
        if !self.sticky_folders {
            return;
        }
        let sticky_refresh = Rc::clone(&self.sticky_refresh);
        window.defer(cx, move |window, cx| {
            if let Some(refresh) = sticky_refresh.borrow().as_ref() {
                refresh(cx);
            }
            let sticky_refresh = Rc::clone(&sticky_refresh);
            window.defer(cx, move |window, cx| {
                if let Some(refresh) = sticky_refresh.borrow().as_ref() {
                    refresh(cx);
                }
                window.refresh();
            });
            window.refresh();
        });
    }

    fn sticky_inset_rows(&self, index: usize, cx: &App) -> usize {
        if !self.sticky_folders || f32::from(self.row_height) <= 0.0 {
            return 0;
        }
        let ancestor_rows = self
            .runtime
            .read(cx)
            .sticky_inset_rows_for_visible_index(index);
        let viewport_rows = self
            .scroll_handle
            .0
            .borrow()
            .last_item_size
            .map(|size| (f32::from(size.item.height) / f32::from(self.row_height)).floor() as usize)
            .unwrap_or(ancestor_rows.saturating_add(1));
        ancestor_rows.min(viewport_rows.saturating_sub(1))
    }

    pub fn search_value(&self, cx: &App) -> String {
        self.runtime.read(cx).search_value().to_owned()
    }

    pub fn is_search_open(&self, cx: &App) -> bool {
        self.runtime.read(cx).is_search_open()
    }

    pub fn set_search(
        &self,
        value: Option<&str>,
        window: &mut Window,
        cx: &mut App,
    ) -> FileTreeRuntimeUpdate {
        self.update_runtime(window, cx, |runtime| runtime.set_search(value))
    }

    pub fn close_search(&self, window: &mut Window, cx: &mut App) -> FileTreeRuntimeUpdate {
        self.set_search(None, window, cx)
    }

    pub fn close_search_and_focus_tree(&self, window: &mut Window, cx: &mut App) {
        self.close_search(window, cx);
        self.focus_tree(window, cx);
    }

    pub fn focus_next_search_match(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_next_search_match);
    }

    pub fn focus_previous_search_match(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_previous_search_match);
    }

    pub fn select_focused_search_match(&self, window: &mut Window, cx: &mut App) {
        let focused_path = self.runtime.read(cx).focused_path().map(str::to_owned);
        if let Some(path) = focused_path {
            self.update_runtime(window, cx, |runtime| runtime.select_only_path(&path));
        }
    }

    pub fn read_runtime<R>(&self, cx: &App, read: impl FnOnce(&FileTreeRuntime) -> R) -> R {
        read(self.runtime.read(cx))
    }

    pub fn update_runtime_cache<R>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut FileTreeRuntime) -> R,
    ) -> R {
        self.runtime.update(cx, |runtime, _| update(runtime))
    }

    pub fn row_click(
        &self,
        path: &str,
        is_directory: bool,
        modifiers: Modifiers,
        window: &mut Window,
        cx: &mut App,
    ) -> FileTreeRuntimeUpdate {
        self.row_click_with_mode(
            path,
            is_directory,
            modifiers,
            FileTreeRowClickMode::Flow,
            window,
            cx,
        )
    }

    pub fn sticky_row_click(
        &self,
        path: &str,
        is_directory: bool,
        modifiers: Modifiers,
        window: &mut Window,
        cx: &mut App,
    ) -> FileTreeRuntimeUpdate {
        self.row_click_with_mode(
            path,
            is_directory,
            modifiers,
            FileTreeRowClickMode::Sticky,
            window,
            cx,
        )
    }

    fn row_click_with_mode(
        &self,
        path: &str,
        is_directory: bool,
        modifiers: Modifiers,
        mode: FileTreeRowClickMode,
        window: &mut Window,
        cx: &mut App,
    ) -> FileTreeRuntimeUpdate {
        self.focus_tree(window, cx);
        let plan = compute_file_tree_row_click_plan(
            modifiers.shift,
            modifiers.control,
            modifiers.platform,
            mode,
            is_directory,
        );
        let reveal_canonical = plan.reveal_canonical;
        let update = self.update_runtime(window, cx, |runtime| runtime.apply_row_click(path, plan));
        if reveal_canonical {
            if let Some(index) =
                self.update_runtime_cache(cx, |runtime| runtime.visible_index_for_path(path))
            {
                self.scroll_to_visible_index(index, ScrollStrategy::Top, true, window, cx);
                window.refresh();
            }
        }
        update
    }

    pub fn context_click(
        &self,
        path: &str,
        window: &mut Window,
        cx: &mut App,
    ) -> FileTreeRuntimeUpdate {
        self.focus_tree(window, cx);
        self.update_runtime(window, cx, |runtime| runtime.apply_context_click(path))
    }

    pub fn focus_next(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_next_item);
    }

    pub fn focus_previous(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_previous_item);
    }

    pub fn focus_first(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_first_item);
    }

    pub fn focus_last(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_last_item);
    }

    pub fn page_up(&self, window: &mut Window, cx: &mut App) {
        self.scroll_page(false);
        self.schedule_sticky_refresh(window, cx);
    }

    pub fn page_down(&self, window: &mut Window, cx: &mut App) {
        self.scroll_page(true);
        self.schedule_sticky_refresh(window, cx);
    }

    pub fn focus_right(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_right);
    }

    pub fn focus_left(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::focus_left);
    }

    pub fn extend_next(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, |runtime| {
            runtime.extend_selection_from_focused(1)
        });
    }

    pub fn extend_previous(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, |runtime| {
            runtime.extend_selection_from_focused(-1)
        });
    }

    pub fn activate_focused(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime_and_scroll(window, cx, FileTreeRuntime::activate_focused);
    }

    pub fn toggle_focused_selection(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime(window, cx, FileTreeRuntime::toggle_focused_selection);
    }

    pub fn select_all_visible(&self, window: &mut Window, cx: &mut App) {
        self.update_runtime(window, cx, FileTreeRuntime::select_all_visible_paths);
    }

    fn scroll_page(&self, forward: bool) {
        let (base_handle, item_size) = {
            let state = self.scroll_handle.0.borrow();
            (state.base_handle.clone(), state.last_item_size)
        };
        let Some(item_size) = item_size else {
            return;
        };
        let offset = base_handle.offset();
        let current_scroll_top = (-offset.y).max(Pixels::ZERO);
        let next_scroll_top = page_scroll_top(
            current_scroll_top,
            item_size.item.height,
            item_size.contents.height,
            forward,
        );
        base_handle.set_offset(point(offset.x, -next_scroll_top));
    }

    fn update_runtime_and_scroll(
        &self,
        window: &mut Window,
        cx: &mut App,
        update: impl FnOnce(&mut FileTreeRuntime) -> FileTreeRuntimeUpdate,
    ) -> FileTreeRuntimeUpdate {
        let update = self.update_runtime(window, cx, update);
        if update.state_changed {
            self.scroll_focused(window, cx);
        }
        update
    }

    fn scroll_focused(&self, window: &mut Window, cx: &mut App) {
        if let Some(index) = self.runtime.read(cx).focused_index() {
            self.scroll_to_visible_index(index, ScrollStrategy::Nearest, false, window, cx);
        }
    }

    pub fn update_runtime(
        &self,
        window: &mut Window,
        cx: &mut App,
        update: impl FnOnce(&mut FileTreeRuntime) -> FileTreeRuntimeUpdate,
    ) -> FileTreeRuntimeUpdate {
        let previous_search_value = self
            .runtime
            .read(cx)
            .search_value_option()
            .map(str::to_owned);
        let update = self.runtime.update(cx, |runtime, cx| {
            let update = update(runtime);
            if update.state_changed {
                cx.notify();
            }
            update
        });
        if update.state_changed && self.sticky_folders {
            if let Some(refresh) = self.sticky_refresh.borrow().as_ref() {
                refresh(cx);
            }
        }
        self.emit_selection_change(update, window, cx);
        let search_value = self
            .runtime
            .read(cx)
            .search_value_option()
            .map(str::to_owned);
        if previous_search_value != search_value {
            if let Some(on_search_change) = self.on_search_change.as_ref() {
                on_search_change(search_value.as_deref(), window, cx);
            }
        }
        update
    }

    fn emit_selection_change(
        &self,
        update: FileTreeRuntimeUpdate,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !update.selection_changed {
            return;
        }
        let selected_paths = self.runtime.read(cx).selected_paths().to_vec();
        if let Some(on_selection_change) = self.on_selection_change.as_ref() {
            on_selection_change(&selected_paths, window, cx);
        }
    }
}

fn apply_drag_auto_scroll(
    scroll_handle: &UniformListScrollHandle,
    (pointer_y, top, bottom): (f32, f32, f32),
    window: &mut Window,
) -> bool {
    let delta = drag_edge_scroll_delta(pointer_y, top, bottom);
    if delta == 0.0 {
        return false;
    }
    let (base_handle, item_size) = {
        let state = scroll_handle.0.borrow();
        (state.base_handle.clone(), state.last_item_size)
    };
    let Some(item_size) = item_size else {
        return false;
    };
    let offset = base_handle.offset();
    let scroll_top = (-offset.y).max(Pixels::ZERO);
    let max_scroll_top = (item_size.contents.height - item_size.item.height).max(Pixels::ZERO);
    let next_scroll_top = (scroll_top + gpui::px(delta)).clamp(Pixels::ZERO, max_scroll_top);
    if next_scroll_top == scroll_top {
        return false;
    }
    base_handle.set_offset(point(offset.x, -next_scroll_top));
    window.refresh();
    true
}
fn semantic_icon_glyph(slot: FileTreeIconSlot) -> &'static str {
    match slot {
        FileTreeIconSlot::File => "·",
        FileTreeIconSlot::Folder => "□",
        FileTreeIconSlot::OpenFolder => "▾",
        FileTreeIconSlot::DisclosureCollapsed => ">",
        FileTreeIconSlot::DisclosureExpanded => "v",
        FileTreeIconSlot::Search => "?",
        FileTreeIconSlot::Rename => "✎",
        FileTreeIconSlot::ContextMenu => "…",
        FileTreeIconSlot::GitChangeDot => "•",
        FileTreeIconSlot::Lock => "▣",
    }
}

fn built_in_file_icon_glyph(icon: FileTreeBuiltInFileIcon) -> &'static str {
    use FileTreeBuiltInFileIcon as Icon;
    match icon {
        Icon::Astro => "A",
        Icon::Babel => "B",
        Icon::Bash => "$",
        Icon::Biome => "B",
        Icon::Bootstrap => "B",
        Icon::Browserslist => "B",
        Icon::Bun => "B",
        Icon::C => "C",
        Icon::Claude => "C",
        Icon::Cpp => "C+",
        Icon::Css => "#",
        Icon::Database => "DB",
        Icon::Default => "·",
        Icon::Docker => "D",
        Icon::Eslint => "E",
        Icon::Font => "F",
        Icon::Git => "G",
        Icon::Go => "Go",
        Icon::Graphql => "◇",
        Icon::Html => "<>",
        Icon::Image => "▧",
        Icon::Javascript => "JS",
        Icon::Json => "{}",
        Icon::Markdown => "MD",
        Icon::Mcp => "M",
        Icon::Nextjs => "N",
        Icon::Npm => "N",
        Icon::Oxc => "O",
        Icon::Postcss => "P",
        Icon::Prettier => "P",
        Icon::Python => "Py",
        Icon::React => "⚛",
        Icon::Ruby => "Rb",
        Icon::Rust => "Rs",
        Icon::Sass => "S",
        Icon::Stylelint => "S",
        Icon::Svelte => "Sv",
        Icon::Svg => "SVG",
        Icon::Svgo => "SO",
        Icon::Swift => "Sw",
        Icon::Table => "▦",
        Icon::Tailwind => "Tw",
        Icon::Terraform => "Tf",
        Icon::Text => "T",
        Icon::Typescript => "TS",
        Icon::Vite => "V",
        Icon::Vscode => "VS",
        Icon::Vue => "V",
        Icon::Wasm => "W",
        Icon::Webpack => "Wp",
        Icon::Yml => "Y",
        Icon::Zig => "Z",
        Icon::Zip => "ZIP",
    }
}

fn built_in_file_icon_color(icon: FileTreeBuiltInFileIcon) -> gpui::Rgba {
    use FileTreeBuiltInFileIcon as Icon;
    match icon {
        Icon::Javascript | Icon::Json | Icon::Bun => gpui::rgb(0xf0db4f),
        Icon::Typescript | Icon::Css | Icon::C | Icon::Cpp | Icon::Swift => gpui::rgb(0x4f8fcc),
        Icon::React | Icon::Go | Icon::Tailwind => gpui::rgb(0x61dafb),
        Icon::Vue | Icon::Svelte | Icon::Astro => gpui::rgb(0x42b883),
        Icon::Html | Icon::Rust | Icon::Git | Icon::Docker => gpui::rgb(0xe76f51),
        Icon::Ruby | Icon::Npm | Icon::Graphql => gpui::rgb(0xdc143c),
        Icon::Python | Icon::Markdown | Icon::Yml | Icon::Text => gpui::rgb(0xf4c95d),
        Icon::Default | Icon::Database | Icon::Font | Icon::Image | Icon::Table | Icon::Zip => {
            gpui::rgb(0x9ca3af)
        }
        _ => gpui::rgb(0xa78bfa),
    }
}

fn same_optional_arc<T>(left: &Option<Arc<T>>, right: &Option<Arc<T>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => Arc::ptr_eq(left, right),
        _ => false,
    }
}

fn page_scroll_top(
    current_scroll_top: Pixels,
    viewport_height: Pixels,
    content_height: Pixels,
    forward: bool,
) -> Pixels {
    let max_scroll_top = (content_height - viewport_height).max(Pixels::ZERO);
    let next_scroll_top = if forward {
        current_scroll_top + viewport_height
    } else {
        current_scroll_top - viewport_height
    };
    next_scroll_top.clamp(Pixels::ZERO, max_scroll_top)
}

#[cfg(test)]
mod tests {
    use gpui::px;

    use super::{built_in_file_icon_color, built_in_file_icon_glyph, page_scroll_top};
    use crate::file_tree::FileTreeBuiltInFileIcon;

    #[test]
    fn built_in_file_tokens_select_distinct_glyphs_and_colors() {
        assert_ne!(
            built_in_file_icon_glyph(FileTreeBuiltInFileIcon::Default),
            built_in_file_icon_glyph(FileTreeBuiltInFileIcon::Typescript)
        );
        assert_ne!(
            built_in_file_icon_color(FileTreeBuiltInFileIcon::Typescript),
            built_in_file_icon_color(FileTreeBuiltInFileIcon::Rust)
        );
    }

    #[test]
    fn page_scroll_moves_one_viewport_and_clamps_to_content_bounds() {
        assert_eq!(
            page_scroll_top(px(0.0), px(100.0), px(450.0), true),
            px(100.0)
        );
        assert_eq!(
            page_scroll_top(px(320.0), px(100.0), px(450.0), true),
            px(350.0)
        );
        assert_eq!(
            page_scroll_top(px(250.0), px(100.0), px(450.0), false),
            px(150.0)
        );
        assert_eq!(
            page_scroll_top(px(40.0), px(100.0), px(450.0), false),
            px(0.0)
        );
        assert_eq!(page_scroll_top(px(0.0), px(100.0), px(80.0), true), px(0.0));
    }
}
