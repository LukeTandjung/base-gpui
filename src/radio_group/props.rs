use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::radio_group::RadioGroupValueChangeDetails;

pub type RadioGroupValueChangeHandler<T> =
    Rc<dyn Fn(Option<&T>, &mut RadioGroupValueChangeDetails, &mut Window, &mut App) + 'static>;

pub struct RadioGroupProps<T: Clone + Eq + 'static> {
    name: Option<SharedString>,
    form: Option<SharedString>,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_value_change: Option<RadioGroupValueChangeHandler<T>>,
}

impl<T: Clone + Eq + 'static> Clone for RadioGroupProps<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            form: self.form.clone(),
            disabled: self.disabled,
            read_only: self.read_only,
            required: self.required,
            on_value_change: self.on_value_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> RadioGroupProps<T> {
    pub fn new(
        name: Option<SharedString>,
        form: Option<SharedString>,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_value_change: Option<RadioGroupValueChangeHandler<T>>,
    ) -> Self {
        Self {
            name,
            form,
            disabled,
            read_only,
            required,
            on_value_change,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn form(&self) -> Option<&SharedString> {
        self.form.as_ref()
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

    pub fn on_value_change(&self) -> Option<&RadioGroupValueChangeHandler<T>> {
        self.on_value_change.as_ref()
    }
}
