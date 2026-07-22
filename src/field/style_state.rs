use crate::field::validation::FieldValidityData;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldRootStyleState {
    pub disabled: bool,
    pub touched: bool,
    pub dirty: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub filled: bool,
    pub focused: bool,
}

impl FieldRootStyleState {
    pub fn new(
        disabled: bool,
        touched: bool,
        dirty: bool,
        valid: Option<bool>,
        filled: bool,
        focused: bool,
    ) -> Self {
        Self {
            disabled,
            touched,
            dirty,
            valid,
            invalid: valid == Some(false),
            filled,
            focused,
        }
    }
}

impl Default for FieldRootStyleState {
    fn default() -> Self {
        Self::new(false, false, false, None, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldItemStyleState {
    pub root: FieldRootStyleState,
    pub disabled: bool,
}

impl FieldItemStyleState {
    pub fn new(root: FieldRootStyleState, disabled: bool) -> Self {
        Self { root, disabled }
    }
}

impl Default for FieldItemStyleState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldLabelStyleState {
    pub root: FieldRootStyleState,
    pub disabled: bool,
}

impl FieldLabelStyleState {
    pub fn new(root: FieldRootStyleState, disabled: bool) -> Self {
        Self { root, disabled }
    }
}

impl Default for FieldLabelStyleState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldDescriptionStyleState {
    pub root: FieldRootStyleState,
    pub disabled: bool,
}

impl FieldDescriptionStyleState {
    pub fn new(root: FieldRootStyleState, disabled: bool) -> Self {
        Self { root, disabled }
    }
}

impl Default for FieldDescriptionStyleState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldErrorStyleState {
    pub root: FieldRootStyleState,
    pub present: bool,
    pub errors: Vec<gpui::SharedString>,
    pub error: gpui::SharedString,
}

impl FieldErrorStyleState {
    pub fn new(
        root: FieldRootStyleState,
        present: bool,
        errors: Vec<gpui::SharedString>,
        error: gpui::SharedString,
    ) -> Self {
        Self {
            root,
            present,
            errors,
            error,
        }
    }
}

impl Default for FieldErrorStyleState {
    fn default() -> Self {
        Self::new(Default::default(), false, Vec::new(), Default::default())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldValidityStyleState {
    pub root: FieldRootStyleState,
    pub validity: FieldValidityData,
}

impl FieldValidityStyleState {
    pub fn new(root: FieldRootStyleState, validity: FieldValidityData) -> Self {
        Self { root, validity }
    }
}

impl Default for FieldValidityStyleState {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}
