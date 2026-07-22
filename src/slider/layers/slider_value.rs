use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Text, Window,
};

use crate::slider::{SliderContext, SliderValueStyleState};

pub type SliderValueDisplayHandler = Rc<dyn Fn(&[SharedString], &[f64]) -> SharedString + 'static>;

#[derive(IntoElement)]
pub struct SliderValue {
    id: Option<ElementId>,
    base: Div,
    context: Option<SliderContext>,
    display: Option<SliderValueDisplayHandler>,
    style_with_state: Option<Rc<dyn Fn(SliderValueStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderValue {
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

impl Styled for SliderValue {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SliderValue {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("SliderValue must be rendered inside SliderRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("value"));
        let style_state = context.read(cx, |runtime, props| runtime.value_state(props));

        let text = match self.display.as_ref() {
            Some(display) => display(&style_state.formatted_values, &style_state.values),
            None => SharedString::from(
                style_state
                    .formatted_values
                    .iter()
                    .map(SharedString::as_ref)
                    .collect::<Vec<_>>()
                    .join(" \u{2013} "),
            ),
        };

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // No role, and the display string is rendered inaccessibly: the
        // thumbs already announce the numeric values, so a mirrored text
        // node would double-announce. Base UI's `aria-live` on this part
        // has no gpui equivalent; value changes are conveyed by the focused
        // thumb's `aria_numeric_value` updating instead.
        base.id(id).child(Text::new_inaccessible(text))
    }
}

impl SliderValue {
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

    pub fn display(
        mut self,
        display: impl Fn(&[SharedString], &[f64]) -> SharedString + 'static,
    ) -> Self {
        self.display = Some(Rc::new(display));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderValueStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
