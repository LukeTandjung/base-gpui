//! Deep Scroll Area module: scroll activity, overflow-edge, hidden-axis, and
//! corner facts derived from observed scroll offsets and layout measurements.
//!
//! The runtime owns the [`ScrollHandle`] the Viewport scrolls with; scroll
//! position itself always lives in that handle. The runtime only remembers
//! the last *observed* offset to detect per-axis scroll activity, so
//! scrolling caused by the composed scrollbar primitive (thumb drag, track
//! click, wheel over track) marks an axis as scrolling through the same
//! observed-offset path as wheel scrolling over the viewport. Everything
//! here is plain `&mut self` / `&self` over injected offsets, sizes, and
//! times — unit-testable without a window.

use std::time::{Duration, Instant};

use gpui::{size, Axis, Pixels, Point, ScrollHandle, Size};

use crate::primitives::scroll::ScrollbarStyle;
use crate::scroll_area::{
    ScrollAreaCornerStyleState, ScrollAreaEdgeThreshold, ScrollAreaRootStyleState,
    ScrollAreaScrollbarStyleState, ScrollAreaThumbStyleState,
};

/// How long an axis stays marked as scrolling after its last observed scroll
/// activity, matching Base UI's `SCROLL_TIMEOUT` of 500ms. The deadline
/// extends on continued scrolling.
pub const SCROLL_TIMEOUT: Duration = Duration::from_millis(500);

/// Which axis a Scroll Area scrollbar (and its thumb) tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAreaOrientation {
    /// Vertical scrollbar (the default, as in Base UI).
    #[default]
    Vertical,
    /// Horizontal scrollbar.
    Horizontal,
}

impl ScrollAreaOrientation {
    /// The GPUI axis this orientation scrolls along.
    pub fn axis(self) -> Axis {
        match self {
            Self::Vertical => Axis::Vertical,
            Self::Horizontal => Axis::Horizontal,
        }
    }
}

/// All Scroll Area state: the shared scroll handle, hover fact, per-axis
/// scrolling deadlines, last-observed offset, overflow-edge flags, hidden
/// state, measurement fact, and scrollbar thicknesses for corner sizing.
pub struct ScrollAreaRuntime {
    scroll_handle: ScrollHandle,
    hovering: bool,
    scrolling_x_deadline: Option<Instant>,
    scrolling_y_deadline: Option<Instant>,
    last_offset: Point<Pixels>,
    overflow_x_start: bool,
    overflow_x_end: bool,
    overflow_y_start: bool,
    overflow_y_end: bool,
    hidden_x: bool,
    hidden_y: bool,
    measured: bool,
    vertical_thickness: Pixels,
    horizontal_thickness: Pixels,
    scroll_timer_scheduled: bool,
}

impl Default for ScrollAreaRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollAreaRuntime {
    /// A runtime that has not observed any layout yet: both axes hidden,
    /// nothing scrolling, corner sized from the primitive's default
    /// scrollbar thickness until real thicknesses are measured.
    pub fn new() -> Self {
        let default_thickness = ScrollbarStyle::default().thickness;
        Self {
            scroll_handle: ScrollHandle::new(),
            hovering: false,
            scrolling_x_deadline: None,
            scrolling_y_deadline: None,
            last_offset: Point::default(),
            overflow_x_start: false,
            overflow_x_end: false,
            overflow_y_start: false,
            overflow_y_end: false,
            hidden_x: true,
            hidden_y: true,
            measured: false,
            vertical_thickness: default_thickness,
            horizontal_thickness: default_thickness,
            scroll_timer_scheduled: false,
        }
    }

    // ── Commands ────────────────────────────────────────────────────────

    /// Record the offset observed this frame. An axis whose component
    /// changed becomes scrolling with a deadline of `now + SCROLL_TIMEOUT`;
    /// a change on one axis does not mark the other. Returns whether
    /// anything changed.
    pub fn observe_scroll(&mut self, offset: Point<Pixels>, now: Instant) -> bool {
        let x_changed = offset.x != self.last_offset.x;
        let y_changed = offset.y != self.last_offset.y;
        if x_changed {
            self.scrolling_x_deadline = Some(now + SCROLL_TIMEOUT);
        }
        if y_changed {
            self.scrolling_y_deadline = Some(now + SCROLL_TIMEOUT);
        }
        self.last_offset = offset;
        x_changed || y_changed
    }

    /// Update whether the pointer is inside the Root's bounds. Returns
    /// whether the fact changed.
    pub fn set_hovering(&mut self, hovering: bool) -> bool {
        let changed = self.hovering != hovering;
        self.hovering = hovering;
        changed
    }

    /// Clear each axis's scrolling flag whose deadline has passed. Returns
    /// whether anything changed; a deadline extended by later activity
    /// survives.
    pub fn expire_scrolling(&mut self, now: Instant) -> bool {
        let mut changed = false;
        if self
            .scrolling_x_deadline
            .is_some_and(|deadline| deadline <= now)
        {
            self.scrolling_x_deadline = None;
            changed = true;
        }
        if self
            .scrolling_y_deadline
            .is_some_and(|deadline| deadline <= now)
        {
            self.scrolling_y_deadline = None;
            changed = true;
        }
        changed
    }

    /// Refresh hidden state and overflow-edge flags from the current offset
    /// and `max_offset` (the GPUI analogue of `clientSize` vs `scrollSize`):
    /// an axis is hidden when its `max_offset` component is zero; an edge
    /// flag is set when the scrolled/remaining distance past that edge
    /// exceeds its threshold. Marks the runtime as measured. Returns whether
    /// anything changed so parts only notify when needed.
    pub fn refresh_overflow(
        &mut self,
        offset: Point<Pixels>,
        max_offset: Point<Pixels>,
        threshold: &ScrollAreaEdgeThreshold,
    ) -> bool {
        let hidden_x = max_offset.x <= Pixels::ZERO;
        let hidden_y = max_offset.y <= Pixels::ZERO;

        // Offsets are zero-or-negative: scrolled distance from the start is
        // `-offset`, remaining distance to the end is `max_offset + offset`.
        let overflow_x_start = !hidden_x && -offset.x > threshold.x_start;
        let overflow_x_end = !hidden_x && max_offset.x + offset.x > threshold.x_end;
        let overflow_y_start = !hidden_y && -offset.y > threshold.y_start;
        let overflow_y_end = !hidden_y && max_offset.y + offset.y > threshold.y_end;

        let changed = !self.measured
            || hidden_x != self.hidden_x
            || hidden_y != self.hidden_y
            || overflow_x_start != self.overflow_x_start
            || overflow_x_end != self.overflow_x_end
            || overflow_y_start != self.overflow_y_start
            || overflow_y_end != self.overflow_y_end;

        self.hidden_x = hidden_x;
        self.hidden_y = hidden_y;
        self.overflow_x_start = overflow_x_start;
        self.overflow_x_end = overflow_x_end;
        self.overflow_y_start = overflow_y_start;
        self.overflow_y_end = overflow_y_end;
        self.measured = true;
        changed
    }

    /// Record one scrollbar's measured cross-axis thickness (vertical bar
    /// width / horizontal bar height), which sizes the corner. Returns
    /// whether it changed.
    pub fn set_scrollbar_thickness(
        &mut self,
        orientation: ScrollAreaOrientation,
        thickness: Pixels,
    ) -> bool {
        let slot = match orientation {
            ScrollAreaOrientation::Vertical => &mut self.vertical_thickness,
            ScrollAreaOrientation::Horizontal => &mut self.horizontal_thickness,
        };
        let changed = *slot != thickness;
        *slot = thickness;
        changed
    }

    /// Record whether the one-shot scroll-timeout timer is scheduled.
    pub fn set_scroll_timer_scheduled(&mut self, scheduled: bool) {
        self.scroll_timer_scheduled = scheduled;
    }

    // ── Queries ─────────────────────────────────────────────────────────

    /// The scroll handle the Viewport tracks and the scrollbar primitive
    /// reads/writes through `ScrollTarget`.
    pub fn scroll_handle(&self) -> ScrollHandle {
        self.scroll_handle.clone()
    }

    /// Whether the one-shot scroll-timeout timer is scheduled.
    pub fn scroll_timer_scheduled(&self) -> bool {
        self.scroll_timer_scheduled
    }

    /// Time until the latest per-axis scrolling deadline, or `None` when no
    /// axis is scrolling. Drives the single re-armed expiry timer.
    pub fn remaining_scroll_activity(&self, now: Instant) -> Option<Duration> {
        let latest = match (self.scrolling_x_deadline, self.scrolling_y_deadline) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (x, y) => x.or(y),
        };
        latest.map(|deadline| deadline.saturating_duration_since(now))
    }

    /// Whether the given axis is currently marked as scrolling.
    pub fn scrolling(&self, orientation: ScrollAreaOrientation) -> bool {
        match orientation {
            ScrollAreaOrientation::Vertical => self.scrolling_y_deadline.is_some(),
            ScrollAreaOrientation::Horizontal => self.scrolling_x_deadline.is_some(),
        }
    }

    /// Whether the given axis has no overflow and its scrollbar should be
    /// unmounted (unless kept mounted).
    pub fn axis_hidden(&self, orientation: ScrollAreaOrientation) -> bool {
        match orientation {
            ScrollAreaOrientation::Vertical => self.hidden_y,
            ScrollAreaOrientation::Horizontal => self.hidden_x,
        }
    }

    /// Whether the Viewport should participate in focus: at least one axis
    /// is scrollable (Base UI's conditional `tabIndex`).
    pub fn viewport_focusable(&self) -> bool {
        !self.hidden_x || !self.hidden_y
    }

    /// Whether at least one overflow measurement has been taken, guarding
    /// against a first-frame flash of a wrong-sized scrollbar.
    pub fn measured(&self) -> bool {
        self.measured
    }

    /// Corner size (vertical scrollbar width × horizontal scrollbar
    /// height), zero unless both axes overflow.
    pub fn corner_size(&self) -> Size<Pixels> {
        if self.corner_hidden() {
            Size::default()
        } else {
            size(self.vertical_thickness, self.horizontal_thickness)
        }
    }

    /// The corner is hidden unless both axes have overflow.
    pub fn corner_hidden(&self) -> bool {
        self.hidden_x || self.hidden_y
    }

    /// Shared style state for Root, Viewport, and Content.
    pub fn root_state(&self) -> ScrollAreaRootStyleState {
        ScrollAreaRootStyleState {
            scrolling: self.scrolling_x_deadline.is_some() || self.scrolling_y_deadline.is_some(),
            has_overflow_x: !self.hidden_x,
            has_overflow_y: !self.hidden_y,
            overflow_x_start: self.overflow_x_start,
            overflow_x_end: self.overflow_x_end,
            overflow_y_start: self.overflow_y_start,
            overflow_y_end: self.overflow_y_end,
            corner_hidden: self.corner_hidden(),
        }
    }

    /// Style state for the Viewport (same shape as the Root's).
    pub fn viewport_state(&self) -> ScrollAreaRootStyleState {
        self.root_state()
    }

    /// Style state for one scrollbar orientation.
    pub fn scrollbar_state(
        &self,
        orientation: ScrollAreaOrientation,
    ) -> ScrollAreaScrollbarStyleState {
        ScrollAreaScrollbarStyleState {
            root: self.root_state(),
            hovering: self.hovering,
            scrolling: self.scrolling(orientation),
            orientation,
        }
    }

    /// Style state for one scrollbar's thumb.
    pub fn thumb_state(&self, orientation: ScrollAreaOrientation) -> ScrollAreaThumbStyleState {
        ScrollAreaThumbStyleState {
            scrolling: self.scrolling(orientation),
            orientation,
        }
    }

    /// Style state for the corner.
    pub fn corner_state(&self) -> ScrollAreaCornerStyleState {
        ScrollAreaCornerStyleState {
            size: self.corner_size(),
            hidden: self.corner_hidden(),
        }
    }
}
