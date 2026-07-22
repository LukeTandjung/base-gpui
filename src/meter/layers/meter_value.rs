use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Text, Window,
};

use crate::meter::{MeterContext, MeterStyleState};

/// GPUI translation of Base UI's function children: receives the formatted
/// text and raw value and returns the text to render.
pub type MeterValueDisplayHandler = Rc<dyn Fn(&str, f64) -> SharedString + 'static>;

#[derive(IntoElement)]
pub struct MeterValue {
    id: Option<ElementId>,
    base: Div,
    context: Option<MeterContext>,
    display: Option<MeterValueDisplayHandler>,
    style_with_state: Option<Rc<dyn Fn(MeterStyleState, Div) -> Div + 'static>>,
}

impl Default for MeterValue {
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

impl Styled for MeterValue {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for MeterValue {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("MeterValue must be rendered inside MeterRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("value"));
        let style_state = context.read(|runtime| runtime.state());

        let text = match self.display.as_ref() {
            Some(display) => display(style_state.formatted.as_str(), style_state.value),
            None => SharedString::from(style_state.formatted.clone()),
        };

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // Base UI marks the value `aria-hidden`; the inaccessible text plus
        // the absent role keeps it out of the a11y tree (the same string
        // reaches AT through the root's `aria_label`).
        base.id(id).child(Text::new_inaccessible(text))
    }
}

impl MeterValue {
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

    pub fn display(mut self, display: impl Fn(&str, f64) -> SharedString + 'static) -> Self {
        self.display = Some(Rc::new(display));
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
