use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, Empty, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::checkbox::{
    child_wiring::CheckboxChildNode, CheckboxContext, CheckboxIndicatorStyleState,
};

#[derive(IntoElement)]
pub struct CheckboxIndicator {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<CheckboxContext>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(CheckboxIndicatorStyleState, Div) -> Div + 'static>>,
}

impl Default for CheckboxIndicator {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl ParentElement for CheckboxIndicator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for CheckboxIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for CheckboxIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.context.as_ref().map(|context| {
            context.read(cx, |runtime, props| {
                runtime.indicator_state(self.keep_mounted, props)
            })
        });
        let state = state.unwrap_or_else(|| {
            CheckboxIndicatorStyleState::new(Default::default(), self.keep_mounted)
        });

        if !state.present {
            return Empty.into_any_element();
        }

        let base = match self.style_with_state {
            Some(style) => style(state, self.base),
            None => self.base,
        };
        base.children(self.children).into_any_element()
    }
}

impl CheckboxChildNode for CheckboxIndicator {
    fn with_checkbox_context(mut self, context: CheckboxContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl CheckboxIndicator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }
    pub fn style_with_state(
        mut self,
        style: impl Fn(CheckboxIndicatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
