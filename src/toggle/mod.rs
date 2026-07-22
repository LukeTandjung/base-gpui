//! # Accessibility
//!
//! The `Toggle` layer enters the AccessKit tree as `Role::Button` with
//! `aria_toggled` reflecting the pressed state (the AccessKit stand-in for
//! `aria-pressed`; screen readers may phrase it as "toggled"/"checked").
//!
//! - Icon-only toggles must set `.aria_label(...)`; there is no
//!   `aria-labelledby` id-reference wiring in this gpui revision.
//! - When a toggle sets `.aria_label(...)` and also renders visible text that
//!   duplicates the label, render that text with `Text::new_inaccessible(...)`
//!   instead of `text!(...)` so the label is not announced twice.
//! - Disabled announcement gap: this gpui revision has no `.aria_disabled(...)`
//!   builder, so assistive technology is not told the toggle is disabled. The
//!   interim behavior is that a disabled toggle is removed from the tab order
//!   and its activation paths (pointer, keyboard, and AT-dispatched actions)
//!   are inert; announcing the disabled state is blocked pending a gpui
//!   upstream `set_disabled` addition.

pub mod actions;
pub mod context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{init, ToggleActivate, TOGGLE_KEY_CONTEXT};
pub use context::ToggleContext;
pub use layers::{toggle_focus_handle, Toggle};
pub use props::{TogglePressedChangeHandler, ToggleProps};
pub use runtime::{
    TogglePressOutcome, TogglePressedChangeDetails, TogglePressedChangeReason,
    TogglePressedChangeSource, ToggleRuntime,
};
pub use style_state::ToggleStyleState;
