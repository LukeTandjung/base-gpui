use crate::toggle::ToggleStyleState;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TogglePressedChangeReason {
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TogglePressedChangeSource {
    Pointer,
    Keyboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TogglePressedChangeDetails {
    reason: TogglePressedChangeReason,
    source: TogglePressedChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl TogglePressedChangeDetails {
    pub fn new(
        reason: TogglePressedChangeReason,
        source: TogglePressedChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> TogglePressedChangeReason {
        self.reason
    }

    pub fn source(&self) -> TogglePressedChangeSource {
        self.source
    }

    pub fn cancelable(&self) -> bool {
        self.cancelable
    }

    pub fn cancel(&mut self) {
        if self.cancelable {
            self.canceled = true;
        }
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }
}

pub struct TogglePressOutcome {
    changed: bool,
    pressed: bool,
}

impl TogglePressOutcome {
    fn new(changed: bool, pressed: bool) -> Self {
        Self { changed, pressed }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }
}

#[derive(Clone, Default)]
pub struct ToggleRuntime {
    pressed: Option<bool>,
    focused: bool,
    own_disabled: bool,
    group_pressed: Option<bool>,
    group_disabled: bool,
}

impl ToggleRuntime {
    pub fn new(pressed: Option<bool>) -> Self {
        Self {
            pressed,
            ..Self::default()
        }
    }

    /// Returns the currently observed pressed value.
    pub fn pressed_value(&self) -> Option<bool> {
        self.pressed
    }

    /// Reconciles the runtime with the externally observed pressed value.
    pub fn sync_pressed_from_context(&mut self, pressed: Option<bool>) {
        self.pressed = pressed;
    }

    /// Reconciles the toggle's own disabled prop.
    pub fn sync_own_disabled(&mut self, disabled: bool) {
        self.own_disabled = disabled;
    }

    /// Reconciles group-derived facts: whether the group value contains this
    /// toggle's `value` (`None` when no group is present) and whether the group
    /// is disabled. Standalone toggles never call this.
    pub fn sync_group(&mut self, group_pressed: Option<bool>, group_disabled: bool) {
        self.group_pressed = group_pressed;
        self.group_disabled = group_disabled;
    }

    /// Answers whether the toggle is currently pressed. Group membership wins
    /// over local state when a group is present.
    pub fn pressed(&self) -> bool {
        match self.group_pressed {
            Some(pressed) => pressed,
            None => self.pressed.unwrap_or(false),
        }
    }

    /// Answers the resolved disabled fact: own disabled prop OR group disabled.
    pub fn disabled(&self) -> bool {
        self.own_disabled || self.group_disabled
    }

    /// Computes the pressed value requested by a user activation without
    /// committing it.
    pub fn request_toggle(&self) -> TogglePressOutcome {
        if self.disabled() {
            return TogglePressOutcome::new(false, self.pressed());
        }

        let next = !self.pressed();

        TogglePressOutcome::new(true, next)
    }

    /// Commits an accepted user activation to uncontrolled runtime state.
    pub fn commit_pressed(&mut self, pressed: bool) -> bool {
        let changed = self.pressed != Some(pressed);
        self.pressed = Some(pressed);
        changed
    }

    /// Reconciles whether the toggle focus handle is focused.
    pub fn sync_focused(&mut self, focused: bool) -> bool {
        if self.focused == focused {
            return false;
        }

        self.focused = focused;
        true
    }

    /// Returns the style state for the `Toggle` layer.
    pub fn state(&self) -> ToggleStyleState {
        ToggleStyleState::new(self.pressed(), self.disabled(), self.focused)
    }
}

#[cfg(test)]
mod tests {
    use super::ToggleRuntime;

    #[test]
    fn disabled_toggle_request_does_not_change_pressed_value() {
        let mut runtime = ToggleRuntime::new(Some(false));
        runtime.sync_own_disabled(true);

        let outcome = runtime.request_toggle();

        assert!(!outcome.changed());
        assert!(!runtime.pressed());
    }

    #[test]
    fn toggle_request_does_not_commit_pressed_value() {
        let runtime = ToggleRuntime::new(Some(false));

        let outcome = runtime.request_toggle();

        assert!(outcome.changed());
        assert!(outcome.pressed());
        assert!(!runtime.pressed());
    }
}
