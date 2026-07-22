/// Typed state for the Context Menu trigger area's `style_with_state`,
/// replacing Base UI's `data-popup-open` / `data-pressed` attributes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContextMenuTriggerStyleState {
    /// Whether the context menu opened from this trigger area is open.
    pub open: bool,
    /// Whether the trigger reads as pressed: the open gesture originated here
    /// and the menu it spawned is still open.
    pub pressed: bool,
}

impl ContextMenuTriggerStyleState {
    pub fn new(open: bool, pressed: bool) -> Self {
        Self { open, pressed }
    }
}
