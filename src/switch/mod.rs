//! # Accessibility
//!
//! `SwitchRoot` exposes an AccessKit node with `Role::Switch`, `aria_toggled` derived
//! from the checked style state, and an accessible name via the `.aria_label(...)`
//! builder. `Action::Click` and `Action::Focus` are auto-registered by the existing
//! `.on_click(...)` / `.track_focus(...)` wiring. `SwitchThumb` is decorative and has
//! no role, so it stays out of the accessibility tree.
//!
//! Known gaps in the pinned gpui revision (no builders exist; do not invent them):
//! - `aria-readonly`: read-only activation is still rejected by the runtime, but AT
//!   cannot perceive the read-only state.
//! - `aria-required`: not exposed until a field/validation accessibility story exists.
//! - `disabled` / `aria-disabled`: a disabled root is removed from the tab order and
//!   rejects activation, but AT cannot perceive the disabled state.
//! - `aria-labelledby`: no relationship builders; use the literal `.aria_label(...)`.

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{init, SwitchToggle, SWITCH_ROOT_KEY_CONTEXT};
pub use child::SwitchChild;
pub use context::SwitchContext;
pub use layers::{SwitchRoot, SwitchThumb};
pub use props::{SwitchCheckedChangeHandler, SwitchProps};
pub use runtime::{
    SwitchCheckedChangeDetails, SwitchCheckedChangeReason, SwitchCheckedChangeSource,
    SwitchRuntime, SwitchToggleOutcome,
};
pub use style_state::{SwitchRootStyleState, SwitchThumbStyleState};
