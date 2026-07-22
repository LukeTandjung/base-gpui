use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::otp_field::{
    OTPFieldChangeDetails, OTPFieldNormalizeValueHandler, OTPFieldValidationType,
};

pub type OTPFieldValueChangeHandler =
    Rc<dyn Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static>;
pub type OTPFieldValueCompleteHandler =
    Rc<dyn Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static>;
pub type OTPFieldValueInvalidHandler =
    Rc<dyn Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone)]
pub struct OTPFieldProps {
    name: Option<SharedString>,
    aria_label: Option<SharedString>,
    length: usize,
    validation_type: OTPFieldValidationType,
    normalize_value: Option<OTPFieldNormalizeValueHandler>,
    mask: bool,
    auto_submit: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_value_change: Option<OTPFieldValueChangeHandler>,
    on_value_complete: Option<OTPFieldValueCompleteHandler>,
    on_value_invalid: Option<OTPFieldValueInvalidHandler>,
}

impl OTPFieldProps {
    pub fn new(
        name: Option<SharedString>,
        length: usize,
        validation_type: OTPFieldValidationType,
        normalize_value: Option<OTPFieldNormalizeValueHandler>,
        mask: bool,
        auto_submit: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_value_change: Option<OTPFieldValueChangeHandler>,
        on_value_complete: Option<OTPFieldValueCompleteHandler>,
        on_value_invalid: Option<OTPFieldValueInvalidHandler>,
    ) -> Self {
        Self {
            name,
            aria_label: None,
            length,
            validation_type,
            normalize_value,
            mask,
            auto_submit,
            disabled,
            read_only,
            required,
            on_value_change,
            on_value_complete,
            on_value_invalid,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    /// Literal accessible name for the OTP group. Stand-in for Base UI's
    /// `aria-labelledby` id-wiring to `FieldLabel` (no relationship builders
    /// exist in this gpui revision).
    pub fn with_aria_label(mut self, aria_label: Option<SharedString>) -> Self {
        self.aria_label = aria_label;
        self
    }

    pub fn aria_label(&self) -> Option<&SharedString> {
        self.aria_label.as_ref()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn validation_type(&self) -> OTPFieldValidationType {
        self.validation_type
    }

    pub fn normalize_value(&self) -> Option<&OTPFieldNormalizeValueHandler> {
        self.normalize_value.as_ref()
    }

    pub fn mask(&self) -> bool {
        self.mask
    }

    pub fn auto_submit(&self) -> bool {
        self.auto_submit
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn on_value_change(&self) -> Option<&OTPFieldValueChangeHandler> {
        self.on_value_change.as_ref()
    }

    pub fn on_value_complete(&self) -> Option<&OTPFieldValueCompleteHandler> {
        self.on_value_complete.as_ref()
    }

    pub fn on_value_invalid(&self) -> Option<&OTPFieldValueInvalidHandler> {
        self.on_value_invalid.as_ref()
    }
}
