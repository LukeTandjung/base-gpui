use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::tooltip::{
    child_wiring::TooltipChildNode, TooltipContext, TooltipPayloadContentBuilder,
    TooltipViewportStyleState,
};

#[derive(IntoElement)]
pub struct TooltipViewport<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TooltipContext<P>>,
    payload_content: Option<TooltipPayloadContentBuilder<P>>,
    style_with_state: Option<Rc<dyn Fn(TooltipViewportStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for TooltipViewport<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            payload_content: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for TooltipViewport<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for TooltipViewport<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for TooltipViewport<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.viewport_state()))
            .unwrap_or_else(|| {
                TooltipViewportStyleState::new(
                    crate::tooltip::TooltipActivationDirection::None,
                    false,
                    crate::tooltip::TooltipInstant::Delay,
                    None,
                    None,
                    None,
                    None,
                )
            });
        let mut children = Vec::new();
        if let Some(payload_content) = self.payload_content {
            let payload = self
                .context
                .as_ref()
                .and_then(|context| context.read(cx, |runtime, _| runtime.active_payload()));
            children.push(payload_content(payload.as_ref(), window, cx));
        }
        children.extend(self.children);

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(children)
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipViewport<P> {
    fn with_tooltip_context(mut self, context: TooltipContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> TooltipViewport<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn payload_content(
        mut self,
        content: impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.payload_content = Some(Rc::new(content));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TooltipViewportStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
