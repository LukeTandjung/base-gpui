use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, Empty, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::radio_group::{
    child_wiring::RadioGroupRadioChildNode, RadioGroupIndicatorStyleState,
    RadioGroupRadioStyleState,
};

#[derive(IntoElement)]
pub struct RadioGroupIndicator {
    base: Div,
    children: Vec<AnyElement>,
    radio_state: Option<RadioGroupRadioStyleState>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(RadioGroupIndicatorStyleState, Div) -> Div + 'static>>,
}

impl Default for RadioGroupIndicator {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            radio_state: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl ParentElement for RadioGroupIndicator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for RadioGroupIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for RadioGroupIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let radio_state = self.radio_state.unwrap_or_default();
        let state = RadioGroupIndicatorStyleState::new(
            radio_state,
            self.keep_mounted || radio_state.checked,
        );

        if !state.present {
            return Empty.into_any_element();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children).into_any_element()
    }
}

impl RadioGroupRadioChildNode for RadioGroupIndicator {
    fn with_radio_state(mut self, state: RadioGroupRadioStyleState) -> Self {
        self.radio_state = Some(state);
        self
    }
}

impl RadioGroupIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(RadioGroupIndicatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
