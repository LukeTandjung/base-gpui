use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Text, Window,
};

use crate::progress::{ProgressContext, ProgressStyleState};

/// GPUI translation of Base UI's function children: receives the formatted
/// text and raw value (`None` for both when indeterminate, replacing the
/// `'indeterminate'` sentinel string) and returns the text to render.
pub type ProgressValueDisplayHandler =
    Rc<dyn Fn(Option<&str>, Option<f64>) -> SharedString + 'static>;

#[derive(IntoElement)]
pub struct ProgressValue {
    id: Option<ElementId>,
    base: Div,
    context: Option<ProgressContext>,
    display: Option<ProgressValueDisplayHandler>,
    style_with_state: Option<Rc<dyn Fn(ProgressStyleState, Div) -> Div + 'static>>,
}

impl Default for ProgressValue {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            context: None,
            display: None,
            style_with_state: None,
        }
    }
}

impl Styled for ProgressValue {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressValue {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("ProgressValue must be rendered inside ProgressRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("value"));
        let style_state = context.read(|runtime| runtime.state());

        let text = match self.display.as_ref() {
            Some(display) => Some(display(style_state.formatted.as_deref(), style_state.value)),
            None => style_state.formatted.clone().map(SharedString::from),
        };

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let base = base.id(id);
        // Inaccessible text is the translation of Base UI's `aria-hidden` on
        // the Value part; the value reaches AT via the root's numeric value.
        match text {
            Some(text) => base.child(Text::new_inaccessible(text)),
            None => base,
        }
    }
}

impl ProgressValue {
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

    pub fn display(
        mut self,
        display: impl Fn(Option<&str>, Option<f64>) -> SharedString + 'static,
    ) -> Self {
        self.display = Some(Rc::new(display));
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
