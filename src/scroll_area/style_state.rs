//! Typed style state handed to Scroll Area `style_with_state(...)`
//! callbacks, replacing Base UI's `data-hovering` / `data-scrolling` /
//! `data-has-overflow-*` / `data-overflow-*` / `data-orientation` /
//! `data-corner-hidden` DOM attributes.

use gpui::{Pixels, Size};

use crate::scroll_area::ScrollAreaOrientation;

/// Shared facts for the Root, Viewport, and Content parts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollAreaRootStyleState {
    /// Either axis scrolled within the last `SCROLL_TIMEOUT`.
    pub scrolling: bool,
    /// Content overflows the viewport horizontally.
    pub has_overflow_x: bool,
    /// Content overflows the viewport vertically.
    pub has_overflow_y: bool,
    /// Scrolled past the start-edge threshold horizontally.
    pub overflow_x_start: bool,
    /// More than the end-edge threshold remains horizontally.
    pub overflow_x_end: bool,
    /// Scrolled past the start-edge threshold vertically.
    pub overflow_y_start: bool,
    /// More than the end-edge threshold remains vertically.
    pub overflow_y_end: bool,
    /// The corner is hidden because at least one axis has no overflow.
    pub corner_hidden: bool,
}

/// Facts for one scrollbar part: the shared root facts plus hover, its own
/// orientation's scrolling flag, and the orientation itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollAreaScrollbarStyleState {
    /// The shared Root facts.
    pub root: ScrollAreaRootStyleState,
    /// The pointer is inside the Root's bounds.
    pub hovering: bool,
    /// This scrollbar's own axis scrolled within the last `SCROLL_TIMEOUT`.
    pub scrolling: bool,
    /// Which axis this scrollbar tracks.
    pub orientation: ScrollAreaOrientation,
}

impl ScrollAreaScrollbarStyleState {
    /// Whether this scrollbar's axis has overflow (`data-has-overflow-*`).
    pub fn has_overflow(&self) -> bool {
        match self.orientation {
            ScrollAreaOrientation::Vertical => self.root.has_overflow_y,
            ScrollAreaOrientation::Horizontal => self.root.has_overflow_x,
        }
    }
}

/// Facts for one scrollbar's thumb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollAreaThumbStyleState {
    /// The thumb's axis scrolled within the last `SCROLL_TIMEOUT`.
    pub scrolling: bool,
    /// Which axis this thumb tracks.
    pub orientation: ScrollAreaOrientation,
}

/// Facts for the corner square. The corner is otherwise styled purely by
/// layout: the part sizes itself from `size` and unmounts when `hidden`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScrollAreaCornerStyleState {
    /// Corner size: vertical scrollbar width × horizontal scrollbar height.
    pub size: Size<Pixels>,
    /// Hidden unless both axes have overflow.
    pub hidden: bool,
}
