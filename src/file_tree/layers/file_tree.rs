use std::{rc::Rc, sync::Arc};

use gpui::{
    div, point, prelude::FluentBuilder as _, px, size, uniform_list, AnyElement, App, Bounds,
    Context, Div, ElementId, Entity, InteractiveElement as _, IntoElement, KeyDownEvent,
    MouseButton, ParentElement as _, Pixels, Render, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::file_tree::{
    actions::{
        FileTreeActivateFocused, FileTreeCollapseOrParent, FileTreeExpandOrNext,
        FileTreeExtendNext, FileTreeExtendPrevious, FileTreeMoveFirst, FileTreeMoveLast,
        FileTreeMoveNext, FileTreeMovePrevious, FileTreePageDown, FileTreePageUp,
        FileTreeSelectAllVisible, FileTreeToggleFocusedSelection, FILE_TREE_KEY_CONTEXT,
    },
    context::{FileTreeContext, FileTreeSearchChangeHandler, FileTreeSelectionChangeHandler},
    controller::{
        bind_file_tree_handle, cancel_file_tree_renaming, commit_file_tree_renaming,
        file_tree_handle_identity,
    },
    layers::{
        FileTreeInternalDragPayload, FileTreeRow, FileTreeSearchInput, FileTreeSearchInteraction,
    },
    sticky::{occluded_row_range, sticky_height},
    FileTreeComposition, FileTreeContextMenuComposition, FileTreeContextMenuSource,
    FileTreeDensity, FileTreeDragAndDropConfig, FileTreeGitStatusEntry, FileTreeHandle,
    FileTreeHeaderComposition, FileTreeIconSlot, FileTreeIcons, FileTreeInitialExpansion,
    FileTreeInputError, FileTreePreparedInput, FileTreeRenamingConfig,
    FileTreeRowDecorationRenderer, FileTreeRowStyleWithState, FileTreeSearchBlurBehavior,
    FileTreeSearchMode, FileTreeStyleState, FileTreeStyleWithState,
};

struct FileTreeStickyOverlay {
    context: FileTreeContext,
    handle: FileTreeHandle,
    row_height: Pixels,
}

impl Render for FileTreeStickyOverlay {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (sticky_rows, visible) = self.context.sticky_overlay_rows(cx);
        if sticky_rows.is_empty() {
            return div().into_any_element();
        }
        let sticky_context = self.context.clone();
        let sticky_handle = self.handle.clone();
        let row_height = self.row_height;
        div()
            .id("file-tree-sticky-overlay")
            .debug_selector(|| "file-tree-sticky-overlay".to_owned())
            .when(!visible, |this| this.invisible())
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            // GPUI has no CSS z-index builder. Paint inner rows first so outer
            // rows win paint and hit testing when subtree push-off overlaps slots.
            .children(sticky_rows.into_iter().rev().map(move |entry| {
                FileTreeRow::sticky(
                    sticky_context.clone(),
                    sticky_handle.clone(),
                    entry.row,
                    row_height,
                    entry.top,
                )
            }))
            .into_any_element()
    }
}

/// A path-first virtualized file tree.
#[derive(IntoElement)]
pub struct FileTree {
    id: ElementId,
    base: Div,
    input: FileTreePreparedInput,
    initial_expansion: FileTreeInitialExpansion,
    initial_expanded_paths: Vec<String>,
    flatten_empty_directories: bool,
    search_mode: FileTreeSearchMode,
    search: bool,
    initial_search_query: Option<String>,
    search_blur_behavior: FileTreeSearchBlurBehavior,
    initial_selected_paths: Vec<String>,
    on_selection_change: Option<FileTreeSelectionChangeHandler>,
    on_search_change: Option<FileTreeSearchChangeHandler>,
    renaming: Option<FileTreeRenamingConfig>,
    drag_and_drop: Option<FileTreeDragAndDropConfig>,
    handle: Option<FileTreeHandle>,
    sticky_folders: bool,
    density: FileTreeDensity,
    explicit_row_height: Option<Pixels>,
    icons: Option<FileTreeIcons>,
    git_status: Vec<FileTreeGitStatusEntry>,
    composition: FileTreeComposition,
    row_decoration: Option<FileTreeRowDecorationRenderer>,
    style_with_state: Option<FileTreeStyleWithState>,
    row_style_with_state: Option<FileTreeRowStyleWithState>,
    filename_max_chars: usize,
}

impl FileTree {
    pub fn new(input: FileTreePreparedInput) -> Self {
        Self {
            id: ElementId::from("file-tree"),
            base: div().size_full(),
            input,
            initial_expansion: FileTreeInitialExpansion::Closed,
            initial_expanded_paths: Vec::new(),
            flatten_empty_directories: false,
            search_mode: FileTreeSearchMode::default(),
            search: false,
            initial_search_query: None,
            search_blur_behavior: FileTreeSearchBlurBehavior::default(),
            initial_selected_paths: Vec::new(),
            on_selection_change: None,
            on_search_change: None,
            renaming: None,
            drag_and_drop: None,
            handle: None,
            sticky_folders: false,
            density: FileTreeDensity::default(),
            explicit_row_height: None,
            icons: None,
            git_status: Vec::new(),
            composition: FileTreeComposition::default(),
            row_decoration: None,
            style_with_state: None,
            row_style_with_state: None,
            filename_max_chars: 48,
        }
    }

    pub fn from_paths(
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, FileTreeInputError> {
        FileTreePreparedInput::prepare(paths).map(Self::new)
    }

    pub fn from_presorted_paths(
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, FileTreeInputError> {
        FileTreePreparedInput::presorted(paths).map(Self::new)
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn initial_expansion(mut self, initial_expansion: FileTreeInitialExpansion) -> Self {
        self.initial_expansion = initial_expansion;
        self
    }

    pub fn initial_expanded_paths(
        mut self,
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.initial_expanded_paths = paths.into_iter().map(Into::into).collect();
        self
    }

    pub fn flatten_empty_directories(mut self, enabled: bool) -> Self {
        self.flatten_empty_directories = enabled;
        self
    }

    pub fn search_mode(mut self, search_mode: FileTreeSearchMode) -> Self {
        self.search_mode = search_mode;
        self
    }

    pub fn search(mut self, enabled: bool) -> Self {
        self.search = enabled;
        self
    }

    pub fn initial_search_query(mut self, query: impl Into<String>) -> Self {
        self.initial_search_query = Some(query.into());
        self
    }

    pub fn search_blur_behavior(mut self, behavior: FileTreeSearchBlurBehavior) -> Self {
        self.search_blur_behavior = behavior;
        self
    }

    pub fn initial_selected_paths(
        mut self,
        paths: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.initial_selected_paths = paths.into_iter().map(Into::into).collect();
        self
    }

    pub fn on_selection_change(
        mut self,
        handler: impl Fn(&[String], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selection_change = Some(Rc::new(handler));
        self
    }

    pub fn on_search_change(
        mut self,
        handler: impl Fn(Option<&str>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_search_change = Some(Rc::new(handler));
        self
    }

    pub fn renaming(mut self, enabled: bool) -> Self {
        self.renaming = enabled.then(FileTreeRenamingConfig::default);
        self
    }

    pub fn renaming_config(mut self, config: FileTreeRenamingConfig) -> Self {
        self.renaming = Some(config);
        self
    }

    pub fn drag_and_drop(mut self, enabled: bool) -> Self {
        self.drag_and_drop = enabled.then(FileTreeDragAndDropConfig::default);
        self
    }

    pub fn drag_and_drop_config(mut self, config: FileTreeDragAndDropConfig) -> Self {
        self.drag_and_drop = Some(config);
        self
    }

    /// Pins expanded ancestor directories at the top of the scroll viewport.
    pub fn sticky_folders(mut self, enabled: bool) -> Self {
        self.sticky_folders = enabled;
        self
    }
    pub fn handle(mut self, handle: FileTreeHandle) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn density(mut self, density: FileTreeDensity) -> Self {
        self.density = density;
        self
    }

    pub fn row_height(mut self, row_height: Pixels) -> Self {
        self.explicit_row_height = Some(row_height);
        self
    }

    pub fn icons(mut self, icons: impl Into<FileTreeIcons>) -> Self {
        self.icons = Some(icons.into());
        self
    }

    pub fn git_status(mut self, entries: impl IntoIterator<Item = FileTreeGitStatusEntry>) -> Self {
        self.git_status = entries.into_iter().collect();
        self
    }

    pub fn composition(mut self, composition: FileTreeComposition) -> Self {
        self.composition = composition;
        self
    }

    pub fn header(
        mut self,
        render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.composition.header = Some(FileTreeHeaderComposition {
            render: Some(Rc::new(move |window, cx| Some(render(window, cx)))),
        });
        self
    }

    pub fn context_menu_config(mut self, config: FileTreeContextMenuComposition) -> Self {
        self.composition.context_menu = Some(config);
        self
    }

    pub fn row_decoration(
        mut self,
        render: impl Fn(
                crate::file_tree::FileTreeRowDecorationContext,
                &mut Window,
                &mut App,
            ) -> Option<AnyElement>
            + 'static,
    ) -> Self {
        self.row_decoration = Some(Rc::new(render));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FileTreeStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    pub fn row_style_with_state(
        mut self,
        style: impl Fn(crate::file_tree::FileTreeRowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.row_style_with_state = Some(Rc::new(style));
        self
    }

    pub fn filename_max_chars(mut self, maximum: usize) -> Self {
        self.filename_max_chars = maximum.max(5);
        self
    }
}

impl Styled for FileTree {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FileTree {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = self.id.clone();
        let search_enabled = self.search;
        let search_blur_behavior = self.search_blur_behavior;
        let density = self.density.resolve(self.explicit_row_height);
        let row_height = density.item_height;
        let controller_handle = self.handle.unwrap_or_default();
        let context = FileTreeContext::new(
            id.clone(),
            self.input,
            self.initial_expansion,
            self.search_mode,
            self.flatten_empty_directories,
            self.initial_search_query,
            self.initial_expanded_paths,
            self.initial_selected_paths,
            self.on_selection_change,
            self.on_search_change,
            self.renaming,
            self.drag_and_drop,
            self.sticky_folders,
            row_height,
            density,
            self.icons,
            self.git_status,
            self.composition,
            self.row_decoration,
            self.style_with_state,
            self.row_style_with_state,
            self.filename_max_chars,
            window,
            cx,
        );
        bind_file_tree_handle(&controller_handle, context.clone());
        let visible_count = context.visible_count(cx);
        let row_context = context.clone();
        let row_handle = controller_handle.clone();
        let focus_handle = context.focus_handle().clone();
        let scroll_handle = context.scroll_handle().clone();
        let search_focus_handle = context.search_focus_handle().clone();
        let sticky_overlay: Entity<FileTreeStickyOverlay> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("sticky-overlay")),
            cx,
            {
                let context = context.clone();
                let handle = controller_handle.clone();
                move |_, _| FileTreeStickyOverlay {
                    context,
                    handle,
                    row_height,
                }
            },
        );
        sticky_overlay.update(cx, |overlay, cx| {
            overlay.context = context.clone();
            overlay.handle = controller_handle.clone();
            overlay.row_height = row_height;
            cx.notify();
        });
        let weak_sticky_overlay = sticky_overlay.downgrade();
        context.bind_sticky_refresh(Rc::new(move |cx| {
            if let Some(sticky_overlay) = weak_sticky_overlay.upgrade() {
                sticky_overlay.update(cx, |_, cx| cx.notify());
            }
        }));
        let search_interaction: Entity<FileTreeSearchInteraction> = window.use_keyed_state(
            ElementId::NamedChild(
                Arc::new(id.clone()),
                SharedString::from("search-interaction"),
            ),
            cx,
            move |_, _| FileTreeSearchInteraction::new(search_blur_behavior),
        );
        let search_open = search_enabled && context.is_search_open(cx);
        let search_render_state = search_interaction.update(cx, |interaction, _| {
            interaction.sync(search_blur_behavior, search_open)
        });
        let search_input = search_open.then(|| {
            let search_icon = context.render_icon(
                FileTreeIconSlot::Search,
                None,
                None,
                None,
                false,
                window,
                cx,
            );
            div()
                .id("file-tree-search-wrapper")
                .debug_selector(|| "file-tree-search-wrapper".to_owned())
                .flex()
                .min_w_0()
                .items_center()
                .gap(px(2.0 * density.factor))
                .child(search_icon)
                .child(div().flex_1().min_w_0().child(FileTreeSearchInput::new(
                    ElementId::NamedChild(
                        Arc::new(id.clone()),
                        SharedString::from(format!(
                            "search-input-{}",
                            search_render_state.session_generation
                        )),
                    ),
                    search_render_state.auto_focus,
                    context.clone(),
                    search_focus_handle,
                    search_interaction.clone(),
                    SharedString::from(context.search_value(cx)),
                )))
        });
        let key_context = context.clone();
        let key_handle = controller_handle.clone();
        let move_next_context = context.clone();
        let move_previous_context = context.clone();
        let move_first_context = context.clone();
        let move_last_context = context.clone();
        let page_up_context = context.clone();
        let page_down_context = context.clone();
        let expand_context = context.clone();
        let collapse_context = context.clone();
        let extend_next_context = context.clone();
        let extend_previous_context = context.clone();
        let activate_context = context.clone();
        let toggle_selection_context = context.clone();
        let select_all_context = context.clone();
        let scroll_refresh_context = context.clone();
        let drag_identity = file_tree_handle_identity(&controller_handle);
        let drag_move_context = context.clone();
        let drag_move_handle = controller_handle.clone();
        let can_drop_handle = controller_handle.clone();
        let drop_handle = controller_handle.clone();
        let exit_handle = controller_handle.clone();
        let exit_context = context.clone();
        let cancel_handle = controller_handle.clone();
        let inside_cancel_handle = controller_handle.clone();
        let header = context.header(window, cx);
        let context_menu_surface = context.context_menu_surface(window, cx);
        let style_state = FileTreeStyleState {
            density,
            has_focus: focus_handle.is_focused(window),
            search_open,
            renaming: context.renaming_path(cx).is_some(),
            context_menu_open: context.context_menu_open(),
            drop_state: context.tree_drop_state(cx),
        };

        let base = self.base.flex().flex_col();
        let base = match context.style_with_state() {
            Some(style) => style(style_state, base),
            None => base,
        };

        base.id(id)
            .role(Role::Tree)
            .track_focus(&focus_handle.tab_stop(true).tab_index(0))
            .focusable()
            .key_context(FILE_TREE_KEY_CONTEXT)
            .on_scroll_wheel(move |_, window, cx| {
                scroll_refresh_context.schedule_sticky_refresh(window, cx);
            })
            .on_drag_move::<FileTreeInternalDragPayload>(move |event, window, cx| {
                if event.drag(cx).tree_identity != drag_identity {
                    return;
                }
                if event.bounds.contains(&event.event.position) {
                    if let Some(generation) = drag_move_context.begin_drag_target_capture(cx) {
                        let deferred_context = drag_move_context.clone();
                        window.defer(cx, move |window, cx| {
                            deferred_context
                                .clear_unresolved_drag_target_capture(generation, window, cx);
                        });
                    }
                    drag_move_context.drag_auto_scroll(
                        event.event.position.y,
                        event.bounds,
                        window,
                        cx,
                    );
                } else {
                    drag_move_context.stop_drag_auto_scroll(cx);
                    drag_move_handle.set_drop_target(None, window, cx);
                }
            })
            .can_drop(move |value, _window, cx| {
                value
                    .downcast_ref::<FileTreeInternalDragPayload>()
                    .is_some_and(|payload| {
                        payload.tree_identity == file_tree_handle_identity(&can_drop_handle)
                            && can_drop_handle.drop_target(cx).is_some()
                    })
            })
            .on_drop::<FileTreeInternalDragPayload>(move |payload, window, cx| {
                if payload.tree_identity != file_tree_handle_identity(&drop_handle) {
                    return;
                }
                let _ = drop_handle.complete_drag(window, cx);
                cx.stop_propagation();
            })
            .on_mouse_exit(move |_, window, cx| {
                exit_handle.set_drop_target(None, window, cx);
                exit_context.stop_drag_auto_scroll(cx);
            })
            .on_mouse_up_out(MouseButton::Left, move |_, window, cx| {
                cancel_handle.cancel_drag(window, cx);
            })
            .on_mouse_up(MouseButton::Left, move |_, window, cx| {
                inside_cancel_handle.cancel_drag(window, cx);
            })
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                if key_context.context_menu_open() {
                    if event.keystroke.key == "escape" {
                        key_context.close_context_menu(true, window, cx);
                    }
                    cx.stop_propagation();
                    return;
                }
                if event.keystroke.key == "f10" && event.keystroke.modifiers.shift {
                    if let Some(row) = key_context.focused_row(cx) {
                        let opened = key_context.open_context_menu(
                            row,
                            Bounds::new(point(px(0.0), px(0.0)), size(px(0.0), px(0.0))),
                            FileTreeContextMenuSource::Keyboard,
                            window,
                            cx,
                        );
                        if opened {
                            cx.stop_propagation();
                            return;
                        }
                    }
                }
                if event.keystroke.key == "escape" && !key_handle.dragged_paths(cx).is_empty() {
                    key_handle.cancel_drag(window, cx);
                    cx.stop_propagation();
                    return;
                }
                if key_handle.is_renaming(cx) {
                    let handled = match event.keystroke.key.as_str() {
                        "escape" => cancel_file_tree_renaming(&key_handle, window, cx),
                        "enter" => commit_file_tree_renaming(&key_handle, window, cx),
                        _ => false,
                    };
                    if handled {
                        cx.stop_propagation();
                    }
                    return;
                }
                if event.keystroke.key == "f2" {
                    if key_handle.start_renaming_focused(window, cx) {
                        cx.stop_propagation();
                    }
                    return;
                }
                if key_context.is_search_open(cx) {
                    let handled = match event.keystroke.key.as_str() {
                        "down" => {
                            key_context.focus_next_search_match(window, cx);
                            true
                        }
                        "up" => {
                            key_context.focus_previous_search_match(window, cx);
                            true
                        }
                        "enter" => {
                            key_context.select_focused_search_match(window, cx);
                            true
                        }
                        "escape" => {
                            key_context.close_search_and_focus_tree(window, cx);
                            true
                        }
                        _ => false,
                    };
                    if handled {
                        cx.stop_propagation();
                    }
                    return;
                }
                if search_enabled {
                    if let Some(text) = printable_search_text(event) {
                        key_context.set_search(Some(&text), window, cx);
                        cx.stop_propagation();
                    }
                }
            })
            .on_action(move |_: &FileTreeMoveNext, window, cx| {
                if !move_next_context.context_menu_open() {
                    move_next_context.focus_next(window, cx);
                }
            })
            .on_action(move |_: &FileTreeMovePrevious, window, cx| {
                if !move_previous_context.context_menu_open() {
                    move_previous_context.focus_previous(window, cx);
                }
            })
            .on_action(move |_: &FileTreeMoveFirst, window, cx| {
                if !move_first_context.context_menu_open() {
                    move_first_context.focus_first(window, cx);
                }
            })
            .on_action(move |_: &FileTreeMoveLast, window, cx| {
                if !move_last_context.context_menu_open() {
                    move_last_context.focus_last(window, cx);
                }
            })
            .on_action(move |_: &FileTreePageUp, window, cx| {
                if !page_up_context.context_menu_open() {
                    page_up_context.page_up(window, cx);
                }
            })
            .on_action(move |_: &FileTreePageDown, window, cx| {
                if !page_down_context.context_menu_open() {
                    page_down_context.page_down(window, cx);
                }
            })
            .on_action(move |_: &FileTreeExpandOrNext, window, cx| {
                if !expand_context.context_menu_open() {
                    expand_context.focus_right(window, cx);
                }
            })
            .on_action(move |_: &FileTreeCollapseOrParent, window, cx| {
                if !collapse_context.context_menu_open() {
                    collapse_context.focus_left(window, cx);
                }
            })
            .on_action(move |_: &FileTreeExtendNext, window, cx| {
                if !extend_next_context.context_menu_open() {
                    extend_next_context.extend_next(window, cx);
                }
            })
            .on_action(move |_: &FileTreeExtendPrevious, window, cx| {
                if !extend_previous_context.context_menu_open() {
                    extend_previous_context.extend_previous(window, cx);
                }
            })
            .on_action(move |_: &FileTreeActivateFocused, window, cx| {
                if !activate_context.context_menu_open() {
                    activate_context.activate_focused(window, cx);
                }
            })
            .on_action(move |_: &FileTreeToggleFocusedSelection, window, cx| {
                if !toggle_selection_context.context_menu_open() {
                    toggle_selection_context.toggle_focused_selection(window, cx);
                }
            })
            .on_action(move |_: &FileTreeSelectAllVisible, window, cx| {
                if !select_all_context.context_menu_open() {
                    select_all_context.select_all_visible(window, cx);
                }
            })
            .children(header)
            .children(search_input)
            .child(
                div()
                    .id("file-tree-scroll-viewport")
                    .relative()
                    .flex_1()
                    .min_h_0()
                    .overflow_hidden()
                    .child(
                        uniform_list(
                            "file-tree-rows",
                            visible_count,
                            move |range, _window, cx| {
                                let sticky_rows = row_context.sticky_rows(cx);
                                let sticky_occluded_range = occluded_row_range(
                                    row_context.sticky_scroll_top(cx),
                                    sticky_height(&sticky_rows, f32::from(row_height)),
                                    f32::from(row_height),
                                    visible_count,
                                );
                                row_context
                                    .visible_rows(range, cx)
                                    .into_iter()
                                    .map(|row| {
                                        let occluded = sticky_occluded_range.contains(&row.index);
                                        FileTreeRow::new(
                                            row_context.clone(),
                                            row_handle.clone(),
                                            row,
                                            row_height,
                                        )
                                        .occluded(occluded)
                                    })
                                    .collect()
                            },
                        )
                        .track_scroll(&scroll_handle)
                        .size_full(),
                    )
                    .child(sticky_overlay),
            )
            .children(context_menu_surface)
    }
}
fn printable_search_text(event: &KeyDownEvent) -> Option<String> {
    if event.keystroke.modifiers.control
        || event.keystroke.modifiers.alt
        || event.keystroke.modifiers.platform
        || event.keystroke.modifiers.function
    {
        return None;
    }

    event
        .keystroke
        .key_char
        .as_ref()
        .or(Some(&event.keystroke.key))
        .filter(|text| text.chars().count() == 1)
        .filter(|text| text.chars().all(|ch| !ch.is_control()))
        .cloned()
}
