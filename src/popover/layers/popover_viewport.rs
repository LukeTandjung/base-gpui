use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::popover::{child_wiring::PopoverChildNode, PopoverContext, PopoverViewportStyleState};

#[derive(IntoElement)]
pub struct PopoverViewport<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PopoverContext<P>>,
    style_with_state: Option<Rc<dyn Fn(PopoverViewportStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverViewport<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for PopoverViewport<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PopoverViewport<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverViewport<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.viewport_state()))
            .unwrap_or_else(|| {
                PopoverViewportStyleState::new(
                    crate::popover::PopoverActivationDirection::None,
                    false,
                    false,
                    None,
                )
            });
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverViewport<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> PopoverViewport<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverViewportStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
