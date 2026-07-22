use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::switch::SwitchCheckedChangeDetails;

pub type SwitchCheckedChangeHandler =
    Rc<dyn Fn(bool, &mut SwitchCheckedChangeDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone, Default)]
pub struct SwitchProps {
    name: Option<SharedString>,
    value: Option<SharedString>,
    form: Option<SharedString>,
    unchecked_value: Option<SharedString>,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_checked_change: Option<SwitchCheckedChangeHandler>,
}

impl SwitchProps {
    pub fn new(
        name: Option<SharedString>,
        value: Option<SharedString>,
        form: Option<SharedString>,
        unchecked_value: Option<SharedString>,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_checked_change: Option<SwitchCheckedChangeHandler>,
    ) -> Self {
        Self {
            name,
            value,
            form,
            unchecked_value,
            disabled,
            read_only,
            required,
            on_checked_change,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn value(&self) -> Option<&SharedString> {
        self.value.as_ref()
    }

    pub fn form(&self) -> Option<&SharedString> {
        self.form.as_ref()
    }

    pub fn unchecked_value(&self) -> Option<&SharedString> {
        self.unchecked_value.as_ref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn on_checked_change(&self) -> Option<&SwitchCheckedChangeHandler> {
        self.on_checked_change.as_ref()
    }
}
