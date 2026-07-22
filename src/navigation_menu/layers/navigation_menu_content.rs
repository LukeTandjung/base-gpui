use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::navigation_menu::{
    child_wiring::NavigationMenuChildNode, NavigationMenuActivationDirection,
    NavigationMenuContentStyleState, NavigationMenuContext,
};

type NavigationMenuContentStyle = Rc<dyn Fn(NavigationMenuContentStyleState, Div) -> Div + 'static>;

/// One item's panel, rendered by the shared popup viewport (never in place
/// under its item — child wiring routes it there; no DOM re-parenting).
/// `keep_mounted(true)` keeps it mounted hidden with closed style state.
#[derive(IntoElement)]
pub struct NavigationMenuContent<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NavigationMenuContext<T>>,
    value: Option<T>,
    keep_mounted: bool,
    style_with_state: Option<NavigationMenuContentStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuContent<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for NavigationMenuContent<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuContent<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuContent<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = match (self.context.as_ref(), self.value.as_ref()) {
            (Some(context), Some(value)) => context.read(cx, |runtime, _| {
                runtime.content_state(value, self.keep_mounted)
            }),
            _ => NavigationMenuContentStyleState::new(
                false,
                self.keep_mounted,
                NavigationMenuActivationDirection::None,
            ),
        };

        if !state.mounted {
            return div();
        }

        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        if !state.open {
            base = base.opacity(0.0).invisible();
        }

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuContent<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuContent<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuContentStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// Internal wiring seam: the enclosing item hands the content its value.
    pub fn wired(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn content_value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}
