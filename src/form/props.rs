use std::{collections::BTreeMap, rc::Rc};

use gpui::{App, SharedString, Window};

use crate::{
    field::FieldValidationMode,
    form::{FormSubmitDetails, FormValues},
};

pub type FormErrors = BTreeMap<SharedString, Vec<SharedString>>;

pub type FormSubmitHandler =
    Rc<dyn Fn(FormValues, FormSubmitDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone)]
pub struct FormProps {
    validation_mode: FieldValidationMode,
    errors: FormErrors,
    on_form_submit: Option<FormSubmitHandler>,
}

impl Default for FormProps {
    fn default() -> Self {
        Self {
            validation_mode: FieldValidationMode::OnSubmit,
            errors: FormErrors::new(),
            on_form_submit: None,
        }
    }
}

impl FormProps {
    pub fn new(
        validation_mode: FieldValidationMode,
        errors: FormErrors,
        on_form_submit: Option<FormSubmitHandler>,
    ) -> Self {
        Self {
            validation_mode,
            errors,
            on_form_submit,
        }
    }

    pub fn validation_mode(&self) -> FieldValidationMode {
        self.validation_mode
    }

    pub fn errors(&self) -> &FormErrors {
        &self.errors
    }

    pub fn on_form_submit(&self) -> Option<&FormSubmitHandler> {
        self.on_form_submit.as_ref()
    }
}
