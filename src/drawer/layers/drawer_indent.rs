use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::drawer::{drawer_provider_registry, DrawerIndentStyleState};

/// App-shell wrapper styled from the provider state (iOS-style scale-back is
/// applied by the user through `style_with_state`). Inactive — never panicking —
/// without a mounted `DrawerProvider`.
#[derive(IntoElement)]
pub struct DrawerIndent {
    base: Div,
    children: Vec<AnyElement>,
    style_with_state: Option<Rc<dyn Fn(DrawerIndentStyleState, Div) -> Div + 'static>>,
}

impl Default for DrawerIndent {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            style_with_state: None,
        }
    }
}

impl ParentElement for DrawerIndent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for DrawerIndent {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for DrawerIndent {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let registry = drawer_provider_registry(window, cx);
        let state = registry.read(cx).indent_state();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl DrawerIndent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerIndentStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
