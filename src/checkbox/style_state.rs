#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxRootStyleState {
    pub checked: bool,
    pub unchecked: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub indeterminate: bool,
    pub focused: bool,
}

impl CheckboxRootStyleState {
    pub fn new(
        checked: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        indeterminate: bool,
        focused: bool,
    ) -> Self {
        Self {
            checked,
            unchecked: !checked && !indeterminate,
            disabled,
            read_only,
            required,
            indeterminate,
            focused,
        }
    }
}

impl Default for CheckboxRootStyleState {
    fn default() -> Self {
        Self::new(false, false, false, false, false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckboxIndicatorStyleState {
    pub root: CheckboxRootStyleState,
    pub present: bool,
}

impl CheckboxIndicatorStyleState {
    pub fn new(root: CheckboxRootStyleState, keep_mounted: bool) -> Self {
        Self {
            present: keep_mounted || root.checked || root.indeterminate,
            root,
        }
    }
}
