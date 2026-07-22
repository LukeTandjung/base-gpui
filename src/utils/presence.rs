#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresenceState {
    pub active: bool,
    pub present: bool,
    pub transitioning: bool,
}

impl PresenceState {
    pub fn new(active: bool, keep_mounted: bool) -> Self {
        Self {
            active,
            present: active || keep_mounted,
            transitioning: false,
        }
    }
}
