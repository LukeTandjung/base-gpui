use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::navigation_menu::{
    child_wiring::NavigationMenuChildNode, NavigationMenuContext, NavigationMenuIconStyleState,
};

type NavigationMenuIconStyle = Rc<dyn Fn(NavigationMenuIconStyleState, Div) -> Div + 'static>;

/// Renders caller-provided children (no hard-coded glyph) and exposes its
/// item's open state for styling (e.g. rotating a caret).
#[derive(IntoElement)]
pub struct NavigationMenuIcon<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NavigationMenuContext<T>>,
    value: Option<T>,
    style_with_state: Option<NavigationMenuIconStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuIcon<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for NavigationMenuIcon<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuIcon<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuIcon<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.icon_state(self.value.as_ref())))
            .unwrap_or_else(|| NavigationMenuIconStyleState::new(false));

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuIcon<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuIcon<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Internal wiring seam: the enclosing trigger hands the icon its item
    /// value.
    pub fn with_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuIconStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
