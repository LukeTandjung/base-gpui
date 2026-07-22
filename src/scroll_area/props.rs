//! Public Scroll Area configuration: the per-edge overflow thresholds that
//! gate the Root's overflow-edge flags.

use gpui::Pixels;

/// Per-edge distances the content must be scrolled past (start edges) or
/// away from (end edges) before the corresponding overflow flag is set.
/// Negative inputs clamp to zero; the default is zero on all edges, which
/// reduces the flags to "not at that edge".
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScrollAreaEdgeThreshold {
    /// Threshold for `overflow_x_start`.
    pub x_start: Pixels,
    /// Threshold for `overflow_x_end`.
    pub x_end: Pixels,
    /// Threshold for `overflow_y_start`.
    pub y_start: Pixels,
    /// Threshold for `overflow_y_end`.
    pub y_end: Pixels,
}

impl ScrollAreaEdgeThreshold {
    /// Per-edge thresholds; each value clamps to zero when negative.
    pub fn new(x_start: Pixels, x_end: Pixels, y_start: Pixels, y_end: Pixels) -> Self {
        Self {
            x_start: x_start.max(Pixels::ZERO),
            x_end: x_end.max(Pixels::ZERO),
            y_start: y_start.max(Pixels::ZERO),
            y_end: y_end.max(Pixels::ZERO),
        }
    }

    /// The same threshold on all four edges.
    pub fn uniform(value: Pixels) -> Self {
        Self::new(value, value, value, value)
    }
}

impl From<Pixels> for ScrollAreaEdgeThreshold {
    fn from(value: Pixels) -> Self {
        Self::uniform(value)
    }
}

/// Injected Scroll Area configuration shared by all parts.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScrollAreaProps {
    overflow_edge_threshold: ScrollAreaEdgeThreshold,
}

impl ScrollAreaProps {
    /// Build props from the Root's configuration.
    pub fn new(overflow_edge_threshold: ScrollAreaEdgeThreshold) -> Self {
        Self {
            overflow_edge_threshold,
        }
    }

    /// The configured overflow-edge thresholds.
    pub fn overflow_edge_threshold(&self) -> &ScrollAreaEdgeThreshold {
        &self.overflow_edge_threshold
    }
}
