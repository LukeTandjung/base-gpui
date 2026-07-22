use std::time::Duration;

use gpui::{App, ElementId, FocusHandle, SharedString, Window};

use crate::preview_card::{
    scoped_trigger_id, PreviewCardChild, PreviewCardContext, PreviewCardPopupChild,
    PreviewCardPortalChild, PreviewCardPositionerChild, PreviewCardTriggerMetadata,
};

pub struct WiredPreviewCardChildren<P: Clone + 'static> {
    pub triggers: Vec<PreviewCardTriggerMetadata<P>>,
    pub trigger_focus_handles: Vec<(ElementId, FocusHandle)>,
    pub children: Vec<PreviewCardChild<P>>,
}

pub trait PreviewCardChildNode<P: Clone + 'static>: Sized {
    fn with_preview_card_context(self, context: PreviewCardContext<P>) -> Self;

    fn wire_preview_card_child(
        self,
        _wiring: &mut PreviewCardChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct PreviewCardChildWiring<P: Clone + 'static> {
    root_id: ElementId,
    next_trigger_order: usize,
    default_delay: Duration,
    default_close_delay: Duration,
    triggers: Vec<PreviewCardTriggerMetadata<P>>,
    trigger_focus_handles: Vec<(ElementId, FocusHandle)>,
}

impl<P: Clone + 'static> PreviewCardChildWiring<P> {
    pub fn new(root_id: ElementId, default_delay: Duration, default_close_delay: Duration) -> Self {
        Self {
            root_id,
            next_trigger_order: 0,
            default_delay,
            default_close_delay,
            triggers: Vec::new(),
            trigger_focus_handles: Vec::new(),
        }
    }

    pub fn effective_delay(&self, delay: Option<Duration>) -> Duration {
        delay.unwrap_or(self.default_delay)
    }

    pub fn effective_close_delay(&self, close_delay: Option<Duration>) -> Duration {
        close_delay.unwrap_or(self.default_close_delay)
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        scoped_trigger_id(&self.root_id, id)
    }

    pub fn wire_portal_children(
        &mut self,
        children: Vec<PreviewCardPortalChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<PreviewCardPortalChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_preview_card_child(self, window, cx))
            .collect()
    }

    pub fn wire_positioner_children(
        &mut self,
        children: Vec<PreviewCardPositionerChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<PreviewCardPositionerChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_preview_card_child(self, window, cx))
            .collect()
    }

    pub fn wire_popup_children(
        &mut self,
        children: Vec<PreviewCardPopupChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<PreviewCardPopupChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_preview_card_child(self, window, cx))
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn register_trigger(
        &mut self,
        scoped_id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        delay: Duration,
        close_delay: Duration,
        payload: Option<P>,
    ) -> usize {
        let order = self.next_trigger_order;
        self.next_trigger_order += 1;
        self.trigger_focus_handles
            .push((scoped_id.clone(), focus_handle.clone()));
        self.triggers.push(PreviewCardTriggerMetadata::new(
            scoped_id,
            source_id,
            focus_handle,
            delay,
            close_delay,
            payload,
            order,
            false,
        ));
        order
    }

    fn finish(self, children: Vec<PreviewCardChild<P>>) -> WiredPreviewCardChildren<P> {
        WiredPreviewCardChildren {
            triggers: self.triggers,
            trigger_focus_handles: self.trigger_focus_handles,
            children,
        }
    }
}

pub fn wire_children<P: Clone + 'static>(
    children: Vec<PreviewCardChild<P>>,
    context: PreviewCardContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> WiredPreviewCardChildren<P> {
    let (default_delay, default_close_delay) =
        context.read(cx, |_runtime, props| (props.delay(), props.close_delay()));
    let mut wiring =
        PreviewCardChildWiring::new(context.root_id(), default_delay, default_close_delay);
    let children = children
        .into_iter()
        .map(|child| child.wire_preview_card_child(&mut wiring, window, cx))
        .map(|child| child.with_preview_card_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardChild<P> {
    fn with_preview_card_context(self, context: PreviewCardContext<P>) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.with_preview_card_context(context)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.with_preview_card_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_preview_card_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_preview_card_context(context))),
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_preview_card_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_preview_card_child(
        self,
        wiring: &mut PreviewCardChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => Self::Trigger(Box::new(
                trigger.wire_preview_card_child(wiring, window, cx),
            )),
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_preview_card_child(wiring, window, cx)))
            }
            Self::Positioner(positioner) => Self::Positioner(Box::new(
                positioner.wire_preview_card_child(wiring, window, cx),
            )),
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_preview_card_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardPortalChild<P> {
    fn with_preview_card_context(self, context: PreviewCardContext<P>) -> Self {
        match self {
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_preview_card_context(context)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_preview_card_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_preview_card_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_preview_card_child(
        self,
        wiring: &mut PreviewCardChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Positioner(positioner) => Self::Positioner(Box::new(
                positioner.wire_preview_card_child(wiring, window, cx),
            )),
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_preview_card_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardPositionerChild<P> {
    fn with_preview_card_context(self, context: PreviewCardContext<P>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_preview_card_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_preview_card_child(
        self,
        wiring: &mut PreviewCardChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_preview_card_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardPopupChild<P> {
    fn with_preview_card_context(self, context: PreviewCardContext<P>) -> Self {
        match self {
            Self::Arrow(arrow) => Self::Arrow(Box::new(arrow.with_preview_card_context(context))),
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_preview_card_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub fn scoped_part_id(root_id: &ElementId, part: &str) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(part)))
}
