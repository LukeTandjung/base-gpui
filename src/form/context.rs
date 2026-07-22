use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, SharedString, Window};

use crate::{
    field::FieldValidationMode,
    form::{
        FormFieldRegistration, FormFieldSnapshot, FormProps, FormRuntime, FormSubmitDetails,
        FormSubmitReason,
    },
};

thread_local! {
    static FORM_CONTEXT_STACK: RefCell<Vec<FormContext>> = const { RefCell::new(Vec::new()) };
}

pub fn current_form_context() -> Option<FormContext> {
    FORM_CONTEXT_STACK.with(|stack| stack.borrow().last().cloned())
}

pub fn with_form_context<Output>(context: FormContext, f: impl FnOnce() -> Output) -> Output {
    struct FormContextGuard;

    impl Drop for FormContextGuard {
        fn drop(&mut self) {
            FORM_CONTEXT_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    FORM_CONTEXT_STACK.with(|stack| stack.borrow_mut().push(context));
    let _guard = FormContextGuard;

    f()
}

pub struct FormContext {
    runtime: Entity<FormRuntime>,
    props: Rc<FormProps>,
}

impl Clone for FormContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
        }
    }
}

impl FormContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: FormProps,
    ) -> Self {
        let runtime = window.use_keyed_state(id, cx, |_, _| FormRuntime::new());
        runtime.update(cx, |runtime, _cx| {
            runtime.sync_external_errors(props.errors());
        });

        Self {
            runtime,
            props: Rc::new(props),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&FormRuntime, &FormProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut FormRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);

            cx.notify();
            output
        })
    }

    pub fn sync_external_errors(&self, cx: &mut App) {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            if runtime.sync_external_errors(props.errors()) {
                cx.notify();
            }
        });
    }

    pub fn begin_registration_pass(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, _cx| {
            runtime.begin_registration_pass();
        });
    }

    pub fn finish_registration_pass(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.finish_registration_pass() {
                cx.notify();
            }
        });
    }

    pub fn register_field(&self, registration: FormFieldRegistration, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.register_field(registration) {
                cx.notify();
            }
        });
    }

    pub fn validation_mode(&self) -> FieldValidationMode {
        self.props.validation_mode()
    }

    pub fn submit_attempted(&self, cx: &App) -> bool {
        self.read(cx, |runtime, _props| runtime.submit_attempted())
    }

    pub fn external_errors_for(&self, name: Option<&SharedString>, cx: &App) -> Vec<SharedString> {
        self.read(cx, |runtime, _props| runtime.external_errors_for(name))
    }

    pub fn clear_external_error(&self, name: SharedString, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.clear_external_error(&name) {
                cx.notify();
            }
        });
    }

    pub fn validate(&self, window: &mut Window, cx: &mut App) -> bool {
        let handlers = self.read(cx, |runtime, _props| runtime.validation_handlers());
        let snapshots = handlers
            .into_iter()
            .map(|handler| handler(window, cx))
            .collect::<Vec<FormFieldSnapshot>>();

        self.update(cx, |runtime| {
            runtime.refresh_snapshots(snapshots);
        });

        self.read(cx, |runtime, _props| runtime.submission_result().valid)
    }

    pub fn validate_field(
        &self,
        name: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let name = name.into();
        let handlers = self.read(cx, |runtime, _props| {
            runtime.validation_handlers_for_name(&name)
        });
        let snapshots = handlers
            .into_iter()
            .map(|handler| handler(window, cx))
            .collect::<Vec<FormFieldSnapshot>>();

        self.update(cx, |runtime| {
            runtime.refresh_snapshots(snapshots);
        });

        self.read(cx, |runtime, _props| runtime.submission_result().valid)
    }

    pub fn submit(&self, reason: FormSubmitReason, window: &mut Window, cx: &mut App) {
        self.update(cx, |runtime| {
            runtime.mark_submit_attempted();
        });
        self.validate(window, cx);

        let (result, on_submit) = self.read(cx, |runtime, props| {
            (runtime.submission_result(), props.on_form_submit().cloned())
        });

        if !result.valid {
            if let Some(focus_handle) = result.first_invalid_focus {
                focus_handle.focus(window, cx);
            }
            return;
        }

        if let Some(on_submit) = on_submit {
            on_submit(result.values, FormSubmitDetails::new(reason), window, cx);
        }
    }
}
