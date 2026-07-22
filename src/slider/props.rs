use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::slider::{
    format_slider_value, SliderOrientation, SliderThumbAlignment, SliderThumbCollisionBehavior,
    SliderValueChangeDetails, SliderValueCommitDetails, SliderValues,
};

pub type SliderValueChangeHandler =
    Rc<dyn Fn(SliderValues, &mut SliderValueChangeDetails, &mut Window, &mut App) + 'static>;
pub type SliderValueCommitHandler =
    Rc<dyn Fn(SliderValues, SliderValueCommitDetails, &mut Window, &mut App) + 'static>;
pub type SliderFormatHandler = Rc<dyn Fn(f64) -> SharedString + 'static>;

#[derive(Clone)]
pub struct SliderProps {
    name: Option<SharedString>,
    min: f64,
    max: f64,
    step: f64,
    large_step: f64,
    min_steps_between_values: f64,
    orientation: SliderOrientation,
    thumb_collision_behavior: SliderThumbCollisionBehavior,
    thumb_alignment: SliderThumbAlignment,
    disabled: bool,
    format: Option<SliderFormatHandler>,
    on_value_change: Option<SliderValueChangeHandler>,
    on_value_committed: Option<SliderValueCommitHandler>,
}

impl SliderProps {
    pub fn new(
        name: Option<SharedString>,
        min: f64,
        max: f64,
        step: f64,
        large_step: f64,
        min_steps_between_values: f64,
        orientation: SliderOrientation,
        thumb_collision_behavior: SliderThumbCollisionBehavior,
        thumb_alignment: SliderThumbAlignment,
        disabled: bool,
        format: Option<SliderFormatHandler>,
        on_value_change: Option<SliderValueChangeHandler>,
        on_value_committed: Option<SliderValueCommitHandler>,
    ) -> Self {
        let min = if min.is_finite() { min } else { 0.0 };
        let max = if max.is_finite() { max } else { 100.0 };
        if min >= max {
            #[cfg(debug_assertions)]
            eprintln!(
                "base_gpui slider: min ({min}) should be lower than max ({max}); the slider will \
                 render but interactions clamp degenerately"
            );
        }

        Self {
            name,
            min,
            max,
            step: positive_or_default(step, 1.0),
            large_step: positive_or_default(large_step, 10.0),
            min_steps_between_values: if min_steps_between_values.is_finite()
                && min_steps_between_values > 0.0
            {
                min_steps_between_values
            } else {
                0.0
            },
            orientation,
            thumb_collision_behavior,
            thumb_alignment,
            disabled,
            format,
            on_value_change,
            on_value_committed,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn large_step(&self) -> f64 {
        self.large_step
    }

    pub fn min_steps_between_values(&self) -> f64 {
        self.min_steps_between_values
    }

    pub fn orientation(&self) -> SliderOrientation {
        self.orientation
    }

    pub fn thumb_collision_behavior(&self) -> SliderThumbCollisionBehavior {
        self.thumb_collision_behavior
    }

    pub fn thumb_alignment(&self) -> SliderThumbAlignment {
        self.thumb_alignment
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn format_value(&self, value: f64) -> SharedString {
        match self.format.as_ref() {
            Some(format) => format(value),
            None => format_slider_value(value),
        }
    }

    pub fn on_value_change(&self) -> Option<&SliderValueChangeHandler> {
        self.on_value_change.as_ref()
    }

    pub fn on_value_committed(&self) -> Option<&SliderValueCommitHandler> {
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
