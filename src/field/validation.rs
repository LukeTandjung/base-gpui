use gpui::SharedString;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub enum FieldValue {
    #[default]
    Empty,
    Present,
    Bool(bool),
    Text(SharedString),
    List(Vec<SharedString>),
}

impl FieldValue {
    pub fn filled(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Present => true,
            Self::Bool(value) => *value,
            Self::Text(value) => !value.is_empty(),
            Self::List(value) => !value.is_empty(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldValidationMode {
    #[default]
    OnSubmit,
    OnBlur,
    OnChange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldErrorMatch {
    #[default]
    Default,
    Always,
    Key(FieldValidityKey),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldValidityKey {
    BadInput,
    CustomError,
    PatternMismatch,
    RangeOverflow,
    RangeUnderflow,
    StepMismatch,
    TooLong,
    TooShort,
    TypeMismatch,
    ValueMissing,
    Valid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct FieldValidityState {
    pub bad_input: bool,
    pub custom_error: bool,
    pub pattern_mismatch: bool,
    pub range_overflow: bool,
    pub range_underflow: bool,
    pub step_mismatch: bool,
    pub too_long: bool,
    pub too_short: bool,
    pub type_mismatch: bool,
    pub value_missing: bool,
    pub valid: Option<bool>,
}

impl FieldValidityState {
    pub fn valid() -> Self {
        Self {
            valid: Some(true),
            ..Self::default()
        }
    }

    pub fn custom_error() -> Self {
        Self {
            custom_error: true,
            valid: Some(false),
            ..Self::default()
        }
    }

    pub fn value_missing() -> Self {
        Self {
            value_missing: true,
            valid: Some(false),
            ..Self::default()
        }
    }

    pub fn flag(&self, key: FieldValidityKey) -> bool {
        match key {
            FieldValidityKey::BadInput => self.bad_input,
            FieldValidityKey::CustomError => self.custom_error,
            FieldValidityKey::PatternMismatch => self.pattern_mismatch,
            FieldValidityKey::RangeOverflow => self.range_overflow,
            FieldValidityKey::RangeUnderflow => self.range_underflow,
            FieldValidityKey::StepMismatch => self.step_mismatch,
            FieldValidityKey::TooLong => self.too_long,
            FieldValidityKey::TooShort => self.too_short,
            FieldValidityKey::TypeMismatch => self.type_mismatch,
            FieldValidityKey::ValueMissing => self.value_missing,
            FieldValidityKey::Valid => self.valid == Some(true),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldValidityData {
    pub state: FieldValidityState,
    pub error: SharedString,
    pub errors: Vec<SharedString>,
    pub value: FieldValue,
    pub initial_value: FieldValue,
}

impl Default for FieldValidityData {
    fn default() -> Self {
        Self {
            state: FieldValidityState::default(),
            error: SharedString::default(),
            errors: Vec::new(),
            value: FieldValue::Empty,
            initial_value: FieldValue::Empty,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldValidationResult {
    Valid,
    Error(SharedString),
    Errors(Vec<SharedString>),
    Validity(FieldValidityData),
}

impl FieldValidationResult {
    pub fn into_validity_data(
        self,
        value: FieldValue,
        initial_value: FieldValue,
    ) -> FieldValidityData {
        match self {
            Self::Valid => FieldValidityData {
                state: FieldValidityState::valid(),
                error: SharedString::default(),
                errors: Vec::new(),
                value,
                initial_value,
            },
            Self::Error(error) => FieldValidityData {
                state: FieldValidityState::custom_error(),
                error: error.clone(),
                errors: vec![error],
                value,
                initial_value,
            },
            Self::Errors(errors) => FieldValidityData {
                state: FieldValidityState::custom_error(),
                error: errors.first().cloned().unwrap_or_default(),
                errors,
                value,
                initial_value,
            },
            Self::Validity(mut data) => {
                data.value = value;
                data.initial_value = initial_value;
                data
            }
        }
    }
}
