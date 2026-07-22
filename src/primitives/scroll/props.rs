//! Public scrollbar configuration: axis selection, visibility policy, and
//! the appearance knobs `style_with_state(...)` operates on.

use gpui::{hsla, px, Hsla, Pixels};

/// Which axes the scrollbar renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarAxis {
    /// Vertical scrollbar only.
    Vertical,
    /// Horizontal scrollbar only.
    Horizontal,
    /// Both scrollbars, with a corner spacer where they meet.
    #[default]
    Both,
}

impl ScrollbarAxis {
    /// Whether the vertical bar is part of this axis selection.
    pub fn has_vertical(self) -> bool {
        matches!(self, Self::Vertical | Self::Both)
    }

    /// Whether the horizontal bar is part of this axis selection.
    pub fn has_horizontal(self) -> bool {
        matches!(self, Self::Horizontal | Self::Both)
    }
}

/// When the scrollbar is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollbarVisibility {
    /// Show while scrolling, fade out after idle (default). Timing is fixed
    /// by [`FADE_OUT_DELAY`](crate::primitives::scroll::FADE_OUT_DELAY) and
    /// [`FADE_OUT_DURATION`](crate::primitives::scroll::FADE_OUT_DURATION).
    #[default]
    Scrolling,
    /// Show while the pointer is over the scrollbar region.
    Hover,
    /// Always visible while content overflows.
    Always,
}

/// Appearance of one scrollbar axis. Passed to `style_with_state(...)` with
/// these defaults already applied; the callback returns the (possibly
/// adjusted) style for the current frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollbarStyle {
    /// Track background color.
    pub track_color: Hsla,
    /// Thumb fill color.
    pub thumb_color: Hsla,
    /// Full bar thickness (track cross-axis size, also the thumb hit region).
    pub thickness: Pixels,
    /// Inset of the painted thumb inside the track on every side.
    pub inset: Pixels,
    /// Thumb corner radius.
    pub corner_radius: Pixels,
}

impl Default for ScrollbarStyle {
    fn default() -> Self {
        Self {
            track_color: hsla(0.0, 0.0, 0.0, 0.0),
            thumb_color: hsla(0.0, 0.0, 0.0, 0.35),
            thickness: px(12.0),
            inset: px(3.0),
            corner_radius: px(3.0),
        }
    }
}
