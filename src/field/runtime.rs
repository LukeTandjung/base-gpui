use gpui::{FocusHandle, SharedString};

use crate::field::{
    FieldErrorMatch, FieldProps, FieldRootStyleState, FieldValidationMode, FieldValidityData,
    FieldValidityState, FieldValue,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldControlRegistration {
    key: SharedString,
    name: Option<SharedString>,
    value: FieldValue,
    disabled: bool,
    focused: bool,
    required: bool,
    value_missing: bool,
    focus_handle: Option<FocusHandle>,
}

impl FieldControlRegistration {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            name: None,
            value: FieldValue::Empty,
            disabled: false,
            focused: false,
            required: false,
            value_missing: false,
            focus_handle: None,
        }
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: FieldValue) -> Self {
        self.value = value;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn value_missing(mut self, value_missing: bool) -> Self {
        self.value_missing = value_missing;
        self
    }

    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn key(&self) -> &SharedString {
        &self.key
    }

    pub fn name_value(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn value_ref(&self) -> &FieldValue {
        &self.value
    }

    pub fn disabled_value(&self) -> bool {
        self.disabled
    }

    pub fn focused_value(&self) -> bool {
        self.focused
    }

    pub fn required_value(&self) -> bool {
        self.required
    }

    pub fn value_missing_value(&self) -> bool {
        self.value_missing
    }

    pub fn focus_handle_ref(&self) -> Option<&FocusHandle> {
        self.focus_handle.as_ref()
    }
}

#[derive(Clone, Debug)]
struct RegisteredControl {
    registration: FieldControlRegistration,
    initial_value: FieldValue,
    generation: u64,
}

impl RegisteredControl {
    fn new(registration: FieldControlRegistration, generation: u64) -> Self {
        Self {
            initial_value: registration.value.clone(),
            registration,
            generation,
        }
    }

    fn filled(&self) -> bool {
        self.registration.value.filled()
    }

    fn dirty(&self) -> bool {
        self.registration.value != self.initial_value
    }
}

#[derive(Clone, Debug)]
pub struct FieldRuntime {
    controls: Vec<RegisteredControl>,
    generation: u64,
    touched: bool,
    was_focused: bool,
    validity_data: FieldValidityData,
    label_registered: bool,
    label_text: Option<SharedString>,
    description_count: usize,
    error_count: usize,
    needs_validation: bool,
    needs_refresh: bool,
    form_external_errors: Vec<SharedString>,
}

impl Default for FieldRuntime {
    fn default() -> Self {
        Self {
            controls: Vec::new(),
            generation: 0,
            touched: false,
            was_focused: false,
            validity_data: FieldValidityData::default(),
            label_registered: false,
            label_text: None,
            description_count: 0,
            error_count: 0,
            needs_validation: false,
            needs_refresh: false,
            form_external_errors: Vec::new(),
        }
    }
}

impl FieldRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts one descendant registration pass for this render/layout traversal.
    pub fn begin_registration_pass(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.label_registered = false;
        self.label_text = None;
        self.description_count = 0;
        self.error_count = 0;
    }

    /// Registers or refreshes one field-aware control for the current pass.
    pub fn register_control(&mut self, registration: FieldControlRegistration) -> bool {
        if let Some(control) = self
            .controls
            .iter_mut()
            .find(|control| control.registration.key == registration.key)
        {
            let changed = control.registration != registration;
            control.registration = registration;
            control.generation = self.generation;
            if changed {
                self.needs_refresh = true;
            }
            return changed;
        }

        self.controls
            .push(RegisteredControl::new(registration, self.generation));
        self.needs_refresh = true;
        true
    }

    /// Finishes descendant registration and prunes removed controls.
    pub fn finish_registration_pass(&mut self, props: &FieldProps) -> bool {
        let previous_len = self.controls.len();
        self.controls
            .retain(|control| control.generation == self.generation);
        let pruned = previous_len != self.controls.len();

        let focused = self
            .controls
            .iter()
            .any(|control| control.registration.focused);
        let blurred = self.was_focused && !focused;
        self.was_focused = focused;
        if blurred {
            self.touched = true;
        }

        if props.validation_mode() == FieldValidationMode::OnBlur && blurred {
            self.needs_validation = true;
        }

        if pruned || blurred {
            self.needs_refresh = true;
        }

        pruned || blurred
    }

    /// Requests validation after the current registration/update pass finishes.
    pub fn request_validation(&mut self) {
        self.needs_validation = true;
    }

    /// Takes the pending validation request flag.
    pub fn take_validation_request(&mut self) -> bool {
        let needs_validation = self.needs_validation;
        self.needs_validation = false;
        needs_validation
    }

    /// Takes the pending refresh request flag.
    pub fn take_refresh_request(&mut self) -> bool {
        let needs_refresh = self.needs_refresh;
        self.needs_refresh = false;
        needs_refresh
    }

    /// Records that a label part rendered in this field, optionally storing
    /// its text so registered controls can expose it as their accessible
    /// name via `.aria_label(...)` (this gpui revision has no
    /// `aria-labelledby` relationship builder).
    pub fn register_label(&mut self, text: Option<SharedString>) {
        self.label_registered = true;
        if text.is_some() {
            self.label_text = text;
        }
    }

    /// Returns the registered label text, if a label part provided one.
    pub fn label_text(&self) -> Option<SharedString> {
        self.label_text.clone()
    }

    /// Records that a description part rendered in this field.
    pub fn register_description(&mut self) {
        self.description_count += 1;
    }

    /// Records that a present error part rendered in this field.
    pub fn register_error(&mut self) {
        self.error_count += 1;
    }

    pub fn label_registered(&self) -> bool {
        self.label_registered
    }

    pub fn description_count(&self) -> usize {
        self.description_count
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }

    /// Returns whether any registered control is disabled.
    pub fn any_control_disabled(&self) -> bool {
        self.controls
            .iter()
            .any(|control| control.registration.disabled)
    }

    /// Returns whether any controls are currently registered.
    pub fn has_registered_controls(&self) -> bool {
        !self.controls.is_empty()
    }

    /// Returns whether any enabled registered control is required.
    pub fn required(&self) -> bool {
        self.controls
            .iter()
            .any(|control| control.registration.required && !control.registration.disabled)
    }

    /// Returns whether any enabled required control is currently missing a value.
    pub fn value_missing(&self) -> bool {
        self.controls.iter().any(|control| {
            control.registration.required
                && !control.registration.disabled
                && (control.registration.value_missing || !control.registration.value.filled())
        })
    }

    /// Returns whether any registered control is filled.
    pub fn filled(&self) -> bool {
        self.controls.iter().any(RegisteredControl::filled)
    }

    /// Returns whether any registered control has diverged from its initial value.
    pub fn dirty(&self) -> bool {
        self.controls.iter().any(RegisteredControl::dirty)
    }

    /// Returns the effective field name from root props or the representative registered control.
    pub fn effective_name(&self, props: &FieldProps) -> Option<SharedString> {
        props.name().cloned().or_else(|| {
            self.controls
                .iter()
                .find(|control| {
                    !control.registration.disabled && control.registration.name.is_some()
                })
                .or_else(|| {
                    self.controls
                        .iter()
                        .find(|control| control.registration.name.is_some())
                })
                .and_then(|control| control.registration.name.clone())
        })
    }

    /// Returns whether this field should be skipped by form validation and value collection.
    pub fn disabled_for_form(&self, props: &FieldProps) -> bool {
        props.disabled()
            || (!self.controls.is_empty()
                && self
                    .controls
                    .iter()
                    .all(|control| control.registration.disabled))
    }

    /// Returns the current representative field value.
    pub fn value(&self) -> FieldValue {
        self.controls
            .iter()
            .find(|control| !control.registration.disabled)
            .or_else(|| self.controls.first())
            .map(|control| control.registration.value.clone())
            .unwrap_or_default()
    }

    /// Returns the current representative initial value.
    pub fn initial_value(&self) -> FieldValue {
        self.controls
            .iter()
            .find(|control| !control.registration.disabled)
            .or_else(|| self.controls.first())
            .map(|control| control.initial_value.clone())
            .unwrap_or_default()
    }

    /// Returns a focus handle for the current field control, if one is registered.
    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.controls
            .iter()
            .find(|control| {
                !control.registration.disabled && control.registration.focus_handle.is_some()
            })
            .and_then(|control| control.registration.focus_handle.clone())
            .or_else(|| {
                self.controls
                    .iter()
                    .find_map(|control| control.registration.focus_handle.clone())
            })
    }

    /// Marks the field touched.
    pub fn mark_touched(&mut self) -> bool {
        if self.touched {
            return false;
        }

        self.touched = true;
        true
    }

    /// Runs the built-in required-only validation used when no custom callback is available.
    pub fn apply_required_only_validation(&mut self) -> bool {
        let previous = self.validity_data.clone();
        let value = self.value();
        let initial_value = self.initial_value();
        let state = if self.value_missing() {
            FieldValidityState::value_missing()
        } else {
            FieldValidityState::valid()
        };
        let error = if state.value_missing {
            SharedString::from("Required")
        } else {
            SharedString::default()
        };
        let errors = if error.is_empty() {
            Vec::new()
        } else {
            vec![error.clone()]
        };

        self.validity_data = FieldValidityData {
            state,
            error,
            errors,
            value,
            initial_value,
        };

        let changed = self.validity_data != previous;
        if changed {
            self.needs_refresh = true;
        }
        changed
    }

    /// Replaces form-level external error messages for this field.
    pub fn set_form_external_errors(&mut self, errors: Vec<SharedString>) -> bool {
        if self.form_external_errors == errors {
            return false;
        }

        self.form_external_errors = errors;
        self.needs_refresh = true;
        true
    }

    /// Commits externally produced validation data.
    pub fn set_validity_data(&mut self, validity_data: FieldValidityData) -> bool {
        if self.validity_data == validity_data {
            return false;
        }

        self.validity_data = validity_data;
        self.needs_refresh = true;
        true
    }

    /// Returns the root style state.
    pub fn root_state(&self, props: &FieldProps) -> FieldRootStyleState {
        let disabled = props.disabled();
        let dirty = props.dirty().unwrap_or_else(|| self.dirty());
        let touched = props.touched().unwrap_or(self.touched);
        let filled = self.filled();
        let focused = self
            .controls
            .iter()
            .any(|control| control.registration.focused);
        let valid = self.validity_data(props).state.valid;

        FieldRootStyleState::new(disabled, touched, dirty, valid, filled, focused)
    }

    /// Returns current validity data combined with externally controlled invalid state.
    pub fn validity_data(&self, props: &FieldProps) -> FieldValidityData {
        let mut data = self.validity_data.clone();
        data.value = self.value();
        data.initial_value = self.initial_value();

        if !props.disabled() && !self.form_external_errors.is_empty() {
            let mut errors = self.form_external_errors.clone();
            errors.extend(data.errors);
            data.state.custom_error = true;
            data.state.valid = Some(false);
            data.error = errors.first().cloned().unwrap_or_default();
            data.errors = errors;
        }

        if props.disabled() {
            data.state.valid = None;
        } else if props.invalid() == Some(true) {
            data.state.valid = Some(false);
        }

        data
    }

    /// Returns whether an error with the given matcher should be present.
    pub fn error_present(&self, props: &FieldProps, matcher: FieldErrorMatch) -> bool {
        if matcher == FieldErrorMatch::Always {
            return true;
        }

        if props.disabled() {
            return false;
        }

        match matcher {
            FieldErrorMatch::Default => self.root_state(props).invalid,
            FieldErrorMatch::Always => true,
            FieldErrorMatch::Key(key) => self.validity_data(props).state.flag(key),
        }
    }
}
