use std::{rc::Rc, time::Duration};

use gpui::{App, SharedString, Window};

use crate::field::{FieldValidationMode, FieldValidationResult, FieldValue};

pub type FieldValidationHandler =
    Rc<dyn Fn(&FieldValue, &mut Window, &mut App) -> FieldValidationResult + 'static>;

#[derive(Clone)]
pub struct FieldProps {
    name: Option<SharedString>,
    disabled: bool,
    invalid: Option<bool>,
    dirty: Option<bool>,
    touched: Option<bool>,
    validation_mode: FieldValidationMode,
    validation_debounce: Option<Duration>,
    validate: Option<FieldValidationHandler>,
}

impl Default for FieldProps {
    fn default() -> Self {
        Self {
            name: None,
            disabled: false,
            invalid: None,
            dirty: None,
            touched: None,
            validation_mode: FieldValidationMode::OnSubmit,
            validation_debounce: None,
            validate: None,
        }
    }
}

impl FieldProps {
    pub fn new(
        name: Option<SharedString>,
        disabled: bool,
        invalid: Option<bool>,
        dirty: Option<bool>,
        touched: Option<bool>,
        validation_mode: FieldValidationMode,
        validation_debounce: Option<Duration>,
        validate: Option<FieldValidationHandler>,
    ) -> Self {
        Self {
            name,
            disabled,
            invalid,
            dirty,
            touched,
            validation_mode,
            validation_debounce,
            validate,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn invalid(&self) -> Option<bool> {
        self.invalid
    }

    pub fn dirty(&self) -> Option<bool> {
        self.dirty
    }

    pub fn touched(&self) -> Option<bool> {
        self.touched
    }

    pub fn validation_mode(&self) -> FieldValidationMode {
        self.validation_mode
    }

    pub fn validation_debounce(&self) -> Option<Duration> {
        self.validation_debounce
    }

    pub fn validate(&self) -> Option<&FieldValidationHandler> {
        self.validate.as_ref()
    }
}
