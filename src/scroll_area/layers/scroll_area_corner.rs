//! Scroll Area Corner: the intersection square where the two scrollbars
//! meet, sized from the measured scrollbar thicknesses and unmounted unless
//! both axes have overflow.

use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::scroll_area::{
    child_wiring::ScrollAreaChildNode, ScrollAreaContext, ScrollAreaCornerStyleState,
};

#[derive(IntoElement)]
pub struct ScrollAreaCorner {
    id: ElementId,
    base: Div,
    style_with_state: Option<Rc<dyn Fn(ScrollAreaCornerStyleState, Div) -> Div + 'static>>,
    context: Option<ScrollAreaContext>,
}

impl Default for ScrollAreaCorner {
    fn default() -> Self {
        Self {
            id: ElementId::from("scroll-area-corner"),
            base: div(),
            style_with_state: None,
            context: None,
        }
    }
}

impl Styled for ScrollAreaCorner {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ScrollAreaChildNode for ScrollAreaCorner {
    fn with_scroll_area_context(mut self, context: ScrollAreaContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl RenderOnce for ScrollAreaCorner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context else {
            return gpui::Empty.into_any_element();
        };
        let state = context.read(cx, |runtime, _| runtime.corner_state());
        if state.hidden {
            return gpui::Empty.into_any_element();
        }

        let base = self
            .base
            .absolute()
            .right_0()
            .bottom_0()
            .w(state.size.width)
            .h(state.size.height);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        base.id(self.id).into_any_element()
    }
}

impl ScrollAreaCorner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ScrollAreaCornerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
