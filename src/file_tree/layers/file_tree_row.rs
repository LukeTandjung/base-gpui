use gpui::{
    div, prelude::FluentBuilder as _, px, size, AnyElement, App, AppContext as _, Bounds, Context,
    FontWeight, InteractiveElement as _, IntoElement, MouseButton, ParentElement as _, Pixels,
    Render, RenderOnce, Role, StatefulInteractiveElement as _, Styled as _, Window,
};

use crate::{
    file_tree::{
        context::FileTreeContext,
        controller::{
            cancel_file_tree_renaming, commit_file_tree_renaming, file_tree_handle_identity,
            prepared_file_tree_dragged_paths, set_file_tree_renaming_value,
        },
        drag_and_drop::FileTreeDropTarget,
        middle_truncate_file_name, FileTreeCompositionItem, FileTreeContextMenuButtonVisibility,
        FileTreeContextMenuSource, FileTreeContextMenuTriggerMode, FileTreeGitStatus,
        FileTreeHandle, FileTreeIconSlot, FileTreeItemKind, FileTreeRowDecorationContext,
        FileTreeRowStyleState, FileTreeVisibleRow,
    },
    primitives::input::Input,
};

#[derive(Clone)]
pub struct FileTreeInternalDragPayload {
    pub tree_identity: usize,
    pub dragged_paths: Vec<String>,
    pub primary_path: String,
    pub handle: FileTreeHandle,
}

struct FileTreeDragPreview {
    label: String,
    count: usize,
}

impl Render for FileTreeDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_2()
            .py_1()
            .rounded_sm()
            .bg(gpui::rgba(0x202020ee))
            .text_color(gpui::white())
            .child(if self.count == 1 {
                self.label.clone()
            } else {
                format!("{} (+{})", self.label, self.count - 1)
            })
    }
}

fn file_tree_row_drop_target(row: &FileTreeVisibleRow) -> FileTreeDropTarget {
    if row.kind == FileTreeItemKind::Directory {
        return FileTreeDropTarget::directory(row.path.clone(), row.path.clone());
    }
    let trimmed = row.path.trim_end_matches('/');
    let Some(slash) = trimmed.rfind('/') else {
        return FileTreeDropTarget::root(row.path.clone());
    };
    FileTreeDropTarget::directory(format!("{}/", &trimmed[..slash]), row.path.clone())
}

fn flattened_segments_element(
    handle: &FileTreeHandle,
    row: &FileTreeVisibleRow,
    id_prefix: &'static str,
    cx: &App,
) -> gpui::Div {
    let current_target = handle.drop_target(cx);
    let mut label = div().flex().items_center().min_w_0();
    for (index, segment) in row.flattened_segments.iter().enumerate() {
        let target =
            FileTreeDropTarget::flattened_directory(segment.path.clone(), row.path.clone());
        let drag_move_handle = handle.clone();
        let drag_move_target = target.clone();
        let can_drop_handle = handle.clone();
        let can_drop_target = target.clone();
        let drop_handle = handle.clone();
        let drop_target = target.clone();
        let targeted = current_target.as_ref() == Some(&target);
        let segment_element = div()
            .id(format!(
                "file-tree-{id_prefix}-flattened-segment-{}-{index}",
                row.index
            ))
            .when(targeted, |this| this.bg(gpui::rgba(0x3b82f633)))
            .child(segment.name.clone())
            .on_drag_move::<FileTreeInternalDragPayload>(move |event, window, cx| {
                if event.drag(cx).tree_identity != file_tree_handle_identity(&drag_move_handle)
                    || !event.bounds.contains(&event.event.position)
                {
                    return;
                }
                drag_move_handle.set_drop_target(Some(drag_move_target.clone()), window, cx);
            })
            .can_drop(move |value, _window, cx| {
                value
                    .downcast_ref::<FileTreeInternalDragPayload>()
                    .is_some_and(|payload| {
                        payload.tree_identity == file_tree_handle_identity(&can_drop_handle)
                            && can_drop_handle.drop_target(cx).as_ref() == Some(&can_drop_target)
                    })
            })
            .on_drop::<FileTreeInternalDragPayload>(move |payload, window, cx| {
                if payload.tree_identity != file_tree_handle_identity(&drop_handle) {
                    return;
                }
                drop_handle.set_drop_target(Some(drop_target.clone()), window, cx);
                let _ = drop_handle.complete_drag(window, cx);
                cx.stop_propagation();
            });
        label = label.child(segment_element);
        if index + 1 < row.flattened_segments.len() {
            label = label.child(" / ");
        }
    }
    label
}
#[derive(Clone, Copy)]
enum FileTreeRowMode {
    Flow,
    Sticky { top: f32 },
}

#[derive(IntoElement)]
pub struct FileTreeRow {
    context: FileTreeContext,
    handle: FileTreeHandle,
    row: FileTreeVisibleRow,
    row_height: Pixels,
    mode: FileTreeRowMode,
    occluded: bool,
}

impl FileTreeRow {
    pub fn new(
        context: FileTreeContext,
        handle: FileTreeHandle,
        row: FileTreeVisibleRow,
        row_height: Pixels,
    ) -> Self {
        Self {
            context,
            handle,
            row,
            row_height,
            mode: FileTreeRowMode::Flow,
            occluded: false,
        }
    }

    pub fn occluded(mut self, occluded: bool) -> Self {
        self.occluded = occluded;
        self
    }
    pub fn sticky(
        context: FileTreeContext,
        handle: FileTreeHandle,
        row: FileTreeVisibleRow,
        row_height: Pixels,
        top: f32,
    ) -> Self {
        Self {
            context,
            handle,
            row,
            row_height,
            mode: FileTreeRowMode::Sticky { top },
            occluded: false,
        }
    }
}

fn icon_element(
    context: &FileTreeContext,
    slot: FileTreeIconSlot,
    row: &FileTreeVisibleRow,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    context.render_icon(
        slot,
        (slot == FileTreeIconSlot::File).then_some(row.path.as_str()),
        Some(&row.name),
        Some(row.kind),
        row.is_expanded,
        window,
        cx,
    )
}

fn git_label(status: FileTreeGitStatus) -> Option<&'static str> {
    match status {
        FileTreeGitStatus::Added => Some("A"),
        FileTreeGitStatus::Deleted => Some("D"),
        FileTreeGitStatus::Ignored => None,
        FileTreeGitStatus::Modified => Some("M"),
        FileTreeGitStatus::Renamed => Some("R"),
        FileTreeGitStatus::Untracked => Some("U"),
    }
}

impl RenderOnce for FileTreeRow {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_directory = self.row.kind == FileTreeItemKind::Directory;
        let is_sticky = matches!(self.mode, FileTreeRowMode::Sticky { .. });
        let occluded = self.occluded;
        let path = self.row.path.clone();
        let selected = self.row.is_selected;
        let focused = self.row.is_focused;
        let tree_focused = self.context.focus_handle().is_focused(window);
        let active = focused && tree_focused;
        let expanded = self.row.is_expanded;
        let level = self.row.level + 1;
        let position_in_set = self.row.position_in_set;
        let set_size = self.row.set_size;
        let visible_index = self.row.index;
        let hovered = self.context.hovered_path().as_deref() == Some(path.as_str());
        let pressed = self.context.pressed_path().as_deref() == Some(path.as_str());
        let context_menu_open = self.context.context_menu_open_for_path(&path);
        let drop_state = self.context.drop_state_for_path(&path, cx);
        let git_status = self.context.git_status_for_path(&path);
        let contains_git_change = is_directory && self.context.directory_has_git_changes(&path);
        let renaming_generation = self.context.renaming_generation(cx);
        let renaming =
            !is_sticky && self.context.renaming_path(cx).as_deref() == Some(path.as_str());
        let rename_input = renaming.then(|| {
            let value = self.context.renaming_value(cx);
            let change_handle = self.handle.clone();
            let enter_handle = self.handle.clone();
            let escape_handle = self.handle.clone();
            let blur_handle = self.handle.clone();
            let rename_icon = icon_element(
                &self.context,
                FileTreeIconSlot::Rename,
                &self.row,
                window,
                cx,
            );
            div()
                .id(("file-tree-rename-wrapper", visible_index))
                .flex()
                .flex_1()
                .min_w_0()
                .items_center()
                .gap(px(2.0 * self.context.density().factor))
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Right, |_, _, cx| cx.stop_propagation())
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(rename_icon)
                .child(
                    div().flex_1().min_w_0().child(
                        Input::new()
                            .id(("file-tree-rename-input", renaming_generation))
                            .value(value)
                            .auto_focus(true)
                            .select_all_on_focus(true)
                            .tab_stop(false)
                            .h_full()
                            .w_full()
                            .on_value_change_with_context(move |value, window, cx| {
                                set_file_tree_renaming_value(
                                    &change_handle,
                                    value.as_ref(),
                                    window,
                                    cx,
                                );
                            })
                            .on_enter_with_context(move |_, window, cx| {
                                commit_file_tree_renaming(&enter_handle, window, cx);
                            })
                            .on_escape_with_context(move |window, cx| {
                                cancel_file_tree_renaming(&escape_handle, window, cx);
                            })
                            .on_blur_with_context(move |window, cx| {
                                commit_file_tree_renaming(&blur_handle, window, cx);
                            }),
                    ),
                )
        });
        let row_drop_target = file_tree_row_drop_target(&self.row);
        let dragging = self
            .handle
            .dragged_paths(cx)
            .iter()
            .any(|dragged| dragged == &self.row.path);
        let drop_targeted = self.handle.drop_target(cx).is_some_and(|target| {
            target.hovered_path.as_deref() == Some(self.row.path.as_str())
                && target.flattened_segment_path.is_none()
        });
        let is_flattened = self.row.is_flattened;
        let flattened_label = (!renaming && is_flattened).then(|| {
            flattened_segments_element(
                &self.handle,
                &self.row,
                if is_sticky { "sticky" } else { "row" },
                cx,
            )
        });
        let file_name =
            middle_truncate_file_name(&self.row.name, self.context.filename_max_chars());
        let drag_payload = (!renaming)
            .then(|| prepared_file_tree_dragged_paths(&self.handle, &self.row.path, cx))
            .flatten()
            .map(|dragged_paths| FileTreeInternalDragPayload {
                tree_identity: file_tree_handle_identity(&self.handle),
                primary_path: self.row.path.clone(),
                handle: self.handle.clone(),
                dragged_paths,
            });
        let drag_move_handle = self.handle.clone();
        let drag_move_target = row_drop_target.clone();
        let can_drop_handle = self.handle.clone();
        let can_drop_target = row_drop_target.clone();
        let drop_handle = self.handle.clone();
        let drop_target = row_drop_target.clone();

        let disclosure_slot = if expanded {
            FileTreeIconSlot::DisclosureExpanded
        } else {
            FileTreeIconSlot::DisclosureCollapsed
        };
        let disclosure = is_directory
            .then(|| icon_element(&self.context, disclosure_slot, &self.row, window, cx));
        let item_icon = icon_element(
            &self.context,
            match (is_directory, expanded) {
                (true, true) => FileTreeIconSlot::OpenFolder,
                (true, false) => FileTreeIconSlot::Folder,
                (false, _) => FileTreeIconSlot::File,
            },
            &self.row,
            window,
            cx,
        );
        let decoration = self.context.row_decoration().and_then(|render| {
            render(
                FileTreeRowDecorationContext {
                    item: FileTreeCompositionItem::from(&self.row),
                    row: self.row.clone(),
                    drop_state: drop_state.clone(),
                },
                window,
                cx,
            )
        });
        let decoration_lane = self.context.row_decoration().is_some().then(|| {
            div()
                .id(("file-tree-row-decoration", visible_index))
                .flex()
                .items_center()
                .children(decoration)
        });
        let git_lane = self.context.has_git_status().then(|| {
            let mut lane = div()
                .id(("file-tree-row-git", visible_index))
                .w(px(18.0))
                .flex()
                .items_center()
                .justify_center();
            if git_status == Some(FileTreeGitStatus::Ignored) {
                lane = lane.child(icon_element(
                    &self.context,
                    FileTreeIconSlot::Lock,
                    &self.row,
                    window,
                    cx,
                ));
            } else if let Some(label) = git_status.and_then(git_label) {
                lane = lane.child(label);
            } else if git_status.is_none() && contains_git_change {
                lane = lane.child(icon_element(
                    &self.context,
                    FileTreeIconSlot::GitChangeDot,
                    &self.row,
                    window,
                    cx,
                ));
            }
            lane
        });
        let trigger_mode = self.context.context_menu_trigger_mode();
        let button_enabled = matches!(
            trigger_mode,
            Some(FileTreeContextMenuTriggerMode::Both | FileTreeContextMenuTriggerMode::Button)
        );
        let right_click_enabled = matches!(
            trigger_mode,
            Some(FileTreeContextMenuTriggerMode::Both | FileTreeContextMenuTriggerMode::RightClick)
        );
        let button_visibility = self.context.context_menu_button_visibility();
        let action_lane = button_enabled.then(|| {
            let menu_context = self.context.clone();
            let menu_row = self.row.clone();
            let show_button = button_visibility
                == Some(FileTreeContextMenuButtonVisibility::Always)
                || hovered
                || active
                || context_menu_open;
            div()
                .id(("file-tree-row-action", visible_index))
                .w(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .when(!show_button, |this| this.invisible())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .on_click(move |event, window, cx| {
                    menu_context.open_context_menu(
                        menu_row.clone(),
                        Bounds::new(event.position(), size(px(0.0), px(0.0))),
                        FileTreeContextMenuSource::Button,
                        window,
                        cx,
                    );
                    cx.stop_propagation();
                })
                .child(icon_element(
                    &self.context,
                    FileTreeIconSlot::ContextMenu,
                    &self.row,
                    window,
                    cx,
                ))
        });

        let row_style_state = FileTreeRowStyleState {
            row: self.row.clone(),
            active,
            hovered,
            pressed,
            sticky: is_sticky,
            occluded,
            renaming,
            context_menu_open,
            git_status,
            contains_git_change,
            drop_state,
        };
        let base = div()
            .h(self.row_height)
            .flex()
            .items_center()
            .gap(px(2.0 * self.context.density().factor))
            .pl(px(self.row.depth.saturating_sub(1) as f32
                * 16.0
                * self.context.density().factor))
            .cursor_pointer()
            .when(selected, |this| this.font_weight(FontWeight::SEMIBOLD))
            .when(active, |this| this.underline())
            .when(dragging, |this| this.opacity(0.55))
            .when(occluded, |this| this.opacity(0.0))
            .when(drop_targeted, |this| this.bg(gpui::rgba(0x3b82f633)));
        let base = match self.context.row_style_with_state() {
            Some(style) => style(row_style_state, base),
            None => base,
        };

        let click_context = self.context.clone();
        let click_path = path.clone();
        let context_click_context = self.context.clone();
        let context_click_row = self.row.clone();
        let hover_context = self.context.clone();
        let hover_path = path.clone();
        let exit_context = self.context.clone();
        let press_context = self.context.clone();
        let press_path = path.clone();
        let release_context = self.context.clone();

        base.id(if is_sticky {
            ("file-tree-sticky-row", self.row.index)
        } else {
            ("file-tree-row", self.row.index)
        })
        .debug_selector(move || {
            if is_sticky {
                format!("file-tree-sticky-row-{visible_index}")
            } else {
                format!("file-tree-row-{visible_index}")
            }
        })
        // Pinned GPUI has no aria-hidden or aria-haspopup builder. Sticky rows
        // omit Role::TreeItem to avoid duplicate AccessKit items. Context-menu
        // availability is exposed by the visible GPUI action button instead.
        .when(!is_sticky, |this| {
            this.role(Role::TreeItem)
                .aria_label(format!(
                    "{}, {}",
                    self.row.name,
                    if is_directory { "folder" } else { "file" }
                ))
                .aria_level(level)
                .aria_position_in_set(position_in_set)
                .aria_size_of_set(set_size)
                .aria_selected(selected)
                .when(is_directory, |this| this.aria_expanded(expanded))
                .when(focused, |this| this.aria_active_descendant())
        })
        .when_some(
            match self.mode {
                FileTreeRowMode::Flow => None,
                FileTreeRowMode::Sticky { top } => Some(top),
            },
            |this, top| this.absolute().top(px(top)).left_0().right_0(),
        )
        .when_some(drag_payload, |this, payload| {
            this.on_drag(payload, move |payload, _, window, cx| {
                let _ = payload.handle.start_drag(&payload.primary_path, window, cx);
                let label = payload
                    .primary_path
                    .trim_end_matches('/')
                    .rsplit('/')
                    .next()
                    .unwrap_or(payload.primary_path.as_str())
                    .to_owned();
                let count = payload.dragged_paths.len();
                cx.new(|_| FileTreeDragPreview { label, count })
            })
        })
        .on_drag_move::<FileTreeInternalDragPayload>(move |event, window, cx| {
            if event.drag(cx).tree_identity != file_tree_handle_identity(&drag_move_handle)
                || !event.bounds.contains(&event.event.position)
            {
                return;
            }
            drag_move_handle.set_drop_target(Some(drag_move_target.clone()), window, cx);
        })
        .can_drop(move |value, _window, cx| {
            value
                .downcast_ref::<FileTreeInternalDragPayload>()
                .is_some_and(|payload| {
                    payload.tree_identity == file_tree_handle_identity(&can_drop_handle)
                        && can_drop_handle.drop_target(cx).as_ref() == Some(&can_drop_target)
                })
        })
        .on_drop::<FileTreeInternalDragPayload>(move |payload, window, cx| {
            if payload.tree_identity != file_tree_handle_identity(&drop_handle) {
                return;
            }
            drop_handle.set_drop_target(Some(drop_target.clone()), window, cx);
            let _ = drop_handle.complete_drag(window, cx);
            cx.stop_propagation();
        })
        .on_mouse_move(move |_, window, cx| {
            hover_context.set_hovered_path(Some(hover_path.clone()), window, cx);
        })
        .on_mouse_exit(move |_, window, cx| {
            exit_context.set_hovered_path(None, window, cx);
            exit_context.set_pressed_path(None, window, cx);
        })
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            press_context.set_pressed_path(Some(press_path.clone()), window, cx);
        })
        .on_mouse_up(MouseButton::Left, move |_, window, cx| {
            release_context.set_pressed_path(None, window, cx);
        })
        .on_mouse_down(MouseButton::Right, move |event, window, cx| {
            if right_click_enabled {
                context_click_context.open_context_menu(
                    context_click_row.clone(),
                    Bounds::new(event.position, size(px(0.0), px(0.0))),
                    FileTreeContextMenuSource::RightClick,
                    window,
                    cx,
                );
                cx.stop_propagation();
            }
        })
        .on_click(move |event, window, cx| {
            if event.is_right_click() {
                return;
            }
            if is_sticky {
                click_context.sticky_row_click(
                    &click_path,
                    is_directory,
                    event.modifiers(),
                    window,
                    cx,
                );
            } else {
                click_context.row_click(&click_path, is_directory, event.modifiers(), window, cx);
            }
        })
        .children(disclosure)
        .child(item_icon)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .when_some(rename_input, |this, input| this.child(input))
                .when_some(flattened_label, |this, label| this.child(label))
                .when(!renaming && !is_flattened, |this| this.child(file_name)),
        )
        .children(decoration_lane)
        .children(git_lane)
        .children(action_lane)
    }
}
