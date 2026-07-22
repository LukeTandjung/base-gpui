use gpui::SharedString;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OTPFieldRootStyleState {
    pub value: SharedString,
    pub length: usize,
    pub complete: bool,
    pub filled: bool,
    pub focused: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub dirty: bool,
    pub touched: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OTPFieldInputStyleState {
    /// This slot's character; empty when the slot is unfilled.
    pub value: SharedString,
    pub index: usize,
    pub filled: bool,
    pub active: bool,
    pub masked: bool,
    pub root: OTPFieldRootStyleState,
}
