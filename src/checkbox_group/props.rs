use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::checkbox_group::CheckboxGroupValueChangeDetails;

pub type CheckboxGroupValueChangeHandler = Rc<
    dyn Fn(Vec<SharedString>, &mut CheckboxGroupValueChangeDetails, &mut Window, &mut App)
        + 'static,
>;

#[derive(Clone, Default)]
pub struct CheckboxGroupProps {
    disabled: bool,
    all_values: Vec<SharedString>,
    on_value_change: Option<CheckboxGroupValueChangeHandler>,
}

impl CheckboxGroupProps {
    pub fn new(
        disabled: bool,
        all_values: Vec<SharedString>,
        on_value_change: Option<CheckboxGroupValueChangeHandler>,
    ) -> Self {
        Self {
            disabled,
            all_values,
            on_value_change,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn all_values(&self) -> &[SharedString] {
        &self.all_values
    }

    pub fn on_value_change(&self) -> Option<&CheckboxGroupValueChangeHandler> {
        self.on_value_change.as_ref()
    }
}
