use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, Window};

use crate::{
    field::{
        FieldControlRegistration, FieldProps, FieldRuntime, FieldValidationMode,
        FieldValidationResult, FieldValidityData, FieldValidityState,
    },
    form::FormContext,
};

thread_local! {
    static FIELD_CONTEXT_STACK: RefCell<Vec<FieldContext>> = const { RefCell::new(Vec::new()) };
}

pub fn current_field_context() -> Option<FieldContext> {
    FIELD_CONTEXT_STACK.with(|stack| stack.borrow().last().cloned())
}

pub fn with_field_context<Output>(context: FieldContext, f: impl FnOnce() -> Output) -> Output {
    struct FieldContextGuard;

    impl Drop for FieldContextGuard {
        fn drop(&mut self) {
            FIELD_CONTEXT_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    FIELD_CONTEXT_STACK.with(|stack| stack.borrow_mut().push(context));
    let _guard = FieldContextGuard;

    f()
}

pub struct FieldContext {
    runtime: Entity<FieldRuntime>,
    props: Rc<FieldProps>,
    form_context: Option<FormContext>,
}

impl Clone for FieldContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            form_context: self.form_context.clone(),
        }
    }
}

impl FieldContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: FieldProps,
        form_context: Option<FormContext>,
    ) -> Self {
        let runtime = window.use_keyed_state(id, cx, |_, _| FieldRuntime::new());

        Self {
            runtime,
            props: Rc::new(props),
            form_context,
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&FieldRuntime, &FieldProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut FieldRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);

            cx.notify();
            output
        })
    }

    pub fn begin_registration_pass(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, _cx| {
            runtime.begin_registration_pass();
        });
    }

    pub fn finish_registration_pass(&self, cx: &mut App) {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            if runtime.finish_registration_pass(props.as_ref()) {
                cx.notify();
            }
        });
    }

    pub fn take_validation_request(&self, cx: &mut App) -> bool {
        self.runtime
            .update(cx, |runtime, _cx| runtime.take_validation_request())
    }

    pub fn take_refresh_request(&self, cx: &mut App) -> bool {
        self.runtime
            .update(cx, |runtime, _cx| runtime.take_refresh_request())
    }

    pub fn register_control(&self, registration: FieldControlRegistration, cx: &mut App) {
        let props = Rc::clone(&self.props);
        let form_context = self.form_context.clone();
        let form_submit_attempted = form_context
            .as_ref()
            .map(|context| context.submit_attempted(cx))
            .unwrap_or(false);
        let changed_name = self.runtime.update(cx, |runtime, cx| {
            let had_control = runtime.has_registered_controls();
            let previous_value = runtime.value();
            let changed = runtime.register_control(registration);
            let value_changed = previous_value != runtime.value();

            if changed {
                cx.notify();
            }

            let should_validate = props.validation_mode() == FieldValidationMode::OnChange
                || (props.validation_mode() == FieldValidationMode::OnSubmit
                    && form_submit_attempted);

            if value_changed && should_validate {
                runtime.request_validation();
                cx.notify();
            }

            if value_changed && had_control {
                runtime.effective_name(props.as_ref())
            } else {
                None
            }
        });

        if let (Some(form_context), Some(name)) = (form_context, changed_name) {
            form_context.clear_external_error(name, cx);
        }
    }

    pub fn register_label(&self, text: Option<gpui::SharedString>, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _cx| runtime.register_label(text));
    }

    /// Returns the registered label text so field-aware controls can expose
    /// it as their accessible name via `.aria_label(...)`. Replaces Base UI's
    /// `aria-labelledby` id wiring, which has no builder in this gpui
    /// revision.
    pub fn label_text(&self, cx: &App) -> Option<gpui::SharedString> {
        self.read(cx, |runtime, _| runtime.label_text())
    }

    pub fn register_description(&self, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _cx| runtime.register_description());
    }

    pub fn register_error(&self, cx: &mut App) {
        self.runtime
            .update(cx, |runtime, _cx| runtime.register_error());
    }

    pub fn set_form_external_errors(&self, errors: Vec<gpui::SharedString>, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.set_form_external_errors(errors) {
                cx.notify();
            }
        });
    }

    pub fn mark_touched(&self, window: &mut Window, cx: &mut App) {
        let validation_mode = self.props.validation_mode();
        let changed = self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.mark_touched();
            if changed {
                cx.notify();
            }
            changed
        });

        if changed && validation_mode == FieldValidationMode::OnBlur {
            self.validate(window, cx);
        }
    }

    pub fn validate(&self, window: &mut Window, cx: &mut App) {
        let value = self.read(cx, |runtime, _| runtime.value());
        let initial_value = self.read(cx, |runtime, _| runtime.initial_value());
        let result = match self.props.validate() {
            Some(validate) => validate(&value, window, cx),
            None if self.read(cx, |runtime, _| runtime.required()) => {
                if !self.read(cx, |runtime, _| runtime.value_missing()) {
                    FieldValidationResult::Valid
                } else {
                    FieldValidationResult::Validity(FieldValidityData {
                        state: FieldValidityState::value_missing(),
                        error: "Required".into(),
                        errors: vec!["Required".into()],
                        value: value.clone(),
                        initial_value: initial_value.clone(),
                    })
                }
            }
            None => FieldValidationResult::Validity({
                let mut data = self.read(cx, |runtime, props| runtime.validity_data(props));
                if data.state.valid.is_none() {
                    data = FieldValidationResult::Valid
                        .into_validity_data(value.clone(), initial_value.clone());
                }
                data
            }),
        };
        let validity_data = result.into_validity_data(value, initial_value);

        self.runtime.update(cx, |runtime, cx| {
            if runtime.set_validity_data(validity_data) {
                cx.notify();
            }
        });
    }

    pub fn focus_control(&self, window: &mut Window, cx: &mut App) {
        if let Some(focus_handle) = self.read(cx, |runtime, _| runtime.focus_handle()) {
            focus_handle.focus(window, cx);
        }
    }
}
