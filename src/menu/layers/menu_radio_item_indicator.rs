use std::rc::Rc;

use gpui::{div, AnyElement, Div, IntoElement, ParentElement, StyleRefinement, Styled};

use crate::menu::MenuRadioItemIndicatorStyleState;

#[derive(derive_setters::Setters)]
pub struct MenuRadioItemIndicator<P: Clone + 'static, V: Clone + Eq + 'static> {
    #[setters(skip)]
    base: Div,
    #[setters(skip)]
    children: Vec<AnyElement>,
    keep_mounted: bool,
    #[setters(skip)]
    facts: Option<(bool, bool, bool)>,
    #[setters(skip)]
    _payload: std::marker::PhantomData<(P, V)>,
    #[setters(skip)]
    style_with_state: Option<Rc<dyn Fn(MenuRadioItemIndicatorStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> Default for MenuRadioItemIndicator<P, V> {
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

impl<P: Clone + 'static, V: Clone + Eq + 'static> ParentElement for MenuRadioItemIndicator<P, V> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> Styled for MenuRadioItemIndicator<P, V> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> IntoElement for MenuRadioItemIndicator<P, V> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let (checked, highlighted, disabled) = self.facts.unwrap_or((false, false, false));
        let state = MenuRadioItemIndicatorStyleState::new(
            checked,
            highlighted,
            disabled,
            checked || self.keep_mounted,
        );
        if !state.present {
            return div().into_any_element();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children).into_any_element()
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> MenuRadioItemIndicator<P, V> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_item_facts(mut self, checked: bool, highlighted: bool, disabled: bool) -> Self {
        self.facts = Some((checked, highlighted, disabled));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuRadioItemIndicatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
