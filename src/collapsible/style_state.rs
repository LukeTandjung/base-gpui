use crate::utils::PresenceState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollapsibleRootStyleState {
    pub open: bool,
    pub closed: bool,
    pub disabled: bool,
}

impl CollapsibleRootStyleState {
    pub fn new(open: bool, disabled: bool) -> Self {
        Self {
            open,
            closed: !open,
            disabled,
        }
    }
}

impl Default for CollapsibleRootStyleState {
    fn default() -> Self {
        Self::new(false, false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollapsibleTriggerStyleState {
    pub open: bool,
    pub closed: bool,
    pub disabled: bool,
    pub focused: bool,
}

impl CollapsibleTriggerStyleState {
    pub fn new(root: CollapsibleRootStyleState, focused: bool) -> Self {
        Self {
            open: root.open,
            closed: root.closed,
            disabled: root.disabled,
            focused,
        }
    }
}

impl Default for CollapsibleTriggerStyleState {
    fn default() -> Self {
        Self::new(Default::default(), false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollapsiblePanelStyleState {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
}

impl CollapsiblePanelStyleState {
    pub fn new(open: bool, keep_mounted: bool) -> Self {
        let presence = PresenceState::new(open, keep_mounted);

        Self {
            open,
            closed: !open,
            mounted: presence.present,
            present: presence.present,
            transitioning: presence.transitioning,
        }
    }
}

impl Default for CollapsiblePanelStyleState {
    fn default() -> Self {
        Self::new(false, false)
    }
}
