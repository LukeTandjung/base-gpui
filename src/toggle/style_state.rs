#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToggleStyleState {
    pub pressed: bool,
    pub unpressed: bool,
    pub disabled: bool,
    pub focused: bool,
}

impl ToggleStyleState {
    pub fn new(pressed: bool, disabled: bool, focused: bool) -> Self {
        Self {
            pressed,
            unpressed: !pressed,
            disabled,
            focused,
        }
    }
}

impl Default for ToggleStyleState {
    fn default() -> Self {
        Self::new(false, false, false)
    }
}
