//! Scroll Area Content: optional wrapper that keeps intrinsic content width
//! measurable for horizontal overflow — the GPUI analogue of Base UI's
//! `min-width: fit-content`. It refuses to shrink below its content
//! (`flex_none`), so `max_offset().x` reflects true overflow instead of the
//! content being clamped to the viewport width.

use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::scroll_area::{
    child_wiring::ScrollAreaChildNode, ScrollAreaContext, ScrollAreaRootStyleState,
};

#[derive(IntoElement)]
pub struct ScrollAreaContent {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    style_with_state: Option<Rc<dyn Fn(ScrollAreaRootStyleState, Div) -> Div + 'static>>,
    context: Option<ScrollAreaContext>,
}

impl Default for ScrollAreaContent {
    fn default() -> Self {
        Self {
            id: ElementId::from("scroll-area-content"),
            base: div(),
            children: Vec::new(),
            style_with_state: None,
            context: None,
        }
    }
}

impl Styled for ScrollAreaContent {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ScrollAreaContent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

impl ScrollAreaChildNode for ScrollAreaContent {
    fn with_scroll_area_context(mut self, context: ScrollAreaContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl RenderOnce for ScrollAreaContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style_state = match &self.context {
            Some(context) => context.read(cx, |runtime, _| runtime.root_state()),
            None => ScrollAreaRootStyleState::default(),
        };
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(self.id).flex_none().children(self.children)
    }
}
