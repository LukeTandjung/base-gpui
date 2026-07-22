use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::combobox::{child_wiring::ComboboxChildNode, ComboboxContext, ComboboxStatusStyleState};

/// Plain container: renders its children. Live-region announcement mechanics
/// are deferred to the AccessKit follow-up.
#[derive(IntoElement)]
pub struct ComboboxStatus<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    style_with_state: Option<Rc<dyn Fn(ComboboxStatusStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxStatus<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxStatus<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxStatus<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxStatus<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(ComboboxStatusStyleState, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxStatus<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxStatus<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxStatusStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
