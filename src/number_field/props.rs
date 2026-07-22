use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::number_field::{
    normalize_optional_value, NumberFieldChangeDetails, NumberFieldCommitDetails, NumberFieldStep,
};

pub type NumberFieldValueChangeHandler =
    Rc<dyn Fn(Option<f64>, NumberFieldChangeDetails, &mut Window, &mut App) + 'static>;
pub type NumberFieldValueCommitHandler =
    Rc<dyn Fn(Option<f64>, NumberFieldCommitDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone)]
pub struct NumberFieldProps {
    name: Option<SharedString>,
    form: Option<SharedString>,
    min: Option<f64>,
    max: Option<f64>,
    step: NumberFieldStep,
    small_step: f64,
    large_step: f64,
    snap_on_step: bool,
    allow_out_of_range: bool,
    allow_wheel_scrub: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_value_change: Option<NumberFieldValueChangeHandler>,
    on_value_committed: Option<NumberFieldValueCommitHandler>,
}

impl NumberFieldProps {
    pub fn new(
        name: Option<SharedString>,
        form: Option<SharedString>,
        min: Option<f64>,
        max: Option<f64>,
        step: NumberFieldStep,
        small_step: f64,
        large_step: f64,
        snap_on_step: bool,
        allow_out_of_range: bool,
        allow_wheel_scrub: bool,
        disabled: bool,
        read_only: bool,
        required: bool,
        on_value_change: Option<NumberFieldValueChangeHandler>,
        on_value_committed: Option<NumberFieldValueCommitHandler>,
    ) -> Self {
        Self {
            name,
            form,
            min: normalize_optional_value(min),
            max: normalize_optional_value(max),
            step,
            small_step: positive_or_default(small_step, 0.1),
            large_step: positive_or_default(large_step, 10.0),
            snap_on_step,
            allow_out_of_range,
            allow_wheel_scrub,
            disabled,
            read_only,
            required,
            on_value_change,
            on_value_committed,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn form(&self) -> Option<&SharedString> {
        self.form.as_ref()
    }

    pub fn min(&self) -> Option<f64> {
        self.min
    }

    pub fn max(&self) -> Option<f64> {
        self.max
    }

    pub fn step(&self) -> NumberFieldStep {
        self.step
    }

    pub fn step_amount(&self) -> f64 {
        self.step.interactive_amount()
    }

    pub fn small_step(&self) -> f64 {
        self.small_step
    }

    pub fn large_step(&self) -> f64 {
        self.large_step
    }

    pub fn snap_on_step(&self) -> bool {
        self.snap_on_step
    }

    pub fn allow_out_of_range(&self) -> bool {
        self.allow_out_of_range
    }

    pub fn allow_wheel_scrub(&self) -> bool {
        self.allow_wheel_scrub
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

    pub fn on_value_change(&self) -> Option<&NumberFieldValueChangeHandler> {
        self.on_value_change.as_ref()
    }

    pub fn on_value_committed(&self) -> Option<&NumberFieldValueCommitHandler> {
        self.on_value_committed.as_ref()
    }
}

fn positive_or_default(value: f64, default: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        default
    }
}
