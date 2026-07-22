use crate::collapsible::{
    CollapsiblePanelStyleState, CollapsibleProps, CollapsibleRootStyleState,
    CollapsibleTriggerStyleState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollapsibleOpenChangeReason {
    TriggerPress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollapsibleOpenChangeSource {
    Pointer,
    Keyboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollapsibleOpenChangeDetails {
    reason: CollapsibleOpenChangeReason,
    source: CollapsibleOpenChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl CollapsibleOpenChangeDetails {
    pub fn new(
        reason: CollapsibleOpenChangeReason,
        source: CollapsibleOpenChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> CollapsibleOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> CollapsibleOpenChangeSource {
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

pub struct CollapsibleToggleOutcome {
    changed: bool,
    open: bool,
}

impl CollapsibleToggleOutcome {
    fn new(changed: bool, open: bool) -> Self {
        Self { changed, open }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn open(&self) -> bool {
        self.open
    }
}

#[derive(Clone, Default)]
pub struct CollapsibleRuntime {
    open: Option<bool>,
    trigger_focused: bool,
}

impl CollapsibleRuntime {
    pub fn new(open: Option<bool>) -> Self {
        Self {
            open,
            ..Self::default()
        }
    }

    /// Returns the currently observed open value.
    pub fn open_value(&self) -> Option<bool> {
        self.open
    }

    /// Reconciles the runtime with the externally observed open value.
    pub fn sync_open_from_context(&mut self, open: Option<bool>) {
        self.open = open;
    }

    /// Answers whether the collapsible panel is currently open.
    pub fn open(&self) -> bool {
        self.open.unwrap_or(false)
    }

    /// Computes the open value requested by a user toggle without committing it.
    pub fn request_toggle(&self, disabled: bool) -> CollapsibleToggleOutcome {
        if disabled {
            return CollapsibleToggleOutcome::new(false, self.open());
        }

        let next = !self.open();
        let changed = self.open != Some(next);

        CollapsibleToggleOutcome::new(changed, next)
    }

    /// Commits an accepted user toggle to uncontrolled runtime state.
    pub fn commit_open(&mut self, open: bool) -> bool {
        let changed = self.open != Some(open);
        self.open = Some(open);
        changed
    }

    /// Reconciles whether any trigger focus handle is focused.
    pub fn sync_trigger_focused(&mut self, focused: bool) -> bool {
        if self.trigger_focused == focused {
            return false;
        }

        self.trigger_focused = focused;
        true
    }

    /// Returns the style state for `CollapsibleRoot`.
    pub fn root_state(&self, props: &CollapsibleProps) -> CollapsibleRootStyleState {
        CollapsibleRootStyleState::new(self.open(), props.disabled())
    }

    /// Returns the style state for `CollapsibleTrigger`.
    pub fn trigger_state(&self, props: &CollapsibleProps) -> CollapsibleTriggerStyleState {
        CollapsibleTriggerStyleState::new(self.root_state(props), self.trigger_focused)
    }

    /// Returns the style state for `CollapsiblePanel`.
    pub fn panel_state(&self, keep_mounted: bool) -> CollapsiblePanelStyleState {
        CollapsiblePanelStyleState::new(self.open(), keep_mounted)
    }
}

#[cfg(test)]
mod tests {
    use super::CollapsibleRuntime;

    #[test]
    fn disabled_toggle_request_does_not_change_open_value() {
        let runtime = CollapsibleRuntime::new(Some(false));

        let outcome = runtime.request_toggle(true);

        assert!(!outcome.changed());
        assert!(!runtime.open());
    }

    #[test]
    fn toggle_request_does_not_commit_open_value() {
        let runtime = CollapsibleRuntime::new(Some(false));

        let outcome = runtime.request_toggle(false);

        assert!(outcome.changed());
        assert!(outcome.open());
        assert!(!runtime.open());
    }
}
