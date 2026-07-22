use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::navigation_menu::{
    child_wiring::NavigationMenuChildNode, NavigationMenuBoundsKind, NavigationMenuContent,
    NavigationMenuContext, NavigationMenuViewportStyleState,
};

type NavigationMenuViewportStyle =
    Rc<dyn Fn(NavigationMenuViewportStyleState, Div) -> Div + 'static>;

/// The clipping container inside the shared popup. It renders exactly the
/// active item's content every frame (plus kept-mounted hidden ones) — the
/// GPUI collapse of Base UI's `createPortal` re-parenting. Its measured size
/// and activation direction are the popup's morph facts.
#[derive(IntoElement)]
pub struct NavigationMenuViewport<T: Clone + Eq + 'static> {
    base: Div,
    contents: Vec<NavigationMenuContent<T>>,
    context: Option<NavigationMenuContext<T>>,
    style_with_state: Option<NavigationMenuViewportStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuViewport<T> {
    fn default() -> Self {
        Self {
            base: div().overflow_hidden(),
            contents: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuViewport<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuViewport<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };

        let state = context.read(cx, |runtime, _| runtime.viewport_state());

        // Each content self-gates on the active value / keep_mounted, so the
        // viewport simply renders the collected set every frame.
        let children = self
            .contents
            .into_iter()
            .map(|content| {
                content
                    .with_navigation_menu_context(context.clone())
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let measure_context = context.clone();
        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_context.update(cx, |runtime| {
                    runtime.set_bounds(NavigationMenuBoundsKind::Viewport, bounds)
                });
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(base.children(children))
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuViewport<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuViewport<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Internal wiring seam: receives the contents collected from items.
    pub fn with_contents(mut self, contents: Vec<NavigationMenuContent<T>>) -> Self {
        self.contents.extend(contents);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuViewportStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
