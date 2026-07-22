#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SwitchRootStyleState {
    pub checked: bool,
    pub unchecked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub focused: bool,
}

impl SwitchRootStyleState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        focused: bool,
    ) -> Self {
        Self {
            checked,
            unchecked: !checked,
            disabled,
            read_only,
            required,
            focused,
        }
    }
}

impl Default for SwitchRootStyleState {
    fn default() -> Self {
        Self::new(false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SwitchThumbStyleState {
    pub root: SwitchRootStyleState,
}

impl SwitchThumbStyleState {
    pub fn new(root: SwitchRootStyleState) -> Self {
        Self { root }
    }
}

impl Default for SwitchThumbStyleState {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
