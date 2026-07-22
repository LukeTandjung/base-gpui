use gpui::SharedString;

use crate::number_field::{
    clamp_value, format_number, normalize_optional_value, option_values_equal, parse_number,
    step_value, NumberFieldDecrementStyleState, NumberFieldGroupStyleState,
    NumberFieldIncrementStyleState, NumberFieldInputStyleState, NumberFieldProps,
    NumberFieldRootStyleState, NumberFieldScrubAreaCursorStyleState,
    NumberFieldScrubAreaStyleState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFieldChangeReason {
    InputChange,
    InputClear,
    InputBlur,
    Keyboard,
    IncrementPress,
    DecrementPress,
    Wheel,
    Scrub,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFieldCommitReason {
    InputBlur,
    InputClear,
    Keyboard,
    IncrementPress,
    DecrementPress,
    Wheel,
    Scrub,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFieldStepAmount {
    Normal,
    Small,
    Large,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFieldStepDirection {
    Up,
    Down,
}

impl NumberFieldStepDirection {
    pub fn multiplier(self) -> i8 {
        match self {
            Self::Up => 1,
            Self::Down => -1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFieldScrubDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NumberFieldChangeDetails {
    pub reason: NumberFieldChangeReason,
    pub direction: Option<NumberFieldStepDirection>,
}

impl NumberFieldChangeDetails {
    pub fn new(reason: NumberFieldChangeReason) -> Self {
        Self {
            reason,
            direction: None,
        }
    }

    pub fn with_direction(
        reason: NumberFieldChangeReason,
        direction: NumberFieldStepDirection,
    ) -> Self {
        Self {
            reason,
            direction: Some(direction),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NumberFieldCommitDetails {
    pub reason: NumberFieldCommitReason,
    pub direction: Option<NumberFieldStepDirection>,
}

impl NumberFieldCommitDetails {
    pub fn new(reason: NumberFieldCommitReason) -> Self {
        Self {
            reason,
            direction: None,
        }
    }

    pub fn with_direction(
        reason: NumberFieldCommitReason,
        direction: NumberFieldStepDirection,
    ) -> Self {
        Self {
            reason,
            direction: Some(direction),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldValueChange {
    pub value: Option<f64>,
    pub details: NumberFieldChangeDetails,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFieldValueCommit {
    pub value: Option<f64>,
    pub details: NumberFieldCommitDetails,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NumberFieldUpdateOutcome {
    pub change: Option<NumberFieldValueChange>,
    pub commit: Option<NumberFieldValueCommit>,
}

impl NumberFieldUpdateOutcome {
    fn changed(value: Option<f64>, details: NumberFieldChangeDetails) -> Self {
        Self {
            change: Some(NumberFieldValueChange { value, details }),
            commit: None,
        }
    }

    fn committed(value: Option<f64>, details: NumberFieldCommitDetails) -> Self {
        Self {
            change: None,
            commit: Some(NumberFieldValueCommit { value, details }),
        }
    }

    fn merge(mut self, other: Self) -> Self {
        if other.change.is_some() {
            self.change = other.change;
        }
        if other.commit.is_some() {
            self.commit = other.commit;
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct NumberFieldRuntime {
    value: Option<f64>,
    initial_value: Option<f64>,
    input_value: SharedString,
    focused: bool,
    touched: bool,
    scrubbing: bool,
    pending_commit: bool,
    controlled: bool,
    scrub_remainder: f64,
}

impl Default for NumberFieldRuntime {
    fn default() -> Self {
        Self::new(None)
    }
}

impl NumberFieldRuntime {
    /// Creates the runtime from the initial committed numeric value.
    pub fn new(value: Option<f64>) -> Self {
        let value = normalize_optional_value(value);
        Self {
            value,
            initial_value: value,
            input_value: format_number(value),
            focused: false,
            touched: false,
            scrubbing: false,
            pending_commit: false,
            controlled: false,
            scrub_remainder: 0.0,
        }
    }

    /// Reconciles the runtime with the value observed from controlled props.
    pub fn reconcile(&mut self, observed_value: Option<f64>, controlled: bool) {
        self.controlled = controlled;
        if !controlled {
            return;
        }

        let observed_value = normalize_optional_value(observed_value);
        if option_values_equal(self.value, observed_value) {
            return;
        }

        self.value = observed_value;
        if !self.focused {
            self.input_value = format_number(observed_value);
        }
    }

    /// Records focus changes and commits pending text when focus leaves the input.
    pub fn sync_focused(
        &mut self,
        focused: bool,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if self.focused == focused {
            return NumberFieldUpdateOutcome::default();
        }

        self.focused = focused;
        if focused {
            return NumberFieldUpdateOutcome::default();
        }

        self.touched = true;
        self.commit_input(NumberFieldCommitReason::InputBlur, props)
    }

    /// Applies direct text entry from the generic input primitive.
    pub fn input_changed(
        &mut self,
        text: SharedString,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if props.disabled() || props.read_only() {
            return NumberFieldUpdateOutcome::default();
        }

        self.input_value = text.clone();
        match parse_number(text.as_ref()) {
            Ok(None) => self.accept_value(
                None,
                NumberFieldChangeDetails::new(NumberFieldChangeReason::InputClear),
                !self.controlled,
            ),
            Ok(Some(parsed)) => {
                let next = if props.allow_out_of_range() {
                    parsed
                } else {
                    clamp_value(parsed, props.min(), props.max())
                };
                self.accept_value(
                    Some(next),
                    NumberFieldChangeDetails::new(NumberFieldChangeReason::InputChange),
                    !self.controlled,
                )
            }
            Err(_) => NumberFieldUpdateOutcome::default(),
        }
    }

    /// Parses, clamps, and formats the current text as a committed value.
    pub fn commit_input(
        &mut self,
        reason: NumberFieldCommitReason,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if props.disabled() || props.read_only() {
            return NumberFieldUpdateOutcome::default();
        }

        let previous_text = self.input_value.clone();
        let parsed = parse_number(previous_text.as_ref());
        let commit_reason = match parsed {
            Ok(None) => NumberFieldCommitReason::InputClear,
            _ => reason,
        };
        let change_reason = match commit_reason {
            NumberFieldCommitReason::InputClear => NumberFieldChangeReason::InputClear,
            NumberFieldCommitReason::InputBlur => NumberFieldChangeReason::InputBlur,
            NumberFieldCommitReason::Keyboard => NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::IncrementPress => NumberFieldChangeReason::IncrementPress,
            NumberFieldCommitReason::DecrementPress => NumberFieldChangeReason::DecrementPress,
            NumberFieldCommitReason::Wheel => NumberFieldChangeReason::Wheel,
            NumberFieldCommitReason::Scrub => NumberFieldChangeReason::Scrub,
            NumberFieldCommitReason::None => NumberFieldChangeReason::None,
        };

        let target = match parsed {
            Ok(None) => None,
            Ok(Some(parsed)) => {
                let committed = if props.allow_out_of_range() {
                    parsed
                } else {
                    clamp_value(parsed, props.min(), props.max())
                };
                Some(committed)
            }
            Err(_) => self.value,
        };

        self.input_value = format_number(target);
        let change = self.accept_value(
            target,
            NumberFieldChangeDetails::new(change_reason),
            !self.controlled,
        );
        let should_commit =
            self.pending_commit || previous_text != self.input_value || change.change.is_some();
        self.pending_commit = false;

        if should_commit {
            change.merge(NumberFieldUpdateOutcome::committed(
                target,
                NumberFieldCommitDetails::new(commit_reason),
            ))
        } else {
            change
        }
    }

    /// Steps the committed value by the requested amount and commits the result.
    pub fn step_by(
        &mut self,
        direction: NumberFieldStepDirection,
        amount: NumberFieldStepAmount,
        change_reason: NumberFieldChangeReason,
        commit_reason: NumberFieldCommitReason,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if props.disabled() || props.read_only() {
            return NumberFieldUpdateOutcome::default();
        }

        let amount = match amount {
            NumberFieldStepAmount::Normal => props.step_amount(),
            NumberFieldStepAmount::Small => props.small_step(),
            NumberFieldStepAmount::Large => props.large_step(),
        };
        let next = match self.value {
            Some(value) => step_value(
                Some(value),
                direction.multiplier(),
                amount,
                props.min(),
                props.max(),
                props.snap_on_step(),
            ),
            None => clamp_value(0.0, props.min(), props.max()),
        };
        self.input_value = format_number(Some(next));
        self.pending_commit = false;

        let change = NumberFieldChangeDetails::with_direction(change_reason, direction);
        let commit = NumberFieldCommitDetails::with_direction(commit_reason, direction);
        let outcome = self.accept_value(Some(next), change, !self.controlled);
        if outcome.change.is_some() {
            outcome.merge(NumberFieldUpdateOutcome::committed(Some(next), commit))
        } else {
            NumberFieldUpdateOutcome::default()
        }
    }

    /// Moves directly to the configured minimum or maximum boundary.
    pub fn move_to_boundary(
        &mut self,
        direction: NumberFieldStepDirection,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if props.disabled() || props.read_only() {
            return NumberFieldUpdateOutcome::default();
        }

        let target = match direction {
            NumberFieldStepDirection::Down => props.min(),
            NumberFieldStepDirection::Up => props.max(),
        };
        let Some(target) = target else {
            return NumberFieldUpdateOutcome::default();
        };

        self.input_value = format_number(Some(target));
        let change =
            NumberFieldChangeDetails::with_direction(NumberFieldChangeReason::Keyboard, direction);
        let commit =
            NumberFieldCommitDetails::with_direction(NumberFieldCommitReason::Keyboard, direction);
        let outcome = self.accept_value(Some(target), change, !self.controlled);
        if outcome.change.is_some() {
            outcome.merge(NumberFieldUpdateOutcome::committed(Some(target), commit))
        } else {
            NumberFieldUpdateOutcome::default()
        }
    }

    /// Marks scrubbing active or inactive and commits when scrubbing stops.
    pub fn set_scrubbing(
        &mut self,
        scrubbing: bool,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if props.disabled() || props.read_only() {
            return NumberFieldUpdateOutcome::default();
        }

        if self.scrubbing == scrubbing {
            return NumberFieldUpdateOutcome::default();
        }

        self.scrubbing = scrubbing;
        self.scrub_remainder = 0.0;
        if scrubbing {
            NumberFieldUpdateOutcome::default()
        } else {
            NumberFieldUpdateOutcome::committed(
                self.value,
                NumberFieldCommitDetails::new(NumberFieldCommitReason::Scrub),
            )
        }
    }

    /// Converts scrub movement into step commands once the pixel threshold is reached.
    pub fn scrub_by_pixels(
        &mut self,
        pixels: f64,
        pixel_sensitivity: f64,
        props: &NumberFieldProps,
    ) -> NumberFieldUpdateOutcome {
        if !self.scrubbing || props.disabled() || props.read_only() {
            return NumberFieldUpdateOutcome::default();
        }

        let sensitivity = if pixel_sensitivity.is_finite() && pixel_sensitivity > 0.0 {
            pixel_sensitivity
        } else {
            2.0
        };
        self.scrub_remainder += pixels;
        let steps = (self.scrub_remainder / sensitivity).trunc();
        if steps == 0.0 {
            return NumberFieldUpdateOutcome::default();
        }

        self.scrub_remainder -= steps * sensitivity;
        let direction = if steps > 0.0 {
            NumberFieldStepDirection::Up
        } else {
            NumberFieldStepDirection::Down
        };
        let amount = props.step_amount() * steps.abs();
        let next = match self.value {
            Some(value) => step_value(
                Some(value),
                direction.multiplier(),
                amount,
                props.min(),
                props.max(),
                props.snap_on_step(),
            ),
            None => clamp_value(0.0, props.min(), props.max()),
        };
        self.input_value = format_number(Some(next));

        self.accept_value(
            Some(next),
            NumberFieldChangeDetails::with_direction(NumberFieldChangeReason::Scrub, direction),
            !self.controlled,
        )
    }

    /// Returns the committed numeric value.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Returns the visible input text.
    pub fn input_value(&self) -> SharedString {
        self.input_value.clone()
    }

    /// Returns whether the increment stepper can currently change the value.
    pub fn can_increment(&self, props: &NumberFieldProps) -> bool {
        if props.disabled() || props.read_only() {
            return false;
        }

        match (self.value, props.max()) {
            (Some(value), Some(max)) => value < max,
            _ => true,
        }
    }

    /// Returns whether the decrement stepper can currently change the value.
    pub fn can_decrement(&self, props: &NumberFieldProps) -> bool {
        if props.disabled() || props.read_only() {
            return false;
        }

        match (self.value, props.min()) {
            (Some(value), Some(min)) => value > min,
            _ => true,
        }
    }

    pub fn root_state(&self, props: &NumberFieldProps) -> NumberFieldRootStyleState {
        let valid = if props.disabled() {
            None
        } else if props.required() && self.value.is_none() && self.touched {
            Some(false)
        } else {
            Some(true)
        };

        NumberFieldRootStyleState::new(
            self.value,
            self.input_value.clone(),
            props.disabled(),
            props.read_only(),
            props.required(),
            self.scrubbing,
            self.touched,
            self.dirty(),
            valid,
            self.focused,
            self.controlled,
        )
    }

    pub fn input_state(&self, props: &NumberFieldProps) -> NumberFieldInputStyleState {
        NumberFieldInputStyleState::new(self.root_state(props))
    }

    pub fn group_state(&self, props: &NumberFieldProps) -> NumberFieldGroupStyleState {
        NumberFieldGroupStyleState::new(self.root_state(props))
    }

    pub fn increment_state(&self, props: &NumberFieldProps) -> NumberFieldIncrementStyleState {
        NumberFieldIncrementStyleState::new(self.root_state(props), self.can_increment(props))
    }

    pub fn decrement_state(&self, props: &NumberFieldProps) -> NumberFieldDecrementStyleState {
        NumberFieldDecrementStyleState::new(self.root_state(props), self.can_decrement(props))
    }

    pub fn scrub_area_state(
        &self,
        props: &NumberFieldProps,
        direction: NumberFieldScrubDirection,
    ) -> NumberFieldScrubAreaStyleState {
        NumberFieldScrubAreaStyleState::new(
            self.root_state(props),
            direction == NumberFieldScrubDirection::Horizontal,
            direction == NumberFieldScrubDirection::Vertical,
        )
    }

    pub fn scrub_cursor_state(
        &self,
        props: &NumberFieldProps,
    ) -> NumberFieldScrubAreaCursorStyleState {
        NumberFieldScrubAreaCursorStyleState::new(self.root_state(props), self.scrubbing)
    }

    fn dirty(&self) -> bool {
        !option_values_equal(self.value, self.initial_value)
    }

    fn accept_value(
        &mut self,
        next: Option<f64>,
        details: NumberFieldChangeDetails,
        mutate_value: bool,
    ) -> NumberFieldUpdateOutcome {
        let next = normalize_optional_value(next);
        let changed = !option_values_equal(self.value, next);
        if changed && mutate_value {
            self.value = next;
        }
        if changed {
            self.pending_commit = true;
            NumberFieldUpdateOutcome::changed(next, details)
        } else {
            NumberFieldUpdateOutcome::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::SharedString;

    use crate::number_field::{
        NumberFieldChangeReason, NumberFieldCommitReason, NumberFieldProps, NumberFieldRuntime,
        NumberFieldStep, NumberFieldStepAmount, NumberFieldStepDirection,
    };

    fn props() -> NumberFieldProps {
        NumberFieldProps::new(
            None,
            None,
            None,
            None,
            NumberFieldStep::Amount(1.0),
            0.1,
            10.0,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
        )
    }

    #[test]
    fn default_value_is_empty() {
        let runtime = NumberFieldRuntime::default();

        assert_eq!(runtime.value(), None);
        assert_eq!(runtime.input_value(), SharedString::default());
    }

    #[test]
    fn initial_value_formats_input_text() {
        let runtime = NumberFieldRuntime::new(Some(12.5));

        assert_eq!(runtime.value(), Some(12.5));
        assert_eq!(runtime.input_value(), SharedString::from("12.5"));
    }

    #[test]
    fn parseable_input_updates_uncontrolled_value() {
        let mut runtime = NumberFieldRuntime::default();
        let outcome = runtime.input_changed(SharedString::from("42"), &props());

        assert_eq!(runtime.value(), Some(42.0));
        assert_eq!(outcome.change.expect("change").value, Some(42.0));
    }

    #[test]
    fn parseable_decimal_input_updates_uncontrolled_value() {
        let mut runtime = NumberFieldRuntime::default();
        let outcome = runtime.input_changed(SharedString::from("4.25"), &props());

        assert_eq!(runtime.value(), Some(4.25));
        assert_eq!(outcome.change.expect("change").value, Some(4.25));
    }

    #[test]
    fn controlled_reconcile_updates_displayed_text() {
        let mut runtime = NumberFieldRuntime::default();

        runtime.reconcile(Some(8.5), true);
        assert_eq!(runtime.value(), Some(8.5));
        assert_eq!(runtime.input_value(), SharedString::from("8.5"));

        runtime.reconcile(None, true);
        assert_eq!(runtime.value(), None);
        assert_eq!(runtime.input_value(), SharedString::default());
    }

    #[test]
    fn controlled_input_does_not_mutate_numeric_source_of_truth() {
        let mut runtime = NumberFieldRuntime::new(Some(1.0));
        runtime.reconcile(Some(1.0), true);
        let outcome = runtime.input_changed(SharedString::from("2"), &props());

        assert_eq!(runtime.value(), Some(1.0));
        assert_eq!(runtime.input_value(), SharedString::from("2"));
        assert_eq!(outcome.change.expect("change").value, Some(2.0));
    }

    #[test]
    fn empty_input_clears_value() {
        let mut runtime = NumberFieldRuntime::new(Some(1.0));
        let outcome = runtime.input_changed(SharedString::from(""), &props());

        assert_eq!(runtime.value(), None);
        assert_eq!(
            outcome.change.expect("change").details.reason,
            NumberFieldChangeReason::InputClear
        );
    }

    #[test]
    fn invalid_intermediate_text_remains_visible_until_blur() {
        let mut runtime = NumberFieldRuntime::new(Some(3.0));
        let props = props();

        let outcome = runtime.input_changed(SharedString::from("-"), &props);

        assert_eq!(runtime.value(), Some(3.0));
        assert_eq!(runtime.input_value(), SharedString::from("-"));
        assert!(outcome.change.is_none());

        runtime.sync_focused(true, &props);
        runtime.sync_focused(false, &props);

        assert_eq!(runtime.input_value(), SharedString::from("3"));
    }

    #[test]
    fn blur_clamps_when_out_of_range_is_not_allowed() {
        let mut runtime = NumberFieldRuntime::default();
        let props = NumberFieldProps::new(
            None,
            None,
            Some(0.0),
            Some(10.0),
            NumberFieldStep::Amount(1.0),
            0.1,
            10.0,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
        );

        runtime.input_changed(SharedString::from("12"), &props);
        runtime.commit_input(NumberFieldCommitReason::InputBlur, &props);

        assert_eq!(runtime.value(), Some(10.0));
        assert_eq!(runtime.input_value(), SharedString::from("10"));
    }

    #[test]
    fn direct_input_can_remain_out_of_range_when_allowed() {
        let mut runtime = NumberFieldRuntime::default();
        let props = NumberFieldProps::new(
            None,
            None,
            Some(0.0),
            Some(10.0),
            NumberFieldStep::Amount(1.0),
            0.1,
            10.0,
            false,
            true,
            false,
            false,
            false,
            false,
            None,
            None,
        );

        runtime.input_changed(SharedString::from("12"), &props);

        assert_eq!(runtime.value(), Some(12.0));
        assert_eq!(runtime.input_value(), SharedString::from("12"));
    }

    #[test]
    fn step_uses_configured_amounts() {
        let mut runtime = NumberFieldRuntime::new(Some(1.0));
        let props = props();

        runtime.step_by(
            NumberFieldStepDirection::Up,
            NumberFieldStepAmount::Normal,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &props,
        );

        assert_eq!(runtime.value(), Some(2.0));
    }

    #[test]
    fn small_and_large_step_amounts_are_supported() {
        let mut runtime = NumberFieldRuntime::new(Some(1.0));
        let props = props();

        runtime.step_by(
            NumberFieldStepDirection::Up,
            NumberFieldStepAmount::Small,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &props,
        );
        assert_eq!(runtime.value(), Some(1.1));

        runtime.step_by(
            NumberFieldStepDirection::Down,
            NumberFieldStepAmount::Large,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &props,
        );
        assert_eq!(runtime.value(), Some(-8.9));
    }

    #[test]
    fn step_interactions_clamp_to_min_and_max() {
        let mut runtime = NumberFieldRuntime::new(Some(9.0));
        let props = NumberFieldProps::new(
            None,
            None,
            Some(0.0),
            Some(10.0),
            NumberFieldStep::Amount(5.0),
            0.1,
            10.0,
            false,
            true,
            false,
            false,
            false,
            false,
            None,
            None,
        );

        runtime.step_by(
            NumberFieldStepDirection::Up,
            NumberFieldStepAmount::Normal,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &props,
        );
        assert_eq!(runtime.value(), Some(10.0));

        runtime.step_by(
            NumberFieldStepDirection::Down,
            NumberFieldStepAmount::Large,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &props,
        );
        assert_eq!(runtime.value(), Some(0.0));
    }

    #[test]
    fn disabled_and_read_only_ignore_step_commands() {
        let mut runtime = NumberFieldRuntime::new(Some(1.0));
        let disabled_props = NumberFieldProps::new(
            None,
            None,
            None,
            None,
            NumberFieldStep::Amount(1.0),
            0.1,
            10.0,
            false,
            false,
            false,
            true,
            false,
            false,
            None,
            None,
        );
        runtime.step_by(
            NumberFieldStepDirection::Up,
            NumberFieldStepAmount::Normal,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &disabled_props,
        );
        assert_eq!(runtime.value(), Some(1.0));

        let read_only_props = NumberFieldProps::new(
            None,
            None,
            None,
            None,
            NumberFieldStep::Amount(1.0),
            0.1,
            10.0,
            false,
            false,
            false,
            false,
            true,
            false,
            None,
            None,
        );
        runtime.step_by(
            NumberFieldStepDirection::Up,
            NumberFieldStepAmount::Normal,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &read_only_props,
        );
        assert_eq!(runtime.value(), Some(1.0));
    }

    #[test]
    fn snap_on_step_snaps_to_grid() {
        let mut runtime = NumberFieldRuntime::new(Some(0.25));
        let props = NumberFieldProps::new(
            None,
            None,
            Some(0.0),
            None,
            NumberFieldStep::Amount(0.1),
            0.1,
            10.0,
            true,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
        );

        runtime.step_by(
            NumberFieldStepDirection::Up,
            NumberFieldStepAmount::Normal,
            NumberFieldChangeReason::Keyboard,
            NumberFieldCommitReason::Keyboard,
            &props,
        );

        assert_eq!(runtime.value(), Some(0.4));
    }
}
