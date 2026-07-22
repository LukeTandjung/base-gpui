use gpui::{App, ElementId, FocusHandle, Window};

use crate::dialog::{
    scoped_dialog_trigger_id, DialogChild, DialogContext, DialogPopupChild, DialogPortalChild,
    DialogTriggerMetadata, DialogViewportChild,
};

pub struct WiredDialogChildren<P: Clone + 'static> {
    pub triggers: Vec<DialogTriggerMetadata<P>>,
    pub title_ids: Vec<ElementId>,
    pub description_ids: Vec<ElementId>,
    pub popup_focus_handles: Vec<FocusHandle>,
    pub children: Vec<DialogChild<P>>,
}

pub trait DialogChildNode<P: Clone + 'static>: Sized {
    fn with_dialog_context(self, context: DialogContext<P>) -> Self;

    fn wire_dialog_child(
        self,
        _wiring: &mut DialogChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct DialogChildWiring<P: Clone + 'static> {
    root_id: ElementId,
    next_trigger_order: usize,
    triggers: Vec<DialogTriggerMetadata<P>>,
    title_ids: Vec<ElementId>,
    description_ids: Vec<ElementId>,
    popup_focus_handles: Vec<FocusHandle>,
}

impl<P: Clone + 'static> DialogChildWiring<P> {
    pub fn new(root_id: ElementId) -> Self {
        Self {
            root_id,
            next_trigger_order: 0,
            triggers: Vec::new(),
            title_ids: Vec::new(),
            description_ids: Vec::new(),
            popup_focus_handles: Vec::new(),
        }
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        scoped_dialog_trigger_id(&self.root_id, id)
    }

    pub fn wire_portal_children(
        &mut self,
        children: Vec<DialogPortalChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<DialogPortalChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_dialog_child(self, window, cx))
            .collect()
    }

    pub fn wire_viewport_children(
        &mut self,
        children: Vec<DialogViewportChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<DialogViewportChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_dialog_child(self, window, cx))
            .collect()
    }

    pub fn wire_popup_children(
        &mut self,
        children: Vec<DialogPopupChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<DialogPopupChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_dialog_child(self, window, cx))
            .collect()
    }

    pub fn register_trigger(
        &mut self,
        scoped_id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        disabled: bool,
        payload: Option<P>,
    ) -> usize {
        let order = self.next_trigger_order;
        self.next_trigger_order += 1;
        self.triggers.push(DialogTriggerMetadata::new(
            scoped_id,
            source_id,
            focus_handle,
            disabled,
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
        self.popup_focus_handles.push(focus_handle);
    }

    pub fn finish(self, children: Vec<DialogChild<P>>) -> WiredDialogChildren<P> {
        WiredDialogChildren {
            triggers: self.triggers,
            title_ids: self.title_ids,
            description_ids: self.description_ids,
            popup_focus_handles: self.popup_focus_handles,
            children,
        }
    }
}

pub fn wire_children<P: Clone + 'static>(
    children: Vec<DialogChild<P>>,
    context: DialogContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> WiredDialogChildren<P> {
    let mut wiring = DialogChildWiring::new(context.root_id());
    let children = children
        .into_iter()
        .map(|child| child.wire_dialog_child(&mut wiring, window, cx))
        .map(|child| child.with_dialog_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogChild<P> {
    fn with_dialog_context(self, context: DialogContext<P>) -> Self {
        match self {
            Self::Trigger(trigger) => Self::Trigger(Box::new(trigger.with_dialog_context(context))),
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_dialog_context(context))),
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_dialog_context(context)))
            }
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_dialog_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_dialog_context(context))),
            Self::Title(title) => Self::Title(Box::new(title.with_dialog_context(context))),
            Self::Description(description) => {
                Self::Description(Box::new(description.with_dialog_context(context)))
            }
            Self::Close(close) => Self::Close(Box::new(close.with_dialog_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_dialog_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.wire_dialog_child(wiring, window, cx)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_dialog_child(wiring, window, cx)))
            }
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.wire_dialog_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_dialog_child(wiring, window, cx)))
            }
            Self::Title(title) => {
                Self::Title(Box::new(title.wire_dialog_child(wiring, window, cx)))
            }
            Self::Description(description) => {
                Self::Description(Box::new(description.wire_dialog_child(wiring, window, cx)))
            }
            Self::Close(close) => {
                Self::Close(Box::new(close.wire_dialog_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogPortalChild<P> {
    fn with_dialog_context(self, context: DialogContext<P>) -> Self {
        match self {
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_dialog_context(context)))
            }
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_dialog_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_dialog_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_dialog_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.wire_dialog_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_dialog_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogViewportChild<P> {
    fn with_dialog_context(self, context: DialogContext<P>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_dialog_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_dialog_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_dialog_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogPopupChild<P> {
    fn with_dialog_context(self, context: DialogContext<P>) -> Self {
        match self {
            Self::Title(title) => Self::Title(Box::new(title.with_dialog_context(context))),
            Self::Description(description) => {
                Self::Description(Box::new(description.with_dialog_context(context)))
            }
            Self::Close(close) => Self::Close(Box::new(close.with_dialog_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_dialog_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Title(title) => {
                Self::Title(Box::new(title.wire_dialog_child(wiring, window, cx)))
            }
            Self::Description(description) => {
                Self::Description(Box::new(description.wire_dialog_child(wiring, window, cx)))
            }
            Self::Close(close) => {
                Self::Close(Box::new(close.wire_dialog_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}
