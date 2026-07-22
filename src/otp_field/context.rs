use std::{rc::Rc, sync::Arc};

use gpui::{App, ElementId, Entity, FocusHandle, SharedString, Window};

use crate::{
    form::{FormContext, FormSubmitReason},
    otp_field::{
        normalize_otp_value, OTPFieldMove, OTPFieldProps, OTPFieldRuntime, OTPFieldUpdateOutcome,
    },
    primitives::input::InputRuntime,
};

pub struct OTPFieldContext {
    id: ElementId,
    runtime: Entity<OTPFieldRuntime>,
    input: Entity<InputRuntime>,
    props: Rc<OTPFieldProps>,
    controlled: Rc<Option<SharedString>>,
    focus_handle: FocusHandle,
    form: Rc<Option<FormContext>>,
    field_valid: Option<bool>,
}

impl Clone for OTPFieldContext {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            input: self.input.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
            focus_handle: self.focus_handle.clone(),
            form: Rc::clone(&self.form),
            field_valid: self.field_valid,
        }
    }
}

impl OTPFieldContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<SharedString>,
        default: SharedString,
        props: OTPFieldProps,
        focus_handle: FocusHandle,
        form: Option<FormContext>,
        field_valid: Option<bool>,
    ) -> Self {
        let id = id.into();
        let initial = controlled.clone().unwrap_or(default);
        let initial = normalize_otp_value(
            initial.as_ref(),
            props.validation_type(),
            props.normalize_value(),
            props.length(),
        )
        .value;

        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| OTPFieldRuntime::new(&initial));
        let input_id = ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("input"));
        let input: Entity<InputRuntime> = window.use_keyed_state(input_id, cx, {
            let focus_handle = focus_handle.clone();
            let initial = initial.clone();
            move |window, cx| {
                InputRuntime::new_with_focus_handle(
                    SharedString::from(initial),
                    focus_handle,
                    window,
                    cx,
                )
            }
        });

        let context = Self {
            id,
            runtime,
            input,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
            focus_handle,
            form: Rc::new(form),
            field_valid,
        };
        context.reconcile(cx);
        context
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&OTPFieldRuntime, &OTPFieldProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut OTPFieldRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);
            cx.notify();
            output
        })
    }

    pub fn reconcile(&self, cx: &mut App) {
        let props = Rc::clone(&self.props);
        let controlled = Rc::clone(&self.controlled);
        self.runtime.update(cx, |runtime, cx| {
            let observed = controlled.as_ref().clone().map(|value| {
                normalize_otp_value(
                    value.as_ref(),
                    props.validation_type(),
                    props.normalize_value(),
                    props.length(),
                )
                .value
            });
            runtime.reconcile(observed, controlled.is_some(), props.as_ref());
            cx.notify();
        });
    }

    pub fn id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn child_id(&self, child: impl Into<SharedString>) -> ElementId {
        ElementId::NamedChild(Arc::new(self.id.clone()), child.into())
    }

    pub fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }

    pub fn input_runtime(&self) -> Entity<InputRuntime> {
        self.input.clone()
    }

    pub fn field_valid(&self) -> Option<bool> {
        self.field_valid
    }

    pub fn record_slot_bounds(
        &self,
        index: usize,
        bounds: gpui::Bounds<gpui::Pixels>,
        cx: &mut App,
    ) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.set_slot_bounds(index, bounds) {
                cx.notify();
            }
        });
    }

    pub fn sync_focus(&self, focused: bool, _window: &mut Window, cx: &mut App) {
        self.update(cx, |runtime| {
            runtime.sync_focused(focused, self.props.as_ref())
        });
    }

    pub fn activate_slot(&self, index: usize, window: &mut Window, cx: &mut App) {
        if self.props.disabled() {
            return;
        }
        self.focus_handle.focus(window, cx);
        self.update(cx, |runtime| {
            runtime.sync_focused(true, self.props.as_ref());
            runtime.activate_slot(index, self.props.as_ref());
        });
    }

    pub fn move_active(&self, movement: OTPFieldMove, _window: &mut Window, cx: &mut App) {
        self.update(cx, |runtime| {
            runtime.move_active(movement, self.props.as_ref())
        });
    }

    pub fn input_edited(&self, value: SharedString, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| {
            runtime.bridge_edited(value.as_ref(), self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn paste(&self, text: SharedString, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| {
            runtime.paste_text(text.as_ref(), self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn backspace(&self, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| runtime.backspace(self.props.as_ref()));
        self.emit(outcome, window, cx);
    }

    pub fn delete(&self, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| runtime.delete(self.props.as_ref()));
        self.emit(outcome, window, cx);
    }

    pub fn clear(&self, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| runtime.clear_all(self.props.as_ref()));
        self.emit(outcome, window, cx);
    }

    fn emit(&self, outcome: OTPFieldUpdateOutcome, window: &mut Window, cx: &mut App) {
        if let Some(invalid) = outcome.invalid {
            if let Some(on_value_invalid) = self.props.on_value_invalid() {
                on_value_invalid(invalid.value, invalid.details, window, cx);
            }
        }

        if let Some(change) = outcome.change {
            if let Some(on_value_change) = self.props.on_value_change() {
                on_value_change(change.value, change.details, window, cx);
            }
        }

        if let Some(complete) = outcome.complete {
            if let Some(on_value_complete) = self.props.on_value_complete() {
                on_value_complete(complete.value.clone(), complete.details, window, cx);
            }
            if self.props.auto_submit() {
                if let Some(form) = self.form.as_ref() {
                    form.submit(FormSubmitReason::Programmatic, window, cx);
                }
            }
        }
    }
}
