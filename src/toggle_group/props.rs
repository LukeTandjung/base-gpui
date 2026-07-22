use std::rc::Rc;

use gpui::{App, Window};

use crate::toggle_group::ToggleGroupValueChangeDetails;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleGroupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

pub type ToggleGroupValueChangeHandler<T> =
    Rc<dyn Fn(&[T], &mut ToggleGroupValueChangeDetails, &mut Window, &mut App) + 'static>;

pub struct ToggleGroupProps<T: Clone + Eq + 'static> {
    disabled: bool,
    orientation: ToggleGroupOrientation,
    multiple: bool,
    loop_focus: bool,
    on_value_change: Option<ToggleGroupValueChangeHandler<T>>,
}

impl<T: Clone + Eq + 'static> Clone for ToggleGroupProps<T> {
    fn clone(&self) -> Self {
        Self {
            disabled: self.disabled,
            orientation: self.orientation,
            multiple: self.multiple,
            loop_focus: self.loop_focus,
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> ToggleGroupProps<T> {
    pub fn new(
        disabled: bool,
        orientation: ToggleGroupOrientation,
        multiple: bool,
        loop_focus: bool,
        on_value_change: Option<ToggleGroupValueChangeHandler<T>>,
    ) -> Self {
        Self {
            disabled,
            orientation,
            multiple,
            loop_focus,
            on_value_change,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn orientation(&self) -> ToggleGroupOrientation {
        self.orientation
    }

    pub fn multiple(&self) -> bool {
        self.multiple
    }

    pub fn loop_focus(&self) -> bool {
        self.loop_focus
    }

    pub fn on_value_change(&self) -> Option<&ToggleGroupValueChangeHandler<T>> {
        self.on_value_change.as_ref()
    }
}
