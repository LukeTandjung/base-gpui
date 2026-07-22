//! Typed style state handed to `style_with_state(...)`, replacing Base UI's
//! `data-hovering` / `data-scrolling` / `data-orientation` DOM attributes.

use gpui::Axis;

/// Facts about one scrollbar axis for the current frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollbarStyleState {
    /// Which axis this bar renders.
    pub axis: Axis,
    /// The pointer is over this axis' track (thumb included).
    pub hovering_track: bool,
    /// The pointer is over this axis' thumb.
    pub hovering_thumb: bool,
    /// The scroll offset changed recently (within the idle delay).
    pub scrolling: bool,
    /// This axis' thumb is being dragged.
    pub dragging: bool,
    /// Content overflows the viewport on this axis.
    pub has_overflow: bool,
    /// The content is scrolled to its start on this axis.
    pub at_start: bool,
    /// The content is scrolled to its end on this axis.
    pub at_end: bool,
    /// Current fade opacity in `0.0..=1.0`; `0.0` means the bar is hidden
    /// and non-interactive.
    pub opacity: f32,
}
