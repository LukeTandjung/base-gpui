//! Safe-polygon hover-intent primitive.
//!
//! Keeps hover-opened popups open while the pointer travels diagonally from a
//! trigger to its popup. On trigger unhover the consumer arms a [`SafePolygon`]
//! tracker with the pointer's exit point, the trigger bounds, the popup bounds,
//! and the popup's side relative to the trigger. On every subsequent mouse move
//! the consumer feeds the pointer position to [`SafePolygon::evaluate`] and acts
//! on the returned [`SafePolygonVerdict`].
//!
//! # Integration contract
//!
//! The primitive owns no timers; it composes with the generation-based hover
//! timer substrate (`schedule_hover` / `cancel_hover` / `take_scheduled_hover`
//! in `crates/base_gpui/src/tooltip/runtime.rs`):
//!
//! 1. **Arm on trigger unhover.** Record the exit point (the same
//!    `window.mouse_position()` source the tooltip's
//!    `close_delay_for_trigger_unhover` uses), call
//!    [`SafePolygon::arm`] with measured trigger/popup bounds and the effective
//!    side, and schedule a delayed close through a `schedule_hover`-style
//!    generation.
//! 2. **Evaluate on every mouse move while armed.** Map verdicts:
//!    - [`SafePolygonVerdict::Inside`] — cancel the pending generation and
//!      reschedule a *short grace* close ([`SafePolygonConfig::inside_grace`],
//!      40ms by default). `Inside` is only a stay of execution: the tracker
//!      never keeps a popup open indefinitely on its own; a pointer that stops
//!      moving inside the polygon must still close via the grace timer.
//!    - [`SafePolygonVerdict::Outside`] — let/force the pending close through
//!      its generation check immediately (still generation-checked so a
//!      concurrent re-hover stays cancel-safe).
//!    - [`SafePolygonVerdict::LandedPopup`] / [`SafePolygonVerdict::LandedTrigger`]
//!      — the tracker disarms itself; the consumer's existing popup-hover /
//!      trigger-hover keep-open paths (`set_popup_hovered`, `cancel_hover`)
//!      take over.
//! 3. **Re-hovering trigger or popup** cancels everything via the consumer's
//!    existing `cancel_hover` path and disarms the tracker.
//!
//! Mouse moves must be observed at the consumer's root (or window) scope while
//! armed: the pointer traverses space owned by neither the trigger nor the
//! popup, so per-element `on_mouse_move` on those two elements is not enough.
//! The exact GPUI listener mechanism (root-element capture vs window-level
//! observation) is a consumer-side decision to be recorded here once the first
//! consumer lands.
//!
//! # Consumers
//!
//! - Menu submenus — `issues/port-baseui-menu.md`
//! - Menubar hover-switch — `issues/port-baseui-menubar.md`
//! - Navigation Menu — `issues/port-baseui-navigation-menu.md`
//! - Preview Card and hover-openable Popover triggers (future ports)
//!
//! # Lower-fidelity fallback
//!
//! The sanctioned cheap fallback is the trough-only approximation the tooltip
//! currently implements privately as `point_in_safe_gap` (an axis-aligned band
//! between the two boxes plus an extended close delay). Consumers that do not
//! need full polygon fidelity can call [`point_in_trough`] directly with a
//! longer close delay and skip the tracker entirely.
//!
//! Consumers with align offsets (e.g. a submenu aligned to its item's top) only
//! need the side: the region is built from measured popup bounds, not anchor
//! math, so it is align-agnostic.

mod config;
mod geometry;
mod tracker;

#[cfg(test)]
mod tests;

pub use config::SafePolygonConfig;
pub use geometry::{point_in_quadrilateral, point_in_trough, safe_polygon_quadrilateral};
pub use tracker::{SafePolygon, SafePolygonSide, SafePolygonVerdict};
