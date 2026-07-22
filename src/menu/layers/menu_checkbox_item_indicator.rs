use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::menu::MenuCheckboxItemIndicatorStyleState;

#[derive(IntoElement)]
pub struct MenuCheckboxItemIndicator<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    keep_mounted: bool,
    facts: Option<(bool, bool, bool)>,
    _payload: std::marker::PhantomData<P>,
    style_with_state: Option<Rc<dyn Fn(MenuCheckboxItemIndicatorStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuCheckboxItemIndicator<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            keep_mounted: false,
            facts: None,
            _payload: std::marker::PhantomData,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for MenuCheckboxItemIndicator<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for MenuCheckboxItemIndicator<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuCheckboxItemIndicator<P> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let (checked, highlighted, disabled) = self.facts.unwrap_or((false, false, false));
        let state = MenuCheckboxItemIndicatorStyleState::new(
            checked,
            highlighted,
            disabled,
            checked || self.keep_mounted,
        );
        if !state.present {
            return div();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<P: Clone + 'static> MenuCheckboxItemIndicator<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn with_item_facts(mut self, checked: bool, highlighted: bool, disabled: bool) -> Self {
        self.facts = Some((checked, highlighted, disabled));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuCheckboxItemIndicatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
