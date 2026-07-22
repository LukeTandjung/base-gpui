use gpui::SharedString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputStyleState {
    pub value: SharedString,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub empty: bool,
    pub filled: bool,
    pub dirty: bool,
    pub controlled: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
}

impl InputStyleState {
    pub fn new(
        value: SharedString,
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
        dirty: bool,
        controlled: bool,
        valid: Option<bool>,
    ) -> Self {
        let empty = value.is_empty();

        Self {
            value,
            disabled,
            read_only,
            required,
            focused,
            empty,
            filled: !empty,
            dirty,
            controlled,
            valid,
            invalid: valid == Some(false),
        }
    }
}

impl Default for InputStyleState {
    fn default() -> Self {
        Self::new(
            SharedString::default(),
            false,
            false,
            false,
            false,
            false,
            false,
            None,
        )
    }
}
