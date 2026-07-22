#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupRootStyleState {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub filled: bool,
}

impl RadioGroupRootStyleState {
    pub fn new(
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
        filled: bool,
    ) -> Self {
        Self {
            disabled,
            read_only,
            required,
            focused,
            filled,
        }
    }
}

impl Default for RadioGroupRootStyleState {
    fn default() -> Self {
        Self::new(false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupRadioStyleState {
    pub checked: bool,
    pub unchecked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
    pub highlighted: bool,
    pub tab_stop: bool,
}

impl RadioGroupRadioStyleState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
        highlighted: bool,
        tab_stop: bool,
    ) -> Self {
        Self {
            checked,
            unchecked: !checked,
            disabled,
            read_only,
            required,
            focused,
            highlighted,
            tab_stop,
        }
    }
}

impl Default for RadioGroupRadioStyleState {
    fn default() -> Self {
        Self::new(false, false, false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadioGroupIndicatorStyleState {
    pub radio: RadioGroupRadioStyleState,
    pub present: bool,
}

impl RadioGroupIndicatorStyleState {
    pub fn new(radio: RadioGroupRadioStyleState, present: bool) -> Self {
        Self { radio, present }
    }
}

impl Default for RadioGroupIndicatorStyleState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}
