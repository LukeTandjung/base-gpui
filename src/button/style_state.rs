#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ButtonRootStyleState {
    pub disabled: bool,
    pub focused: bool,
}

impl ButtonRootStyleState {
    pub fn new(disabled: bool, focused: bool) -> Self {
        Self { disabled, focused }
    }
}
