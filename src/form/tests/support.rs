use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Pixels, Render, SharedString, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::{
    field::{
        FieldError, FieldErrorStyleState, FieldRoot, FieldRootStyleState, FieldValidationMode,
        FieldValidity, FieldValidityStyleState,
    },
    form::{
        current_form_context, Form, FormContext, FormErrors, FormStyleState, FormSubmitDetails,
        FormSubmitReason, FormValue, FormValues,
    },
    input::{Input, InputStyleState},
};

#[derive(Clone, Debug)]
pub struct FormTestConfig {
    pub value: SharedString,
    pub required: bool,
    pub field_disabled: bool,
    pub external_errors: FormErrors,
    pub form_validation_mode: FieldValidationMode,
    pub field_validation_mode: Option<FieldValidationMode>,
}

impl Default for FormTestConfig {
    fn default() -> Self {
        Self {
            value: SharedString::default(),
            required: false,
            field_disabled: false,
            external_errors: FormErrors::new(),
            form_validation_mode: FieldValidationMode::OnSubmit,
            field_validation_mode: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FormSubmissionObservation {
    pub values: FormValues,
    pub details: FormSubmitDetails,
}

#[derive(Clone, Default)]
pub struct FormObservations {
    pub form_states: Vec<FormStyleState>,
    pub field_states: Vec<FieldRootStyleState>,
    pub error_states: Vec<FieldErrorStyleState>,
    pub validity_states: Vec<FieldValidityStyleState>,
    pub input_states: Vec<InputStyleState>,
    pub submissions: Vec<FormSubmissionObservation>,
}

impl FormObservations {
    fn begin_render(&mut self) {
        self.form_states.clear();
        self.field_states.clear();
        self.error_states.clear();
        self.validity_states.clear();
        self.input_states.clear();
    }

    pub fn last_field_state(&self) -> FieldRootStyleState {
        self.field_states
            .last()
            .copied()
            .expect("field state should be observed")
    }

    pub fn last_error_state(&self) -> FieldErrorStyleState {
        self.error_states
            .last()
            .cloned()
            .expect("field error state should be observed")
    }

    pub fn last_validity_state(&self) -> FieldValidityStyleState {
        self.validity_states
            .last()
            .cloned()
            .expect("field validity state should be observed")
    }

    pub fn last_input_state(&self) -> InputStyleState {
        self.input_states
            .last()
            .cloned()
            .expect("input state should be observed")
    }
}

pub struct FormTestView {
    pub config: FormTestConfig,
    observations: Rc<RefCell<FormObservations>>,
    form_context: Rc<RefCell<Option<FormContext>>>,
}

impl FormTestView {
    pub fn new(config: FormTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(FormObservations::default())),
            form_context: Rc::new(RefCell::new(None)),
        }
    }

    pub fn read_observations(&self) -> FormObservations {
        self.observations.borrow().clone()
    }
}

impl Render for FormTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();
        self.form_context.borrow_mut().take();

        let form_state_observations = Rc::clone(&self.observations);
        let submit_observations = Rc::clone(&self.observations);
        let mut form = Form::new()
            .id("form-test")
            .validation_mode(self.config.form_validation_mode)
            .errors(self.config.external_errors.clone())
            .flex()
            .flex_col()
            .gap_2()
            .on_form_submit(move |values, details, _window, _cx| {
                submit_observations
                    .borrow_mut()
                    .submissions
                    .push(FormSubmissionObservation { values, details });
            })
            .style_with_state(move |state, form| {
                form_state_observations.borrow_mut().form_states.push(state);
                form.debug_selector(|| "form-root".into())
            });

        let field_state_observations = Rc::clone(&self.observations);
        let error_observations = Rc::clone(&self.observations);
        let validity_observations = Rc::clone(&self.observations);
        let input_observations = Rc::clone(&self.observations);
        let mut field = FieldRoot::new()
            .id("field-email")
            .name("email")
            .disabled(self.config.field_disabled)
            .flex()
            .flex_col()
            .gap_1()
            .style_with_state(move |state, field| {
                field_state_observations
                    .borrow_mut()
                    .field_states
                    .push(state);
                field.debug_selector(|| "email-field".into())
            })
            .child(
                Input::new()
                    .id("email-input")
                    .name("email")
                    .default_value(self.config.value.clone())
                    .required(self.config.required)
                    .w(px(220.0))
                    .h(px(28.0))
                    .style_with_state(move |state, input| {
                        input_observations.borrow_mut().input_states.push(state);
                        input.debug_selector(|| "email-input".into())
                    }),
            )
            .child(FieldError::new().style_with_state(move |state, error| {
                error_observations.borrow_mut().error_states.push(state);
                error.debug_selector(|| "email-error".into())
            }))
            .child(
                FieldValidity::new().style_with_state(move |state, validity| {
                    validity_observations
                        .borrow_mut()
                        .validity_states
                        .push(state);
                    validity
                }),
            );

        if let Some(validation_mode) = self.config.field_validation_mode {
            field = field.validation_mode(validation_mode);
        }

        form = form
            .child(field)
            .child(
                div()
                    .id("arbitrary-child")
                    .size(px(16.0))
                    .debug_selector(|| "arbitrary-child".into()),
            )
            .child(FormContextProbe::new(&self.form_context));

        div().size_full().p_4().child(form)
    }
}

#[derive(IntoElement)]
struct FormContextProbe {
    context: Rc<RefCell<Option<FormContext>>>,
}

impl FormContextProbe {
    fn new(context: &Rc<RefCell<Option<FormContext>>>) -> Self {
        Self {
            context: Rc::clone(context),
        }
    }
}

impl gpui::RenderOnce for FormContextProbe {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        *self.context.borrow_mut() = current_form_context();

        div().size(px(0.0))
    }
}

pub fn open_form(cx: &mut TestAppContext, config: FormTestConfig) -> WindowHandle<FormTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(360.0), px(240.0)), move |_, _| {
        FormTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<FormTestView>,
) -> FormObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("form test window should be open")
}

pub fn submit_form(cx: &mut TestAppContext, window: WindowHandle<FormTestView>) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.submit(FormSubmitReason::Programmatic, window, cx);
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn validate_form(cx: &mut TestAppContext, window: WindowHandle<FormTestView>) -> bool {
    let valid = window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.validate(window, cx)
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
    valid
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<FormTestView>,
    update: impl FnOnce(&mut FormTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<FormTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn blur(cx: &mut TestAppContext, window: WindowHandle<FormTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_input(cx: &mut TestAppContext, window: WindowHandle<FormTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<FormTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

pub fn text_value(value: &str) -> FormValue {
    FormValue::Text(SharedString::from(value))
}
