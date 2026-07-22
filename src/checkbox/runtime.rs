use crate::checkbox::{CheckboxIndicatorStyleState, CheckboxProps, CheckboxRootStyleState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CheckboxCheckedChangeReason {
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckboxCheckedChangeSource {
    Pointer,
    Keyboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckboxCheckedChangeDetails {
    reason: CheckboxCheckedChangeReason,
    source: CheckboxCheckedChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl CheckboxCheckedChangeDetails {
    pub fn new(
        reason: CheckboxCheckedChangeReason,
        source: CheckboxCheckedChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> CheckboxCheckedChangeReason {
        self.reason
    }

    pub fn source(&self) -> CheckboxCheckedChangeSource {
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

pub struct ToggleOutcome {
    changed: bool,
    checked: bool,
}

impl ToggleOutcome {
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
pub struct CheckboxRuntime {
    checked: Option<bool>,
    focused: bool,
}

impl CheckboxRuntime {
    pub fn new(checked: Option<bool>) -> Self {
        Self {
            checked,
            ..Self::default()
        }
    }

    pub fn checked_value(&self) -> Option<bool> {
        self.checked
    }

    pub fn sync_checked_from_context(&mut self, checked: Option<bool>) {
        self.checked = checked;
    }

    pub fn checked(&self) -> bool {
        self.checked.unwrap_or(false)
    }

    pub fn request_toggle(&self, disabled: bool, read_only: bool) -> ToggleOutcome {
        if disabled || read_only {
            return ToggleOutcome::new(false, self.checked());
        }

        let next = !self.checked();
        let changed = self.checked != Some(next);

        ToggleOutcome::new(changed, next)
    }

    pub fn commit_checked(&mut self, checked: bool) -> bool {
        let changed = self.checked != Some(checked);
        self.checked = Some(checked);
        changed
    }

    pub fn sync_focused(&mut self, focused: bool) -> bool {
        if self.focused == focused {
            return false;
        }

        self.focused = focused;
        true
    }

    pub fn root_state(&self, props: &CheckboxProps) -> CheckboxRootStyleState {
        CheckboxRootStyleState::new(
            self.checked(),
            props.disabled(),
            props.read_only(),
            props.required(),
            props.indeterminate(),
            self.focused,
        )
    }

    pub fn indicator_state(
        &self,
        keep_mounted: bool,
        props: &CheckboxProps,
    ) -> CheckboxIndicatorStyleState {
        CheckboxIndicatorStyleState::new(self.root_state(props), keep_mounted)
    }
}

#[cfg(test)]
mod tests {
    use super::CheckboxRuntime;

    #[test]
    fn disabled_toggle_does_not_change_checked_value() {
        let runtime = CheckboxRuntime::new(Some(false));

        let outcome = runtime.request_toggle(true, false);

        assert!(!outcome.changed());
        assert!(!runtime.checked());
    }
}
