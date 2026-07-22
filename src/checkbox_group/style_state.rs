#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxGroupStyleState {
    pub disabled: bool,
    pub touched: bool,
    pub dirty: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub filled: bool,
    pub focused: bool,
}

impl CheckboxGroupStyleState {
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

impl Default for CheckboxGroupStyleState {
    fn default() -> Self {
        Self::new(false, false, false, None, false, false)
    }
}
