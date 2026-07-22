use std::sync::Arc;

use gpui::{App, ElementId, Entity, FocusHandle, SharedString, Window};

use crate::toolbar::{
    ToolbarChild, ToolbarContext, ToolbarGroupChild, ToolbarItemMetadata, ToolbarMove,
};

pub struct WiredToolbarChildren {
    pub items: Vec<ToolbarItemMetadata>,
    pub focus_handles: Vec<(usize, FocusHandle)>,
    pub focused_index: Option<usize>,
    pub children: Vec<ToolbarChild>,
}

struct ToolbarWiring {
    context: ToolbarContext,
    toolbar_disabled: bool,
    items: Vec<ToolbarItemMetadata>,
    focus_handles: Vec<(usize, FocusHandle)>,
    focused_index: Option<usize>,
}

impl ToolbarWiring {
    /// Registers one roving item with its effective (cascaded) metadata and
    /// returns the assigned flattened index and focus handle. This is the
    /// single registration channel every item variant flows through — the
    /// same one a future Toggle/ToggleGroup port will use, with a nested
    /// ToggleGroup contributing one slot per toggle and none for itself.
    fn register_item(
        &mut self,
        id: &ElementId,
        metadata: ToolbarItemMetadata,
        window: &mut Window,
        cx: &mut App,
    ) -> (usize, FocusHandle) {
        let index = self.items.len();
        let focus_handle = toolbar_item_focus_handle(id, window, cx);

        self.items.push(metadata);
        if focus_handle.is_focused(window) {
            self.focused_index = Some(index);
        }
        self.focus_handles.push((index, focus_handle.clone()));

        (index, focus_handle)
    }

    fn wire_group_child(
        &mut self,
        child: ToolbarGroupChild,
        cascade_disabled: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> ToolbarGroupChild {
        match child {
            ToolbarGroupChild::Button(button) => {
                let metadata = ToolbarItemMetadata::new(
                    button.own_disabled() || cascade_disabled,
                    button.own_focusable_when_disabled(),
                );
                let (index, focus_handle) =
                    self.register_item(button.item_id(), metadata, window, cx);

                ToolbarGroupChild::Button(button.with_toolbar(
                    self.context.clone(),
                    index,
                    focus_handle,
                    cascade_disabled,
                ))
            }
            ToolbarGroupChild::Link(link) => {
                // Links can never be disabled; the cascade skips them.
                let metadata = ToolbarItemMetadata::new(false, true);
                let (index, focus_handle) =
                    self.register_item(link.item_id(), metadata, window, cx);

                ToolbarGroupChild::Link(link.with_toolbar(
                    self.context.clone(),
                    index,
                    focus_handle,
                ))
            }
            ToolbarGroupChild::Input(input) => {
                let metadata = ToolbarItemMetadata::new(
                    input.own_disabled() || cascade_disabled,
                    input.own_focusable_when_disabled(),
                );
                let (index, focus_handle) =
                    self.register_item(input.item_id(), metadata, window, cx);

                ToolbarGroupChild::Input(input.with_toolbar(
                    self.context.clone(),
                    index,
                    focus_handle,
                    cascade_disabled,
                ))
            }
        }
    }
}

/// The only place item indices are assigned: walks the typed children in
/// source order, flattens group children into the toolbar's single item
/// order (groups and separators occupy no roving slot), and collects the
/// item metadata plus focus handles for `sync_children`.
pub fn wire_children(
    children: Vec<ToolbarChild>,
    context: ToolbarContext,
    toolbar_disabled: bool,
    window: &mut Window,
    cx: &mut App,
) -> WiredToolbarChildren {
    let mut wiring = ToolbarWiring {
        context: context.clone(),
        toolbar_disabled,
        items: Vec::new(),
        focus_handles: Vec::new(),
        focused_index: None,
    };
    let mut wired = Vec::new();

    for child in children {
        wired.push(wire_root_child(&mut wiring, child, window, cx));
    }

    WiredToolbarChildren {
        items: wiring.items,
        focus_handles: wiring.focus_handles,
        focused_index: wiring.focused_index,
        children: wired,
    }
}

fn wire_root_child(
    wiring: &mut ToolbarWiring,
    child: ToolbarChild,
    window: &mut Window,
    cx: &mut App,
) -> ToolbarChild {
    let toolbar_disabled = wiring.toolbar_disabled;

    match child {
        ToolbarChild::Button(button) => {
            match wiring.wire_group_child(
                ToolbarGroupChild::Button(button),
                toolbar_disabled,
                window,
                cx,
            ) {
                ToolbarGroupChild::Button(button) => ToolbarChild::Button(button),
                _ => unreachable!("button wiring should return a button"),
            }
        }
        ToolbarChild::Link(link) => {
            match wiring.wire_group_child(
                ToolbarGroupChild::Link(link),
                toolbar_disabled,
                window,
                cx,
            ) {
                ToolbarGroupChild::Link(link) => ToolbarChild::Link(link),
                _ => unreachable!("link wiring should return a link"),
            }
        }
        ToolbarChild::Input(input) => {
            match wiring.wire_group_child(
                ToolbarGroupChild::Input(input),
                toolbar_disabled,
                window,
                cx,
            ) {
                ToolbarGroupChild::Input(input) => ToolbarChild::Input(input),
                _ => unreachable!("input wiring should return an input"),
            }
        }
        ToolbarChild::Group(group) => {
            let merged_disabled = toolbar_disabled || group.own_disabled();
            let (group, children) = group.split_children();
            let children = children
                .into_iter()
                .map(|child| wiring.wire_group_child(child, merged_disabled, window, cx))
                .collect();

            ToolbarChild::Group(group.with_toolbar(
                wiring.context.clone(),
                merged_disabled,
                children,
            ))
        }
        ToolbarChild::Separator(separator) => {
            ToolbarChild::Separator(separator.with_toolbar(wiring.context.clone()))
        }
    }
}

/// The stable keyed focus handle for a toolbar item, shared across renders.
pub fn toolbar_item_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("toolbar-focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

/// Moves the roving highlight and real focus in one step; shared by root
/// arrow handlers and the input's caret-edge handlers.
pub fn move_focus(
    context: &ToolbarContext,
    direction: ToolbarMove,
    window: &mut Window,
    cx: &mut App,
) {
    let loop_focus = context.read(cx, |_runtime, props| props.loop_focus());
    let focus_handle = context.update(cx, |runtime| {
        runtime.move_highlight(direction, loop_focus);
        runtime.highlighted_focus_handle()
    });

    if let Some(focus_handle) = focus_handle {
        focus_handle.focus(window, cx);
    }
}
