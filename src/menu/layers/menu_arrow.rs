use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{MenuChildNode, MenuChildWiring},
    MenuArrowStyleState, MenuContext,
};

#[derive(IntoElement)]
pub struct MenuArrow<P: Clone + 'static = ()> {
    base: Div,
    context: Option<MenuContext<P>>,
    style_with_state: Option<Rc<dyn Fn(MenuArrowStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuArrow<P> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuArrow<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuArrow<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let state = context.read(cx, |runtime, _| {
            runtime.arrow_state(Default::default(), Default::default())
        });
        if !state.open {
            return div();
        }

        let measure_context = context.clone();
        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .absolute();
        if let Some(offset_x) = state.offset_x {
            base = base.left(offset_x);
        }
        if let Some(offset_y) = state.offset_y {
            base = base.top(offset_y);
        }

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed =
                    measure_context.update(cx, |runtime| runtime.set_arrow_bounds(bounds));
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(base)
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuArrow<P> {
    fn wire_menu_child(
        mut self,
        _wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuArrow<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuArrowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
