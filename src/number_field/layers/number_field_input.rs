use std::rc::Rc;

use gpui::{App, Div, ElementId, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::{
    number_field::{NumberFieldContext, NumberFieldInputStyleState, NumberFieldStepDirection},
    primitives::input::{input, Input},
};

#[derive(IntoElement)]
pub struct NumberFieldInput {
    id: Option<ElementId>,
    input: Input,
    context: Option<NumberFieldContext>,
    style_with_state: Option<Rc<dyn Fn(NumberFieldInputStyleState, Div) -> Div + 'static>>,
}

impl Default for NumberFieldInput {
    fn default() -> Self {
        Self {
            id: None,
            input: input(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for NumberFieldInput {
    fn style(&mut self) -> &mut StyleRefinement {
        self.input.style()
    }
}

impl RenderOnce for NumberFieldInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("NumberFieldInput must be rendered inside NumberFieldRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("input"));
        let style_state = context.read(cx, |runtime, props| runtime.input_state(props));
        let input_value = style_state.root.input_value.clone();
        let disabled = style_state.root.disabled;
        let read_only = style_state.root.read_only;
        let required = style_state.root.required;
        let focus_handle = context.focus_handle();
        let change_context = context.clone();
        let focus_context = context.clone();
        let home_context = context.clone();
        let end_context = context.clone();
        let style_with_state = self.style_with_state;
        let number_state = style_state.clone();

        self.input
            .id(id)
            .value(input_value)
            .disabled(disabled)
            .read_only(read_only)
            .required(required)
            .focus_handle(focus_handle)
            .on_value_change_with_context(move |value, window, cx| {
                change_context.input_changed(value, window, cx);
            })
            .on_style_state(move |state, window, cx| {
                focus_context.sync_focus(state.focused, window, cx);
            })
            .on_home(move |_value, window, cx| {
                home_context.move_to_boundary(NumberFieldStepDirection::Down, window, cx);
                true
            })
            .on_end(move |_value, window, cx| {
                end_context.move_to_boundary(NumberFieldStepDirection::Up, window, cx);
                true
            })
            .style_with_state(move |_state, input| match style_with_state.as_ref() {
                Some(style_with_state) => style_with_state(number_state.clone(), input),
                None => input,
            })
    }
}

impl NumberFieldInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<gpui::SharedString>) -> Self {
        self.input = self.input.placeholder(placeholder);
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.input = self.input.auto_focus(auto_focus);
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.input = self.input.tab_index(tab_index);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NumberFieldInputStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
