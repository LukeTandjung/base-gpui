use std::rc::Rc;

use gpui::{
    div, relative, App, Div, ElementId, InteractiveElement as _, IntoElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::meter::{MeterContext, MeterStyleState};

#[derive(IntoElement)]
pub struct MeterIndicator {
    id: Option<ElementId>,
    base: Div,
    context: Option<MeterContext>,
    style_with_state: Option<Rc<dyn Fn(MeterStyleState, Div) -> Div + 'static>>,
}

impl Default for MeterIndicator {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for MeterIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for MeterIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("MeterIndicator must be rendered inside MeterTrack");
        let id = self.id.unwrap_or_else(|| context.child_id("indicator"));
        let style_state = context.read(|runtime| runtime.state());

        // Fill the track proportionally to the derived percentage.
        let base = self
            .base
            .w(relative((style_state.percentage / 100.0) as f32));

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, base),
            None => base,
        };

        base.id(id)
    }
}

impl MeterIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_meter_context(mut self, context: MeterContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MeterStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
