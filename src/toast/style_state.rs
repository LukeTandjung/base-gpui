use gpui::SharedString;

use crate::toast::ToastActionDef;

/// The toast's semantic type. Base UI uses free-form strings with `"loading"`
/// special-cased; the enum keeps the `Loading` no-timer rule typed while
/// `Custom` preserves free-form types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToastType {
    Loading,
    Success,
    Error,
    Custom(SharedString),
}

/// Announce priority kept from Base UI's toast record (styling/announced
/// state); its ARIA projection is deferred to the AccessKit follow-up.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPriority {
    #[default]
    Low,
    High,
}

/// A permitted swipe-to-dismiss direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastSwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ToastSwipeDirection {
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
}

/// The mount/unmount transition phase: `Starting` from add until the first
/// height measurement; `Ending` from close until removal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToastTransitionStatus {
    Starting,
    Ending,
}

/// Style state for `ToastViewport`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToastViewportStyleState {
    /// Whether the stack is expanded (hovering || focused).
    pub expanded: bool,
    /// The newest live toast's measured height (collapsed-stack sizing).
    pub frontmost_height: f32,
    /// Total mounted toast count (including ending/limited toasts).
    pub toast_count: usize,
}

/// Style state for `ToastRoot` — the typed translation of Base UI's data
/// attributes and `--toast-*` CSS variables.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToastRootStyleState {
    pub transition_status: Option<ToastTransitionStatus>,
    /// Announce priority: `High` roots take `Role::AlertDialog`, others
    /// `Role::Dialog`.
    pub priority: ToastPriority,
    pub expanded: bool,
    pub limited: bool,
    pub toast_type: Option<ToastType>,
    pub swiping: bool,
    pub swipe_direction: Option<ToastSwipeDirection>,
    pub swipe_movement_x: f32,
    pub swipe_movement_y: f32,
    /// Stacking index: the visible index while live, the dom index while
    /// ending.
    pub index: Option<usize>,
    /// The index among live (non-ending) toasts; `None` while ending.
    pub visible_index: Option<usize>,
    /// Cumulative height of preceding (newer) live toasts.
    pub offset_y: f32,
    /// The toast's measured natural height.
    pub height: f32,
}

/// Style state for `ToastContent`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToastContentStyleState {
    pub expanded: bool,
    /// Whether another live toast stacks in front (visible index > 0).
    pub behind: bool,
}

/// Style state for `ToastTitle`, carrying the record's default title text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToastTitleStyleState {
    pub toast_type: Option<ToastType>,
    pub title: Option<SharedString>,
}

/// Style state for `ToastDescription`, carrying the record's default text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToastDescriptionStyleState {
    pub toast_type: Option<ToastType>,
    pub description: Option<SharedString>,
}

/// Style state for `ToastClose`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToastCloseStyleState {
    pub toast_type: Option<ToastType>,
    /// Whether the stack is expanded (hovering || focused). Collapsed close
    /// buttons drop their a11y role, approximating Base UI's `aria-hidden`.
    pub expanded: bool,
}

/// Style state for `ToastAction`, carrying the record's action definition.
#[derive(Clone, Default)]
pub struct ToastActionStyleState {
    pub toast_type: Option<ToastType>,
    pub action: Option<ToastActionDef>,
}
