use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::checkbox::CheckboxCheckedChangeDetails;

pub type CheckboxCheckedChangeHandler =
    Rc<dyn Fn(bool, &mut CheckboxCheckedChangeDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone, Default)]
pub struct CheckboxProps {
    name: Option<SharedString>,
    value: Option<SharedString>,
    form: Option<SharedString>,
    parent: bool,
    unchecked_value: Option<SharedString>,
    indeterminate: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_checked_change: Option<CheckboxCheckedChangeHandler>,
}

impl CheckboxProps {
    pub fn new(
        name: Option<SharedString>,
        value: Option<SharedString>,
        form: Option<SharedString>,
        parent: bool,
        unchecked_value: Option<SharedString>,
        indeterminate: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_checked_change: Option<CheckboxCheckedChangeHandler>,
    ) -> Self {
        Self {
            name,
            value,
            form,
            parent,
            unchecked_value,
            indeterminate,
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

    pub fn parent(&self) -> bool {
        self.parent
    }

    pub fn unchecked_value(&self) -> Option<&SharedString> {
        self.unchecked_value.as_ref()
    }

    pub fn indeterminate(&self) -> bool {
        self.indeterminate
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

    pub fn on_checked_change(&self) -> Option<&CheckboxCheckedChangeHandler> {
        self.on_checked_change.as_ref()
    }
}
