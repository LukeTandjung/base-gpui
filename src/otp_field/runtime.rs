use gpui::{Bounds, Pixels, SharedString};

use crate::otp_field::{
    code_point_count, normalize_otp_value, remove_at_index, replace_at_index,
    OTPFieldInputStyleState, OTPFieldProps, OTPFieldRootStyleState,
};

/// Why an OTP value changed, completed, or was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OTPFieldChangeReason {
    InputChange,
    InputClear,
    InputPaste,
    Keyboard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OTPFieldChangeDetails {
    pub reason: OTPFieldChangeReason,
}

impl OTPFieldChangeDetails {
    pub fn new(reason: OTPFieldChangeReason) -> Self {
        Self { reason }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OTPFieldValueChange {
    pub value: SharedString,
    pub details: OTPFieldChangeDetails,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OTPFieldUpdateOutcome {
    pub change: Option<OTPFieldValueChange>,
    pub complete: Option<OTPFieldValueChange>,
    /// The raw attempted value that lost characters to normalization.
    pub invalid: Option<OTPFieldValueChange>,
}

/// Which slot the keyboard navigation targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OTPFieldMove {
    Previous,
    Next,
    First,
    /// The slot after the last filled character, clamped to the last slot.
    EndOfValue,
}

/// The single deep module owning the OTP value, the virtual active slot, and
/// all editing/navigation rules. Plain `&mut self` methods; unit-testable
/// without a window.
#[derive(Clone, Debug)]
pub struct OTPFieldRuntime {
    value: String,
    initial_value: String,
    /// Mirror of the text the platform input bridge currently holds, used to
    /// derive typed fragments from full-text edit notifications.
    bridge_value: String,
    active_index: usize,
    focused: bool,
    touched: bool,
    controlled: bool,
    slot_bounds: Vec<(usize, Bounds<Pixels>)>,
}

impl Default for OTPFieldRuntime {
    fn default() -> Self {
        Self::new("")
    }
}

impl OTPFieldRuntime {
    /// Creates the runtime from an already-normalized initial value.
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            initial_value: value.to_string(),
            bridge_value: value.to_string(),
            active_index: 0,
            focused: false,
            touched: false,
            controlled: false,
            slot_bounds: Vec::new(),
        }
    }

    /// Reconciles with the (already normalized) controlled value and re-arms
    /// the input-bridge mirror for the value that will be synced this render.
    pub fn reconcile(&mut self, observed: Option<String>, controlled: bool, props: &OTPFieldProps) {
        self.controlled = controlled;
        if controlled {
            if let Some(observed) = observed {
                if self.value != observed {
                    self.value = observed;
                }
            }
        }
        self.bridge_value = self.value.clone();
        self.clamp_active_index(props);
    }

    /// Records focus transitions; blurring marks the field touched.
    pub fn sync_focused(&mut self, focused: bool, props: &OTPFieldProps) {
        if self.focused == focused {
            return;
        }
        self.focused = focused;
        if !focused {
            self.touched = true;
        }
        self.clamp_active_index(props);
    }

    /// Pointer activation of a slot: clamps to the end-of-value slot.
    pub fn activate_slot(&mut self, index: usize, props: &OTPFieldProps) {
        if props.disabled() || props.length() == 0 {
            return;
        }
        self.active_index = index.min(self.end_of_value_index(props));
    }

    /// Applies a full-text edit observed from the platform input bridge:
    /// derives the typed fragment against the bridge mirror and distributes it.
    pub fn bridge_edited(
        &mut self,
        new_text: &str,
        props: &OTPFieldProps,
    ) -> OTPFieldUpdateOutcome {
        let previous = std::mem::replace(&mut self.bridge_value, new_text.to_string());
        if new_text == previous {
            return OTPFieldUpdateOutcome::default();
        }
        if new_text.is_empty() && !previous.is_empty() {
            return self.apply_value(
                String::new(),
                OTPFieldChangeReason::InputClear,
                false,
                props,
            );
        }

        let common = previous
            .char_indices()
            .zip(new_text.char_indices())
            .take_while(|((_, a), (_, b))| a == b)
            .count();
        let typed: String = new_text.chars().skip(common).collect();
        if typed.is_empty() {
            return OTPFieldUpdateOutcome::default();
        }
        self.insert_text(&typed, OTPFieldChangeReason::InputChange, props)
    }

    /// Distributes typed (or IME-committed) text forward from the active slot.
    pub fn insert_text(
        &mut self,
        text: &str,
        reason: OTPFieldChangeReason,
        props: &OTPFieldProps,
    ) -> OTPFieldUpdateOutcome {
        if props.disabled() || props.read_only() || props.length() == 0 {
            return OTPFieldUpdateOutcome::default();
        }

        let active = self.active_index.min(props.length().saturating_sub(1));

        // Same-character overtype: typing the character a filled active slot
        // already contains advances without changing the value.
        let normalized_fragment = normalize_otp_value(
            text,
            props.validation_type(),
            props.normalize_value(),
            props.length(),
        );
        if code_point_count(&normalized_fragment.value) == 1
            && self.value.chars().nth(active) == normalized_fragment.value.chars().next()
        {
            self.active_index = (active + 1).min(self.end_of_value_index(props));
            return OTPFieldUpdateOutcome::default();
        }

        let replaced = replace_at_index(
            &self.value,
            active,
            text,
            props.validation_type(),
            props.normalize_value(),
            props.length(),
        );

        let mut outcome = OTPFieldUpdateOutcome::default();
        if replaced.rejected {
            outcome.invalid = Some(OTPFieldValueChange {
                value: SharedString::from(text.to_string()),
                details: OTPFieldChangeDetails::new(reason),
            });
        }
        if replaced.accepted == 0 {
            return outcome;
        }

        self.active_index = (active + replaced.accepted).min(props.length().saturating_sub(1));
        let applied = self.apply_value(
            replaced.value,
            reason,
            reason == OTPFieldChangeReason::InputPaste,
            props,
        );
        outcome.change = applied.change;
        outcome.complete = applied.complete;
        outcome
    }

    /// Distributes pasted clipboard text forward from the active slot.
    pub fn paste_text(&mut self, text: &str, props: &OTPFieldProps) -> OTPFieldUpdateOutcome {
        self.insert_text(text, OTPFieldChangeReason::InputPaste, props)
    }

    /// Moves the active slot per the keyboard contract.
    pub fn move_active(&mut self, movement: OTPFieldMove, props: &OTPFieldProps) {
        if props.disabled() || props.length() == 0 {
            return;
        }
        self.active_index = match movement {
            OTPFieldMove::Previous => self.active_index.saturating_sub(1),
            OTPFieldMove::Next => (self.active_index + 1).min(self.end_of_value_index(props)),
            OTPFieldMove::First => 0,
            OTPFieldMove::EndOfValue => self.end_of_value_index(props),
        };
    }

    /// Backspace: removes the active slot's character when filled, otherwise
    /// the previous slot's character; moves the active slot back one.
    pub fn backspace(&mut self, props: &OTPFieldProps) -> OTPFieldUpdateOutcome {
        if props.disabled() || props.read_only() || props.length() == 0 {
            return OTPFieldUpdateOutcome::default();
        }

        let count = code_point_count(&self.value);
        let active = self.active_index;
        let next = if active < count {
            remove_at_index(&self.value, active)
        } else if active > 0 {
            remove_at_index(&self.value, active - 1)
        } else {
            return OTPFieldUpdateOutcome::default();
        };
        self.active_index = active.saturating_sub(1);
        self.apply_value(next, OTPFieldChangeReason::Keyboard, false, props)
    }

    /// Delete: removes the character at the active slot; the active slot stays.
    pub fn delete(&mut self, props: &OTPFieldProps) -> OTPFieldUpdateOutcome {
        if props.disabled() || props.read_only() || props.length() == 0 {
            return OTPFieldUpdateOutcome::default();
        }
        let next = remove_at_index(&self.value, self.active_index);
        self.apply_value(next, OTPFieldChangeReason::Keyboard, false, props)
    }

    /// Clears the entire value and moves the active slot to the first slot.
    pub fn clear_all(&mut self, props: &OTPFieldProps) -> OTPFieldUpdateOutcome {
        if props.disabled() || props.read_only() {
            return OTPFieldUpdateOutcome::default();
        }
        self.active_index = 0;
        self.apply_value(String::new(), OTPFieldChangeReason::Keyboard, false, props)
    }

    /// Records the measured bounds of a slot; returns whether anything changed.
    pub fn set_slot_bounds(&mut self, index: usize, bounds: Bounds<Pixels>) -> bool {
        match self.slot_bounds.iter_mut().find(|(i, _)| *i == index) {
            Some((_, existing)) if *existing == bounds => false,
            Some((_, existing)) => {
                *existing = bounds;
                true
            }
            None => {
                self.slot_bounds.push((index, bounds));
                true
            }
        }
    }

    /// The current OTP value.
    pub fn value(&self) -> SharedString {
        SharedString::from(self.value.clone())
    }

    pub fn root_state(&self, props: &OTPFieldProps, valid: Option<bool>) -> OTPFieldRootStyleState {
        let count = code_point_count(&self.value);
        OTPFieldRootStyleState {
            value: self.value(),
            length: props.length(),
            complete: props.length() > 0 && count == props.length(),
            filled: count > 0,
            focused: self.focused,
            disabled: props.disabled(),
            read_only: props.read_only(),
            required: props.required(),
            dirty: self.value != self.initial_value,
            touched: self.touched,
            valid,
            invalid: valid == Some(false),
        }
    }

    pub fn input_state(
        &self,
        index: usize,
        props: &OTPFieldProps,
        valid: Option<bool>,
    ) -> OTPFieldInputStyleState {
        let root = self.root_state(props, valid);
        let character = self.value.chars().nth(index);
        let active_index = if self.focused {
            self.active_index.min(props.length().saturating_sub(1))
        } else {
            self.end_of_value_index(props)
        };
        OTPFieldInputStyleState {
            value: character
                .map(|c| SharedString::from(c.to_string()))
                .unwrap_or_default(),
            index,
            filled: character.is_some(),
            active: props.length() > 0 && index == active_index,
            masked: props.mask(),
            root,
        }
    }

    fn end_of_value_index(&self, props: &OTPFieldProps) -> usize {
        code_point_count(&self.value).min(props.length().saturating_sub(1))
    }

    fn clamp_active_index(&mut self, props: &OTPFieldProps) {
        self.active_index = self.active_index.min(props.length().saturating_sub(1));
    }

    /// The single value-transition point: applies (uncontrolled) or reports
    /// (controlled) a normalized next value, deriving change and complete
    /// outcomes. `complete_on_equal` re-fires completion when a paste re-produces
    /// an already-complete value.
    fn apply_value(
        &mut self,
        next: String,
        reason: OTPFieldChangeReason,
        complete_on_equal: bool,
        props: &OTPFieldProps,
    ) -> OTPFieldUpdateOutcome {
        let previous_count = code_point_count(&self.value);
        let was_complete = props.length() > 0 && previous_count == props.length();
        let next_count = code_point_count(&next);
        let becomes_complete = props.length() > 0 && next_count == props.length();
        let changed = self.value != next;

        if changed && !self.controlled {
            self.value = next.clone();
            self.bridge_value = next.clone();
        }

        let mut outcome = OTPFieldUpdateOutcome::default();
        let details = OTPFieldChangeDetails::new(reason);
        if changed {
            outcome.change = Some(OTPFieldValueChange {
                value: SharedString::from(next.clone()),
                details,
            });
        }
        if becomes_complete && (!was_complete || (complete_on_equal && !changed)) {
            outcome.complete = Some(OTPFieldValueChange {
                value: SharedString::from(next),
                details,
            });
        }
        outcome
    }
}

#[cfg(test)]
mod tests {
    use crate::otp_field::{
        OTPFieldChangeReason, OTPFieldMove, OTPFieldProps, OTPFieldRuntime, OTPFieldValidationType,
    };

    fn props(length: usize) -> OTPFieldProps {
        OTPFieldProps::new(
            None,
            length,
            OTPFieldValidationType::Numeric,
            None,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
        )
    }

    fn read_only_props(length: usize) -> OTPFieldProps {
        OTPFieldProps::new(
            None,
            length,
            OTPFieldValidationType::Numeric,
            None,
            false,
            false,
            false,
            true,
            false,
            None,
            None,
            None,
        )
    }

    #[test]
    fn typing_distributes_forward_and_advances_active_slot() {
        let mut runtime = OTPFieldRuntime::default();
        let props = props(4);
        runtime.sync_focused(true, &props);

        let outcome = runtime.insert_text("12", OTPFieldChangeReason::InputChange, &props);

        assert_eq!(runtime.value().as_ref(), "12");
        assert!(outcome.change.is_some());
        assert!(runtime.input_state(2, &props, None).active);
    }

    #[test]
    fn same_character_overtype_advances_without_change() {
        let mut runtime = OTPFieldRuntime::new("12");
        let props = props(4);
        runtime.sync_focused(true, &props);
        runtime.activate_slot(0, &props);

        let outcome = runtime.insert_text("1", OTPFieldChangeReason::InputChange, &props);

        assert!(outcome.change.is_none());
        assert_eq!(runtime.value().as_ref(), "12");
        assert!(runtime.input_state(1, &props, None).active);
    }

    #[test]
    fn fully_rejected_text_fires_invalid_and_leaves_value_unchanged() {
        let mut runtime = OTPFieldRuntime::new("1");
        let props = props(4);

        let outcome = runtime.insert_text("x", OTPFieldChangeReason::InputChange, &props);

        assert!(outcome.change.is_none());
        assert!(outcome.invalid.is_some());
        assert_eq!(runtime.value().as_ref(), "1");
    }

    #[test]
    fn paste_clamps_reports_rejections_and_completes() {
        let mut runtime = OTPFieldRuntime::default();
        let props = props(4);

        let outcome = runtime.paste_text("1 2x345", &props);

        assert_eq!(runtime.value().as_ref(), "1234");
        assert!(outcome.invalid.is_some());
        assert!(outcome.complete.is_some());
    }

    #[test]
    fn complete_paste_over_identical_complete_value_fires_complete_without_change() {
        let mut runtime = OTPFieldRuntime::new("1234");
        let props = props(4);
        runtime.activate_slot(0, &props);

        let outcome = runtime.paste_text("1234", &props);

        assert!(outcome.change.is_none());
        assert!(outcome.complete.is_some());
    }

    #[test]
    fn backspace_on_filled_and_empty_slots_matches_contract() {
        let mut runtime = OTPFieldRuntime::new("123");
        let props = props(4);
        runtime.sync_focused(true, &props);
        runtime.activate_slot(1, &props);

        runtime.backspace(&props);
        assert_eq!(runtime.value().as_ref(), "13");
        assert!(runtime.input_state(0, &props, None).active);

        // Active slot beyond the value: removes the previous character.
        let mut runtime = OTPFieldRuntime::new("12");
        runtime.sync_focused(true, &props);
        runtime.activate_slot(2, &props);
        runtime.backspace(&props);
        assert_eq!(runtime.value().as_ref(), "1");
        assert!(runtime.input_state(1, &props, None).active);
    }

    #[test]
    fn clear_all_empties_value_and_returns_to_first_slot() {
        let mut runtime = OTPFieldRuntime::new("123");
        let props = props(4);

        let outcome = runtime.clear_all(&props);

        assert_eq!(runtime.value().as_ref(), "");
        assert_eq!(
            outcome.change.expect("change").details.reason,
            OTPFieldChangeReason::Keyboard
        );
        assert!(
            runtime.input_state(0, &props, None).active
                || !runtime.root_state(&props, None).focused
        );
    }

    #[test]
    fn delete_removes_at_active_and_keeps_index() {
        let mut runtime = OTPFieldRuntime::new("123");
        let props = props(4);
        runtime.sync_focused(true, &props);
        runtime.activate_slot(1, &props);

        runtime.delete(&props);

        assert_eq!(runtime.value().as_ref(), "13");
        assert!(runtime.input_state(1, &props, None).active);
    }

    #[test]
    fn navigation_clamps_to_end_of_value_and_first_slot() {
        let mut runtime = OTPFieldRuntime::new("12");
        let props = props(4);
        runtime.sync_focused(true, &props);
        runtime.activate_slot(0, &props);

        runtime.move_active(OTPFieldMove::Next, &props);
        runtime.move_active(OTPFieldMove::Next, &props);
        runtime.move_active(OTPFieldMove::Next, &props);
        assert!(runtime.input_state(2, &props, None).active);

        runtime.move_active(OTPFieldMove::First, &props);
        assert!(runtime.input_state(0, &props, None).active);

        runtime.move_active(OTPFieldMove::Previous, &props);
        assert!(runtime.input_state(0, &props, None).active);

        runtime.move_active(OTPFieldMove::EndOfValue, &props);
        assert!(runtime.input_state(2, &props, None).active);
    }

    #[test]
    fn unfocused_styling_active_slot_is_end_of_value() {
        let runtime = OTPFieldRuntime::new("12");
        let props = props(4);

        assert!(runtime.input_state(2, &props, None).active);
        assert!(!runtime.input_state(0, &props, None).active);
    }

    #[test]
    fn read_only_ignores_edits_but_allows_navigation() {
        let mut runtime = OTPFieldRuntime::new("12");
        let props = read_only_props(4);
        runtime.sync_focused(true, &props);

        let outcome = runtime.insert_text("3", OTPFieldChangeReason::InputChange, &props);
        assert!(outcome.change.is_none());
        assert_eq!(runtime.value().as_ref(), "12");

        runtime.backspace(&props);
        assert_eq!(runtime.value().as_ref(), "12");

        runtime.move_active(OTPFieldMove::First, &props);
        assert!(runtime.input_state(0, &props, None).active);
    }

    #[test]
    fn bridge_edit_derives_typed_suffix_and_clear() {
        let mut runtime = OTPFieldRuntime::new("12");
        let props = props(4);
        runtime.sync_focused(true, &props);
        runtime.move_active(OTPFieldMove::EndOfValue, &props);

        let outcome = runtime.bridge_edited("123", &props);
        assert_eq!(runtime.value().as_ref(), "123");
        assert!(outcome.change.is_some());

        let outcome = runtime.bridge_edited("", &props);
        assert_eq!(runtime.value().as_ref(), "");
        assert_eq!(
            outcome.change.expect("change").details.reason,
            OTPFieldChangeReason::InputClear
        );
    }

    #[test]
    fn controlled_edits_report_without_mutating() {
        let mut runtime = OTPFieldRuntime::new("12");
        let props = props(4);
        runtime.reconcile(Some(String::from("12")), true, &props);
        runtime.move_active(OTPFieldMove::EndOfValue, &props);

        let outcome = runtime.insert_text("3", OTPFieldChangeReason::InputChange, &props);

        assert_eq!(runtime.value().as_ref(), "12");
        assert_eq!(outcome.change.expect("change").value.as_ref(), "123");
    }
}
