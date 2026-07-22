//! Reusable GPUI scrollbar primitive: an overlay track + draggable thumb
//! over any scroll handle.
//!
//! # Composition
//!
//! The element positions itself absolutely and fills its parent. Wrap the
//! scrollable content in a `relative()` container and overlay the scrollbar
//! as a sibling layer:
//!
//! ```ignore
//! div()
//!     .relative()
//!     .size_full()
//!     .child(
//!         div()
//!             .id("content")
//!             .size_full()
//!             .overflow_y_scroll()
//!             .track_scroll(&handle)
//!             .child(content),
//!     )
//!     .child(scrollbar(&handle).axis(ScrollbarAxis::Vertical))
//! ```
//!
//! Works over plain scrollable divs ([`gpui::ScrollHandle`]), virtualized
//! `uniform_list` ([`gpui::UniformListScrollHandle`]), and `list`
//! ([`gpui::ListState`]) through the [`ScrollTarget`] trait; custom scroll
//! containers can implement the trait themselves. The scrollbar never owns
//! scroll position.
//!
//! # Visibility modes
//!
//! - [`ScrollbarVisibility::Scrolling`] (default): visible while scrolling,
//!   fully visible for [`FADE_OUT_DELAY`] seconds after the last
//!   scroll/hover activity, then fades to hidden over [`FADE_OUT_DURATION`]
//!   seconds. Hovering the visible bar resets the idle clock; dragging keeps
//!   it visible.
//! - [`ScrollbarVisibility::Hover`]: visible while the pointer is over the
//!   scrollbar region.
//! - [`ScrollbarVisibility::Always`]: always visible while content
//!   overflows.
//!
//! Styling goes through `style_with_state(...)` with a typed
//! [`ScrollbarStyleState`]; there are no DOM data attributes or CSS
//! variables.
//!
//! # Consumers
//!
//! The Base UI Scroll Area port (`issues/port-baseui-scroll-area.md`) is the
//! intended compound consumer, composing this primitive for its
//! Root/Viewport/Scrollbar/Thumb/Corner anatomy. Select's dropdown may adopt
//! it later (`issues/port-baseui-select.md`).

mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;
pub mod target;

#[cfg(test)]
mod tests;

pub use layers::{scrollbar, scrollbar_horizontal, scrollbar_vertical, Scrollbar};
pub use props::{ScrollbarAxis, ScrollbarStyle, ScrollbarVisibility};
pub use runtime::{
    axis_geometry, corner_size, drag_scroll_position, horizontal_margin_end,
    scroll_offset_for_axis, track_click_scroll_position, ScrollbarAxisGeometry, ScrollbarFadePhase,
    ScrollbarRuntime, FADE_OUT_DELAY, FADE_OUT_DURATION, MAX_DRAG_UPDATES_PER_SECOND,
    MIN_THUMB_SIZE,
};
pub use style_state::ScrollbarStyleState;
pub use target::ScrollTarget;
