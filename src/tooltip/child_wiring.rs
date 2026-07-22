use std::time::Duration;

use gpui::{App, ElementId, FocusHandle, SharedString, Window};

use crate::tooltip::{
    scoped_trigger_id, TooltipChild, TooltipContext, TooltipPopupChild, TooltipPortalChild,
    TooltipPositionerChild, TooltipProviderConfig, TooltipTriggerMetadata,
};

pub struct WiredTooltipChildren<P: Clone + 'static> {
    pub triggers: Vec<TooltipTriggerMetadata<P>>,
    pub trigger_focus_handles: Vec<(ElementId, FocusHandle)>,
    pub children: Vec<TooltipChild<P>>,
}

pub trait TooltipChildNode<P: Clone + 'static>: Sized {
    fn with_tooltip_context(self, context: TooltipContext<P>) -> Self;

    fn wire_tooltip_child(
        self,
        _wiring: &mut TooltipChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct TooltipChildWiring<P: Clone + 'static> {
    root_id: ElementId,
    next_trigger_order: usize,
    provider_config: TooltipProviderConfig,
    triggers: Vec<TooltipTriggerMetadata<P>>,
    trigger_focus_handles: Vec<(ElementId, FocusHandle)>,
}

impl<P: Clone + 'static> TooltipChildWiring<P> {
    pub fn new(root_id: ElementId, provider_config: TooltipProviderConfig) -> Self {
        Self {
            root_id,
            next_trigger_order: 0,
            provider_config,
            triggers: Vec::new(),
            trigger_focus_handles: Vec::new(),
        }
    }

    pub fn effective_delay(&self, delay: Option<Duration>) -> Duration {
        delay.unwrap_or_else(|| self.provider_config.delay())
    }

    pub fn effective_close_delay(&self, close_delay: Option<Duration>) -> Duration {
        close_delay.unwrap_or_else(|| self.provider_config.close_delay())
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        scoped_trigger_id(&self.root_id, id)
    }

    pub fn wire_portal_children(
        &mut self,
        children: Vec<TooltipPortalChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<TooltipPortalChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_tooltip_child(self, window, cx))
            .collect()
    }

    pub fn wire_positioner_children(
        &mut self,
        children: Vec<TooltipPositionerChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<TooltipPositionerChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_tooltip_child(self, window, cx))
            .collect()
    }

    pub fn wire_popup_children(
        &mut self,
        children: Vec<TooltipPopupChild<P>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<TooltipPopupChild<P>> {
        children
            .into_iter()
            .map(|child| child.wire_tooltip_child(self, window, cx))
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn register_trigger(
        &mut self,
        scoped_id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        disabled: bool,
        delay: Duration,
        close_delay: Duration,
        close_on_click: bool,
        payload: Option<P>,
    ) -> usize {
        let order = self.next_trigger_order;
        self.next_trigger_order += 1;
        self.trigger_focus_handles
            .push((scoped_id.clone(), focus_handle.clone()));
        self.triggers.push(TooltipTriggerMetadata::new(
            scoped_id,
            source_id,
            focus_handle,
            disabled,
            delay,
            close_delay,
            close_on_click,
            payload,
            order,
            false,
        ));
        order
    }

    fn finish(self, children: Vec<TooltipChild<P>>) -> WiredTooltipChildren<P> {
        WiredTooltipChildren {
            triggers: self.triggers,
            trigger_focus_handles: self.trigger_focus_handles,
            children,
        }
    }
}

pub fn wire_children<P: Clone + 'static>(
    children: Vec<TooltipChild<P>>,
    context: TooltipContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> WiredTooltipChildren<P> {
    let provider_config = context.read(cx, |_runtime, props| props.provider());
    let mut wiring = TooltipChildWiring::new(context.root_id(), provider_config);
    let children = children
        .into_iter()
        .map(|child| child.wire_tooltip_child(&mut wiring, window, cx))
        .map(|child| child.with_tooltip_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipChild<P> {
    fn with_tooltip_context(self, context: TooltipContext<P>) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.with_tooltip_context(context)))
            }
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_tooltip_context(context))),
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_tooltip_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_tooltip_context(context))),
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_tooltip_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_tooltip_child(
        self,
        wiring: &mut TooltipChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.wire_tooltip_child(wiring, window, cx)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_tooltip_child(wiring, window, cx)))
            }
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_tooltip_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_tooltip_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipPortalChild<P> {
    fn with_tooltip_context(self, context: TooltipContext<P>) -> Self {
        match self {
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.with_tooltip_context(context)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_tooltip_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_tooltip_child(
        self,
        wiring: &mut TooltipChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.wire_tooltip_child(wiring, window, cx)))
            }
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_tooltip_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipPositionerChild<P> {
    fn with_tooltip_context(self, context: TooltipContext<P>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_tooltip_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_tooltip_child(
        self,
        wiring: &mut TooltipChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_tooltip_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipPopupChild<P> {
    fn with_tooltip_context(self, context: TooltipContext<P>) -> Self {
        match self {
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_tooltip_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub fn scoped_part_id(root_id: &ElementId, part: &str) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(part)))
}
