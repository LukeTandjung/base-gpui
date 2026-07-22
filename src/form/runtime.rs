use std::{collections::BTreeSet, rc::Rc};

use gpui::{App, FocusHandle, SharedString, Window};

use crate::form::{FormErrors, FormStyleState, FormValue, FormValues};

pub type FormFieldValidationHandler =
    Rc<dyn Fn(&mut Window, &mut App) -> FormFieldSnapshot + 'static>;

#[derive(Clone, Debug)]
pub struct FormFieldSnapshot {
    key: SharedString,
    name: Option<SharedString>,
    value: FormValue,
    disabled: bool,
    valid: Option<bool>,
    focus_handle: Option<FocusHandle>,
}

impl FormFieldSnapshot {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            name: None,
            value: FormValue::Empty,
            disabled: false,
            valid: None,
            focus_handle: None,
        }
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn maybe_name(mut self, name: Option<SharedString>) -> Self {
        self.name = name;
        self
    }

    pub fn value(mut self, value: impl Into<FormValue>) -> Self {
        self.value = value.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn valid(mut self, valid: Option<bool>) -> Self {
        self.valid = valid;
        self
    }

    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn maybe_focus_handle(mut self, focus_handle: Option<FocusHandle>) -> Self {
        self.focus_handle = focus_handle;
        self
    }

    pub fn key(&self) -> &SharedString {
        &self.key
    }

    pub fn name_value(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn value_ref(&self) -> &FormValue {
        &self.value
    }

    pub fn disabled_value(&self) -> bool {
        self.disabled
    }

    pub fn valid_value(&self) -> Option<bool> {
        self.valid
    }

    pub fn focus_handle_ref(&self) -> Option<&FocusHandle> {
        self.focus_handle.as_ref()
    }
}

impl PartialEq for FormFieldSnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.name == other.name
            && self.value == other.value
            && self.disabled == other.disabled
            && self.valid == other.valid
            && self.focus_handle == other.focus_handle
    }
}

#[derive(Clone)]
pub struct FormFieldRegistration {
    snapshot: FormFieldSnapshot,
    validate: Option<FormFieldValidationHandler>,
}

impl FormFieldRegistration {
    pub fn new(snapshot: FormFieldSnapshot) -> Self {
        Self {
            snapshot,
            validate: None,
        }
    }

    pub fn validate_with(mut self, validate: FormFieldValidationHandler) -> Self {
        self.validate = Some(validate);
        self
    }

    pub fn snapshot(&self) -> &FormFieldSnapshot {
        &self.snapshot
    }

    pub fn validate(&self) -> Option<&FormFieldValidationHandler> {
        self.validate.as_ref()
    }

    fn update_snapshot(&mut self, snapshot: FormFieldSnapshot) -> bool {
        if self.snapshot == snapshot {
            return false;
        }

        self.snapshot = snapshot;
        true
    }
}

impl core::fmt::Debug for FormFieldRegistration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FormFieldRegistration")
            .field("snapshot", &self.snapshot)
            .field("validate", &self.validate.is_some())
            .finish()
    }
}

impl PartialEq for FormFieldRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.snapshot == other.snapshot
    }
}

#[derive(Clone, Debug)]
struct RegisteredFormField {
    registration: FormFieldRegistration,
    generation: u64,
    order: usize,
}

impl RegisteredFormField {
    fn new(registration: FormFieldRegistration, generation: u64, order: usize) -> Self {
        Self {
            registration,
            generation,
            order,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FormSubmissionResult {
    pub valid: bool,
    pub values: FormValues,
    pub first_invalid_focus: Option<FocusHandle>,
}

#[derive(Clone, Debug, Default)]
pub struct FormRuntime {
    fields: Vec<RegisteredFormField>,
    generation: u64,
    next_order: usize,
    submit_attempted: bool,
    external_errors: FormErrors,
    cleared_external_error_names: BTreeSet<SharedString>,
}

impl FormRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    /// Synchronizes the externally controlled error map for this form render.
    pub fn sync_external_errors(&mut self, errors: &FormErrors) -> bool {
        if &self.external_errors == errors {
            return false;
        }

        self.external_errors = errors.clone();
        self.cleared_external_error_names.clear();
        true
    }

    /// Returns active external errors for an effective field name.
    pub fn external_errors_for(&self, name: Option<&SharedString>) -> Vec<SharedString> {
        let Some(name) = name else {
            return Vec::new();
        };
        if self.cleared_external_error_names.contains(name) {
            return Vec::new();
        }

        self.external_errors.get(name).cloned().unwrap_or_default()
    }

    /// Clears active external errors for one changed field name.
    pub fn clear_external_error(&mut self, name: &SharedString) -> bool {
        if !self.external_errors.contains_key(name) {
            return false;
        }

        self.cleared_external_error_names.insert(name.clone())
    }

    /// Starts one descendant field registration pass for the form subtree.
    pub fn begin_registration_pass(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.next_order = 0;
    }

    /// Registers or refreshes one mounted field for the current pass.
    pub fn register_field(&mut self, registration: FormFieldRegistration) -> bool {
        let order = self.next_order;
        self.next_order += 1;

        if let Some(field) = self
            .fields
            .iter_mut()
            .find(|field| field.registration.snapshot().key() == registration.snapshot().key())
        {
            let changed = field.registration != registration || field.order != order;
            field.registration = registration;
            field.generation = self.generation;
            field.order = order;
            return changed;
        }

        self.fields.push(RegisteredFormField::new(
            registration,
            self.generation,
            order,
        ));
        true
    }

    /// Finishes descendant registration, prunes unmounted fields, and restores render order.
    pub fn finish_registration_pass(&mut self) -> bool {
        let previous_len = self.fields.len();
        self.fields
            .retain(|field| field.generation == self.generation);
        self.fields.sort_by_key(|field| field.order);

        previous_len != self.fields.len()
    }

    /// Replaces field snapshots after submit-time validation reruns.
    pub fn refresh_snapshots(&mut self, snapshots: Vec<FormFieldSnapshot>) -> bool {
        let mut changed = false;

        for snapshot in snapshots {
            if let Some(field) = self
                .fields
                .iter_mut()
                .find(|field| field.registration.snapshot().key() == snapshot.key())
            {
                changed |= field.registration.update_snapshot(snapshot);
            }
        }

        changed
    }

    /// Marks that the form has attempted at least one submit.
    pub fn mark_submit_attempted(&mut self) -> bool {
        if self.submit_attempted {
            return false;
        }

        self.submit_attempted = true;
        true
    }

    /// Returns whether submit has been attempted for validation-mode behavior.
    pub fn submit_attempted(&self) -> bool {
        self.submit_attempted
    }

    /// Returns field validators in current render order.
    pub fn validation_handlers(&self) -> Vec<FormFieldValidationHandler> {
        self.fields
            .iter()
            .filter(|field| !field.registration.snapshot().disabled_value())
            .filter_map(|field| field.registration.validate().cloned())
            .collect()
    }

    /// Returns field validators matching the given effective field name.
    pub fn validation_handlers_for_name(
        &self,
        name: &SharedString,
    ) -> Vec<FormFieldValidationHandler> {
        self.fields
            .iter()
            .filter(|field| !field.registration.snapshot().disabled_value())
            .filter(|field| field.registration.snapshot().name_value() == Some(name))
            .filter_map(|field| field.registration.validate().cloned())
            .collect()
    }

    /// Computes the current submit result from registered field snapshots.
    pub fn submission_result(&self) -> FormSubmissionResult {
        let first_invalid_focus = self
            .fields
            .iter()
            .filter(|field| !field.registration.snapshot().disabled_value())
            .filter(|field| field.registration.snapshot().valid_value() == Some(false))
            .find_map(|field| field.registration.snapshot().focus_handle_ref().cloned());
        let valid = !self
            .fields
            .iter()
            .filter(|field| !field.registration.snapshot().disabled_value())
            .any(|field| field.registration.snapshot().valid_value() == Some(false));
        let mut values = FormValues::new();

        for field in self
            .fields
            .iter()
            .filter(|field| !field.registration.snapshot().disabled_value())
        {
            if let Some(name) = field.registration.snapshot().name_value() {
                values.insert(
                    name.clone(),
                    field.registration.snapshot().value_ref().clone(),
                );
            }
        }

        FormSubmissionResult {
            valid,
            values,
            first_invalid_focus,
        }
    }

    /// Returns the form style state.
    pub fn root_state(&self) -> FormStyleState {
        FormStyleState
    }

    pub fn registered_field_count(&self) -> usize {
        self.fields.len()
    }
}
