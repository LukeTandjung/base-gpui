use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxAlign, ComboboxArrowStyleState, ComboboxContext,
    ComboboxSide,
};

/// Combobox-local equivalent of `popover_arrow.rs`.
#[derive(IntoElement)]
pub struct ComboboxArrow<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    side: ComboboxSide,
    align: ComboboxAlign,
    style_with_state: Option<Rc<dyn Fn(ComboboxArrowStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxArrow<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: ComboboxSide::Bottom,
            align: ComboboxAlign::Start,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxArrow<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxArrow<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxArrow<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.arrow_state(self.side, self.align))
            })
            .unwrap_or_else(|| ComboboxArrowStyleState::new(false, self.side, self.align, false));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxArrow<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxArrow<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: ComboboxSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: ComboboxAlign) -> Self {
        self.align = align;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxArrowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
