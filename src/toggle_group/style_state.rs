use crate::toggle_group::ToggleGroupOrientation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToggleGroupStyleState {
    pub disabled: bool,
    pub orientation: ToggleGroupOrientation,
    pub multiple: bool,
}

impl ToggleGroupStyleState {
    pub fn new(disabled: bool, orientation: ToggleGroupOrientation, multiple: bool) -> Self {
        Self {
            disabled,
            orientation,
            multiple,
        }
    }
}

impl Default for ToggleGroupStyleState {
    fn default() -> Self {
        Self::new(false, ToggleGroupOrientation::Horizontal, false)
    }
}
