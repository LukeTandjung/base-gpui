use std::rc::Rc;

use gpui::{
    div, px, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::navigation_menu::{
    child_wiring::NavigationMenuChildNode, NavigationMenuAlign, NavigationMenuArrowStyleState,
    NavigationMenuBoundsKind, NavigationMenuContext, NavigationMenuSide,
};

type NavigationMenuArrowStyle = Rc<dyn Fn(NavigationMenuArrowStyleState, Div) -> Div + 'static>;

/// Decorative arrow following the resolved side/align of the **active**
/// trigger; it retargets with the anchor and exposes `uncentered`.
#[derive(IntoElement)]
pub struct NavigationMenuArrow<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NavigationMenuContext<T>>,
    side: NavigationMenuSide,
    align: NavigationMenuAlign,
    style_with_state: Option<NavigationMenuArrowStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuArrow<T> {
    fn default() -> Self {
        Self {
            base: div().w(px(8.0)).h(px(8.0)),
            children: Vec::new(),
            context: None,
            side: NavigationMenuSide::Bottom,
            align: NavigationMenuAlign::Center,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for NavigationMenuArrow<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuArrow<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuArrow<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.arrow_state(self.side, self.align))
            })
            .unwrap_or_else(|| {
                NavigationMenuArrowStyleState::new(false, self.side, self.align, None, None, false)
            });
        let measure_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        let base = position_arrow(base, state).children(self.children);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| {
                        runtime.set_bounds(NavigationMenuBoundsKind::Arrow, bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(base)
    }
}

fn position_arrow(base: Div, state: NavigationMenuArrowStyleState) -> Div {
    match (state.offset_x, state.offset_y) {
        (Some(offset_x), Some(offset_y)) => base.absolute().left(offset_x).top(offset_y),
        _ => base,
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuArrow<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuArrow<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: NavigationMenuSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: NavigationMenuAlign) -> Self {
        self.align = align;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuArrowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
