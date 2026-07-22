use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::select::{
    child_wiring::SelectChildNode, SelectAlign, SelectArrowStyleState, SelectContext, SelectSide,
};

#[derive(IntoElement)]
pub struct SelectArrow<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SelectContext<T>>,
    side: SelectSide,
    align: SelectAlign,
    style_with_state: Option<Rc<dyn Fn(SelectArrowStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectArrow<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: SelectSide::Bottom,
            align: SelectAlign::Start,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for SelectArrow<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectArrow<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectArrow<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.arrow_state(self.side, self.align))
            })
            .unwrap_or_else(|| SelectArrowStyleState::new(false, self.side, self.align, false));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectArrow<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectArrow<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: SelectSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: SelectAlign) -> Self {
        self.align = align;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectArrowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
