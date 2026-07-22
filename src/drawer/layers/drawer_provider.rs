use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::drawer::drawer_provider_registry;

/// App-level coordinator wrapper: mounting it activates the provider registry
/// so `DrawerIndent` / `DrawerIndentBackground` receive live drawer state.
/// Takes children and no other required props.
#[derive(IntoElement)]
pub struct DrawerProvider {
    base: Div,
    children: Vec<AnyElement>,
}

impl Default for DrawerProvider {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for DrawerProvider {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for DrawerProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for DrawerProvider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let registry = drawer_provider_registry(window, cx);
        registry.update(cx, |registry, _| registry.mark_provider_mounted());

        self.base.children(self.children)
    }
}

impl DrawerProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
