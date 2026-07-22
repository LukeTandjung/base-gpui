use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::drawer::{drawer_provider_registry, DrawerIndentBackgroundStyleState};

/// Backdrop behind the indented app shell, styled from the provider's `active`
/// flag. Inactive without a mounted `DrawerProvider`.
#[derive(IntoElement)]
pub struct DrawerIndentBackground {
    base: Div,
    children: Vec<AnyElement>,
    style_with_state: Option<Rc<dyn Fn(DrawerIndentBackgroundStyleState, Div) -> Div + 'static>>,
}

impl Default for DrawerIndentBackground {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            style_with_state: None,
        }
    }
}

impl ParentElement for DrawerIndentBackground {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for DrawerIndentBackground {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for DrawerIndentBackground {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let registry = drawer_provider_registry(window, cx);
        let state = registry.read(cx).indent_background_state();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl DrawerIndentBackground {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerIndentBackgroundStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
