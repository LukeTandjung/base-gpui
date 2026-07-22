use std::time::Duration;

use gpui::{App, ElementId, FocusHandle, SharedString, Window};

use crate::popover::{
    scoped_trigger_id, PopoverChild, PopoverContext, PopoverPopupChild, PopoverPortalChild,
    PopoverPositionerChild, PopoverTriggerMetadata,
};

pub struct WiredPopoverChildren<P: Clone + 'static> {
    pub triggers: Vec<PopoverTriggerMetadata<P>>,
    pub title_ids: Vec<ElementId>,
    pub description_ids: Vec<ElementId>,
    pub focus_handles: Vec<FocusHandle>,
    pub popup_focus_handles: Vec<FocusHandle>,
    pub children: Vec<PopoverChild<P>>,
}

pub trait PopoverChildNode<P: Clone + 'static>: Sized {
    fn with_popover_context(self, context: PopoverContext<P>) -> Self;

    fn wire_popover_child(
        self,
        _wiring: &mut PopoverChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct PopoverChildWiring<P: Clone + 'static> {
    root_id: ElementId,
    next_trigger_order: usize,
    triggers: Vec<PopoverTriggerMetadata<P>>,
    title_ids: Vec<ElementId>,
    description_ids: Vec<ElementId>,
    focus_handles: Vec<FocusHandle>,
    popup_focus_handles: Vec<FocusHandle>,
}

impl<P: Clone + 'static> PopoverChildWiring<P> {
    pub fn new(root_id: ElementId) -> Self {
        Self {
            root_id,
            next_trigger_order: 0,
            triggers: Vec::new(),
            title_ids: Vec::new(),
            description_ids: Vec::new(),
            focus_handles: Vec::new(),
            popup_focus_handles: Vec::new(),
        }
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        scoped_trigger_id(&self.root_id, id)
    }

    pub fn wire_portal_children(
        &mut self,
        children: Vec<PopoverPortalChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<PopoverPortalChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_popover_child(self, window, cx))
            .collect()
    }

    pub fn wire_positioner_children(
        &mut self,
        children: Vec<PopoverPositionerChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<PopoverPositionerChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_popover_child(self, window, cx))
            .collect()
    }

    pub fn wire_popup_children(
        &mut self,
        children: Vec<PopoverPopupChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<PopoverPopupChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_popover_child(self, window, cx))
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn register_trigger(
        &mut self,
        scoped_id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        disabled: bool,
        open_on_hover: bool,
        delay: Duration,
        close_delay: Duration,
        payload: Option<P>,
    ) -> usize {
        let order = self.next_trigger_order;
        self.next_trigger_order += 1;
        self.focus_handles.push(focus_handle.clone());
        self.triggers.push(PopoverTriggerMetadata::new(
            scoped_id,
            source_id,
            focus_handle,
            disabled,
            open_on_hover,
            delay,
            close_delay,
            payload,
            order,
            false,
        ));
        order
    }

    pub fn register_title(&mut self, id: ElementId) {
        self.title_ids.push(id);
    }

    pub fn register_description(&mut self, id: ElementId) {
        self.description_ids.push(id);
    }

    pub fn register_popup_focus_handle(&mut self, focus_handle: FocusHandle) {
        self.focus_handles.push(focus_handle.clone());
        self.popup_focus_handles.push(focus_handle);
    }

    fn finish(self, children: Vec<PopoverChild<P>>) -> WiredPopoverChildren<P> {
        WiredPopoverChildren {
            triggers: self.triggers,
            title_ids: self.title_ids,
            description_ids: self.description_ids,
            focus_handles: self.focus_handles,
            popup_focus_handles: self.popup_focus_handles,
            children,
        }
    }
}

pub fn wire_children<P: Clone + 'static>(
    children: Vec<PopoverChild<P>>,
    context: PopoverContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> WiredPopoverChildren<P> {
    let mut wiring = PopoverChildWiring::new(context.root_id());
    let children = children
        .into_iter()
        .map(|child| child.wire_popover_child(&mut wiring, window, cx))
        .map(|child| child.with_popover_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverChild<P> {
    fn with_popover_context(self, context: PopoverContext<P>) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.with_popover_context(context)))
            }
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_popover_context(context))),
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_popover_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_popover_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_popover_context(context))),
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_popover_context(context))),
            Self::Title(title) => Self::Title(Box::new(title.with_popover_context(context))),
            Self::Description(description) => {
                Self::Description(Box::new(description.with_popover_context(context)))
            }
            Self::Close(close) => Self::Close(Box::new(close.with_popover_context(context))),
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_popover_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_popover_child(
        self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.wire_popover_child(wiring, window, cx)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_popover_child(wiring, window, cx)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_popover_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_popover_child(wiring, window, cx)))
            }
            Self::Title(title) => {
                Self::Title(Box::new(title.wire_popover_child(wiring, window, cx)))
            }
            Self::Description(description) => {
                Self::Description(Box::new(description.wire_popover_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPortalChild<P> {
    fn with_popover_context(self, context: PopoverContext<P>) -> Self {
        match self {
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_popover_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_popover_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_popover_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_popover_child(
        self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_popover_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_popover_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPositionerChild<P> {
    fn with_popover_context(self, context: PopoverContext<P>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_popover_context(context))),
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_popover_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_popover_child(
        self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_popover_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPopupChild<P> {
    fn with_popover_context(self, context: PopoverContext<P>) -> Self {
        match self {
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_popover_context(context))),
            Self::Title(title) => Self::Title(Box::new(title.with_popover_context(context))),
            Self::Description(description) => {
                Self::Description(Box::new(description.with_popover_context(context)))
            }
            Self::Close(close) => Self::Close(Box::new(close.with_popover_context(context))),
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_popover_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_popover_child(
        self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Title(title) => {
                Self::Title(Box::new(title.wire_popover_child(wiring, window, cx)))
            }
            Self::Description(description) => {
                Self::Description(Box::new(description.wire_popover_child(wiring, window, cx)))
            }
            Self::Close(close) => {
                Self::Close(Box::new(close.wire_popover_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

pub fn scoped_part_id(root_id: &ElementId, part: &str) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(part)))
}
