use std::rc::Rc;

use gpui::{App, Window};

use crate::toggle::TogglePressedChangeDetails;

pub type TogglePressedChangeHandler =
    Rc<dyn Fn(bool, &mut TogglePressedChangeDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone, Default)]
pub struct ToggleProps {
    disabled: bool,
    on_pressed_change: Option<TogglePressedChangeHandler>,
}

impl ToggleProps {
    pub fn new(disabled: bool, on_pressed_change: Option<TogglePressedChangeHandler>) -> Self {
        Self {
            disabled,
            on_pressed_change,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn on_pressed_change(&self) -> Option<&TogglePressedChangeHandler> {
        self.on_pressed_change.as_ref()
    }
}
