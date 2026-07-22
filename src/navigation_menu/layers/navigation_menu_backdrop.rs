use std::rc::Rc;

use gpui::{div, App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::navigation_menu::{
    child_wiring::NavigationMenuChildNode, NavigationMenuBackdropStyleState, NavigationMenuContext,
};

type NavigationMenuBackdropStyle =
    Rc<dyn Fn(NavigationMenuBackdropStyleState, Div) -> Div + 'static>;

/// Presentation-only backdrop: it never captures pointer events and never
/// dismisses; outside-press dismissal is the positioner's runtime path.
#[derive(IntoElement)]
pub struct NavigationMenuBackdrop<T: Clone + Eq + 'static> {
    base: Div,
    context: Option<NavigationMenuContext<T>>,
    keep_mounted: bool,
    style_with_state: Option<NavigationMenuBackdropStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuBackdrop<T> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuBackdrop<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuBackdrop<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.backdrop_state(self.keep_mounted)))
            .unwrap_or_else(|| NavigationMenuBackdropStyleState::new(false, self.keep_mounted));
        if !state.mounted {
            return div();
        }

        match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuBackdrop<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuBackdrop<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
