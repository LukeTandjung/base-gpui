use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::preview_card::{
    child_wiring::PreviewCardChildNode, PreviewCardActivationDirection, PreviewCardContext,
    PreviewCardInstant, PreviewCardPayloadContentBuilder, PreviewCardViewportStyleState,
};

/// Renders the current payload-driven content and exposes trigger-to-trigger
/// transition facts. Full morphing previous/current content containers follow
/// the Tooltip decision: the facts are exposed and morphing is deferred until
/// GPUI animation infrastructure warrants it.
#[derive(IntoElement)]
pub struct PreviewCardViewport<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PreviewCardContext<P>>,
    payload_content: Option<PreviewCardPayloadContentBuilder<P>>,
    style_with_state: Option<Rc<dyn Fn(PreviewCardViewportStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PreviewCardViewport<P> {
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

impl<P: Clone + 'static> ParentElement for PreviewCardViewport<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PreviewCardViewport<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardViewport<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if let Some(context) = self.context.as_ref() {
            context.update(cx, |runtime| runtime.set_viewport_present(true));
        }
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.viewport_state()))
            .unwrap_or_else(|| {
                PreviewCardViewportStyleState::new(
                    PreviewCardActivationDirection::None,
                    false,
                    PreviewCardInstant::None,
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

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardViewport<P> {
    fn with_preview_card_context(mut self, context: PreviewCardContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> PreviewCardViewport<P> {
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
        style: impl Fn(PreviewCardViewportStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
