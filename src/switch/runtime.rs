use crate::switch::{SwitchProps, SwitchRootStyleState, SwitchThumbStyleState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SwitchCheckedChangeReason {
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwitchCheckedChangeSource {
    Pointer,
    Keyboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwitchCheckedChangeDetails {
    reason: SwitchCheckedChangeReason,
    source: SwitchCheckedChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl SwitchCheckedChangeDetails {
    pub fn new(
        reason: SwitchCheckedChangeReason,
        source: SwitchCheckedChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> SwitchCheckedChangeReason {
        self.reason
    }

    pub fn source(&self) -> SwitchCheckedChangeSource {
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

pub struct SwitchToggleOutcome {
    changed: bool,
    checked: bool,
}

impl SwitchToggleOutcome {
    fn new(changed: bool, checked: bool) -> Self {
        Self { changed, checked }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn checked(&self) -> bool {
        self.checked
    }
}

#[derive(Clone, Default)]
pub struct SwitchRuntime {
    checked: Option<bool>,
    focused: bool,
}

impl SwitchRuntime {
    pub fn new(checked: Option<bool>) -> Self {
        Self {
            checked,
            ..Self::default()
        }
    }

    /// Returns the currently observed checked value.
    pub fn checked_value(&self) -> Option<bool> {
        self.checked
    }

    /// Reconciles the runtime with the externally observed checked value.
    pub fn sync_checked_from_context(&mut self, checked: Option<bool>) {
        self.checked = checked;
    }

    /// Answers whether the switch is currently on.
    pub fn checked(&self) -> bool {
        self.checked.unwrap_or(false)
    }

    /// Computes the checked value requested by a user toggle without committing it.
    pub fn request_toggle(&self, disabled: bool, read_only: bool) -> SwitchToggleOutcome {
        if disabled || read_only {
            return SwitchToggleOutcome::new(false, self.checked());
        }

        let next = !self.checked();
        let changed = self.checked != Some(next);

        SwitchToggleOutcome::new(changed, next)
    }

    /// Commits an accepted user toggle to uncontrolled runtime state.
    pub fn commit_checked(&mut self, checked: bool) -> bool {
        let changed = self.checked != Some(checked);
        self.checked = Some(checked);
        changed
    }

    /// Reconciles whether the root focus handle is focused.
    pub fn sync_focused(&mut self, focused: bool) -> bool {
        if self.focused == focused {
            return false;
        }

        self.focused = focused;
        true
    }

    /// Returns the style state for `SwitchRoot`.
    pub fn root_state(&self, props: &SwitchProps) -> SwitchRootStyleState {
        SwitchRootStyleState::new(
            self.checked(),
            props.disabled(),
            props.read_only(),
            props.required(),
            self.focused,
        )
    }

    /// Returns the style state for `SwitchThumb`.
    pub fn thumb_state(&self, props: &SwitchProps) -> SwitchThumbStyleState {
        SwitchThumbStyleState::new(self.root_state(props))
    }
}

#[cfg(test)]
mod tests {
    use super::SwitchRuntime;

    #[test]
    fn disabled_toggle_request_does_not_change_checked_value() {
        let runtime = SwitchRuntime::new(Some(false));

        let outcome = runtime.request_toggle(true, false);

        assert!(!outcome.changed());
        assert!(!runtime.checked());
    }

    #[test]
    fn toggle_request_does_not_commit_checked_value() {
        let runtime = SwitchRuntime::new(Some(false));

        let outcome = runtime.request_toggle(false, false);

        assert!(outcome.changed());
        assert!(outcome.checked());
        assert!(!runtime.checked());
    }
}
