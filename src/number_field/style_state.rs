use gpui::SharedString;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldRootStyleState {
    pub value: Option<f64>,
    pub input_value: SharedString,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub scrubbing: bool,
    pub touched: bool,
    pub dirty: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub filled: bool,
    pub focused: bool,
    pub controlled: bool,
}

impl NumberFieldRootStyleState {
    pub fn new(
        value: Option<f64>,
        input_value: SharedString,
        disabled: bool,
        read_only: bool,
        required: bool,
        scrubbing: bool,
        touched: bool,
        dirty: bool,
        valid: Option<bool>,
        focused: bool,
        controlled: bool,
    ) -> Self {
        Self {
            value,
            input_value,
            disabled,
            read_only,
            required,
            scrubbing,
            touched,
            dirty,
            valid,
            invalid: valid == Some(false),
            filled: value.is_some(),
            focused,
            controlled,
        }
    }
}

impl Default for NumberFieldRootStyleState {
    fn default() -> Self {
        Self::new(
            None,
            SharedString::default(),
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            false,
            false,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldInputStyleState {
    pub root: NumberFieldRootStyleState,
}

impl NumberFieldInputStyleState {
    pub fn new(root: NumberFieldRootStyleState) -> Self {
        Self { root }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldGroupStyleState {
    pub root: NumberFieldRootStyleState,
}

impl NumberFieldGroupStyleState {
    pub fn new(root: NumberFieldRootStyleState) -> Self {
        Self { root }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldIncrementStyleState {
    pub root: NumberFieldRootStyleState,
    pub can_increment: bool,
}

impl NumberFieldIncrementStyleState {
    pub fn new(root: NumberFieldRootStyleState, can_increment: bool) -> Self {
        Self {
            root,
            can_increment,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldDecrementStyleState {
    pub root: NumberFieldRootStyleState,
    pub can_decrement: bool,
}

impl NumberFieldDecrementStyleState {
    pub fn new(root: NumberFieldRootStyleState, can_decrement: bool) -> Self {
        Self {
            root,
            can_decrement,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldScrubAreaStyleState {
    pub root: NumberFieldRootStyleState,
    pub horizontal: bool,
    pub vertical: bool,
}

impl NumberFieldScrubAreaStyleState {
    pub fn new(root: NumberFieldRootStyleState, horizontal: bool, vertical: bool) -> Self {
        Self {
            root,
            horizontal,
            vertical,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldScrubAreaCursorStyleState {
    pub root: NumberFieldRootStyleState,
    pub present: bool,
}

impl NumberFieldScrubAreaCursorStyleState {
    pub fn new(root: NumberFieldRootStyleState, present: bool) -> Self {
        Self { root, present }
    }
}
