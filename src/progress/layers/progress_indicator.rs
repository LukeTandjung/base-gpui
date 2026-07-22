use std::rc::Rc;

use gpui::{
    div, relative, App, Div, ElementId, InteractiveElement as _, IntoElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::progress::{ProgressContext, ProgressStyleState};

#[derive(IntoElement)]
pub struct ProgressIndicator {
    id: Option<ElementId>,
    base: Div,
    context: Option<ProgressContext>,
    style_with_state: Option<Rc<dyn Fn(ProgressStyleState, Div) -> Div + 'static>>,
}

impl Default for ProgressIndicator {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for ProgressIndicator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressIndicator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("ProgressIndicator must be rendered inside ProgressTrack");
        let id = self.id.unwrap_or_else(|| context.child_id("indicator"));
        let style_state = context.read(|runtime| runtime.state());

        // Determinate: fill the track proportionally. Indeterminate: no
        // default fill (Base UI's empty style object) — appearance is left
        // entirely to `style_with_state`.
        let base = match style_state.percentage {
            Some(percentage) => self.base.w(relative((percentage / 100.0) as f32)),
            None => self.base,
        };

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, base),
            None => base,
        };

        base.id(id)
    }
}

impl ProgressIndicator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_progress_context(mut self, context: ProgressContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ProgressStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
