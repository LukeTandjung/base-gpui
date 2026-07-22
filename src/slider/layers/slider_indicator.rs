use std::rc::Rc;

use gpui::{
    div, relative, App, Div, ElementId, InteractiveElement as _, IntoElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::slider::{SliderContext, SliderIndicatorStyleState, SliderOrientation};
use crate::utils::current_direction;

#[derive(IntoElement)]
pub struct SliderIndicator {
    id: Option<ElementId>,
    base: Div,
    context: Option<SliderContext>,
    style_with_state: Option<Rc<dyn Fn(SliderIndicatorStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderIndicator {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for SliderIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SliderIndicator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("SliderIndicator must be rendered inside SliderTrack");
        let id = self.id.unwrap_or_else(|| context.child_id("indicator"));
        let style_state = context.read(cx, |runtime, props| runtime.indicator_state(props));
        let orientation = style_state.root.orientation;
        let rtl = current_direction().is_rtl();
        let start = style_state.start_fraction as f32;
        let end = style_state.end_fraction as f32;
        let positioned = style_state.positioned;

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let base = base.id(id).absolute();
        if !positioned {
            return base.invisible();
        }

        match orientation {
            SliderOrientation::Horizontal => {
                if rtl {
                    base.right(relative(start)).left(relative(1.0 - end))
                } else {
                    base.left(relative(start)).right(relative(1.0 - end))
                }
            }
            SliderOrientation::Vertical => base.bottom(relative(start)).top(relative(1.0 - end)),
        }
    }
}

impl SliderIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_slider_context(mut self, context: SliderContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderIndicatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
