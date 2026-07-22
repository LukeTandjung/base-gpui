use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::navigation_menu::{
    child_wiring::{NavigationMenuChildNode, NavigationMenuChildWiring},
    NavigationMenuContext, NavigationMenuPortalChild, NavigationMenuPortalStyleState,
};

type NavigationMenuPortalStyle = Rc<dyn Fn(NavigationMenuPortalStyleState, Div) -> Div + 'static>;

/// Renders the positioner chain only while mounted (or `keep_mounted`);
/// closed keep-mounted content reports closed style state.
#[derive(IntoElement)]
pub struct NavigationMenuPortal<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<NavigationMenuPortalChild<T>>,
    context: Option<NavigationMenuContext<T>>,
    keep_mounted: bool,
    style_with_state: Option<NavigationMenuPortalStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuPortal<T> {
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

impl<T: Clone + Eq + 'static> Styled for NavigationMenuPortal<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuPortal<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.portal_state(self.keep_mounted)))
            .unwrap_or_else(|| NavigationMenuPortalStyleState::new(false, self.keep_mounted));

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

        base.children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuPortal<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context.clone());
        let keep_mounted = self.keep_mounted;
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuPortalChild::Backdrop(backdrop) => {
                    let backdrop = match keep_mounted {
                        true => backdrop.keep_mounted(true),
                        false => *backdrop,
                    };
                    NavigationMenuPortalChild::Backdrop(Box::new(
                        backdrop.with_navigation_menu_context(context.clone()),
                    ))
                }
                NavigationMenuPortalChild::Positioner(positioner) => {
                    let positioner = match keep_mounted {
                        true => positioner.keep_mounted(true),
                        false => *positioner,
                    };
                    NavigationMenuPortalChild::Positioner(Box::new(
                        positioner.with_navigation_menu_context(context.clone()),
                    ))
                }
                NavigationMenuPortalChild::Any(any) => NavigationMenuPortalChild::Any(any),
            })
            .collect();
        self
    }

    fn wire_navigation_menu_child(
        mut self,
        wiring: &mut NavigationMenuChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuPortalChild::Positioner(positioner) => {
                    NavigationMenuPortalChild::Positioner(Box::new(
                        positioner.wire_navigation_menu_child(wiring, window, cx),
                    ))
                }
                other => other,
            })
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuPortal<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<NavigationMenuPortalChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuPortalChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuPortalStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
