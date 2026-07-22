//! Deep scrollbar module: pure thumb geometry, drag mapping, and
//! visibility/fade facts.
//!
//! Everything here operates on plain values ([`Pixels`], [`Instant`]) and is
//! unit-testable without a window. The scrollbar element translates GPUI
//! events into these commands and paints from these queries; it holds no
//! scroll knowledge of its own. The scroll position itself always lives in
//! the consumer's scroll handle behind
//! [`ScrollTarget`](crate::primitives::scroll::ScrollTarget) — the runtime
//! only remembers the last *observed* offset to detect scroll activity.

use std::time::{Duration, Instant};

use gpui::{px, size, Axis, Pixels, Point, Size};

use crate::primitives::scroll::{ScrollbarStyleState, ScrollbarVisibility};

/// Minimum thumb length. Very long content would otherwise shrink the thumb
/// to an ungrabbable sliver; 24px sits in the sanctioned 16–48px range.
pub const MIN_THUMB_SIZE: Pixels = px(24.0);

/// Seconds the bar stays fully visible after the last scroll/hover activity
/// in [`ScrollbarVisibility::Scrolling`] mode.
pub const FADE_OUT_DELAY: f32 = 2.0;

/// Seconds the fade from fully visible to hidden takes once the idle delay
/// has elapsed in [`ScrollbarVisibility::Scrolling`] mode.
pub const FADE_OUT_DURATION: f32 = 1.0;

/// Maximum `set_offset` calls per second while dragging the thumb, to bound
/// re-layout cost of complex scroll content.
pub const MAX_DRAG_UPDATES_PER_SECOND: u32 = 120;

/// Track-local geometry for one scrollbar axis, computed fresh each prepaint
/// from the target's current offset/content/viewport sizes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollbarAxisGeometry {
    /// Full track length along the axis (the container edge).
    pub track_len: Pixels,
    /// Reserved space at the track end (horizontal bars reserve the vertical
    /// bar's thickness so the tracks do not overlap).
    pub margin_end: Pixels,
    /// Thumb length along the axis.
    pub thumb_len: Pixels,
    /// Thumb offset from the track start.
    pub thumb_offset: Pixels,
}

impl ScrollbarAxisGeometry {
    /// The track length the thumb can actually occupy.
    pub fn usable_track_len(&self) -> Pixels {
        self.track_len - self.margin_end
    }
}

/// Compute one axis' thumb geometry, or `None` when the content does not
/// overflow the viewport on that axis (the axis then renders nothing).
///
/// `scroll_offset` follows GPUI's zero-or-negative convention.
pub fn axis_geometry(
    content_len: Pixels,
    viewport_len: Pixels,
    scroll_offset: Pixels,
    track_len: Pixels,
    margin_end: Pixels,
) -> Option<ScrollbarAxisGeometry> {
    let usable = track_len - margin_end;
    if content_len <= viewport_len || usable <= px(0.0) {
        return None;
    }
    let thumb_len = (usable * (viewport_len / content_len))
        .max(MIN_THUMB_SIZE)
        .min(usable);
    let max_scroll = content_len - viewport_len;
    let scroll_ratio = (-scroll_offset / max_scroll).clamp(0.0, 1.0);
    let thumb_offset = (usable - thumb_len) * scroll_ratio;
    Some(ScrollbarAxisGeometry {
        track_len,
        margin_end,
        thumb_len,
        thumb_offset,
    })
}

/// Map a dragged pointer position back to a clamped scroll position on the
/// axis. `grab_offset` is where inside the thumb the drag grabbed it, so the
/// grab point stays under the pointer; `track_origin` is the track start in
/// the pointer's coordinate space. The result is zero-or-negative.
pub fn drag_scroll_position(
    geometry: &ScrollbarAxisGeometry,
    pointer: Pixels,
    grab_offset: Pixels,
    track_origin: Pixels,
    content_len: Pixels,
    viewport_len: Pixels,
) -> Pixels {
    let range = geometry.usable_track_len() - geometry.thumb_len;
    if range <= px(0.0) {
        return px(0.0);
    }
    let ratio = ((pointer - grab_offset - track_origin) / range).clamp(0.0, 1.0);
    -(content_len - viewport_len) * ratio
}

/// Map a click on the track to the clamped scroll position that centers the
/// thumb on the click.
pub fn track_click_scroll_position(
    geometry: &ScrollbarAxisGeometry,
    click: Pixels,
    track_origin: Pixels,
    content_len: Pixels,
    viewport_len: Pixels,
) -> Pixels {
    drag_scroll_position(
        geometry,
        click,
        geometry.thumb_len * 0.5,
        track_origin,
        content_len,
        viewport_len,
    )
}

/// Build a full scroll offset that moves `axis` to `position` while
/// preserving the cross-axis component of `current`.
pub fn scroll_offset_for_axis(
    axis: Axis,
    position: Pixels,
    current: Point<Pixels>,
) -> Point<Pixels> {
    match axis {
        Axis::Vertical => Point {
            x: current.x,
            y: position,
        },
        Axis::Horizontal => Point {
            x: position,
            y: current.y,
        },
    }
}

/// End margin the horizontal track reserves so it does not overlap a visible
/// vertical track.
pub fn horizontal_margin_end(vertical_visible: bool, vertical_thickness: Pixels) -> Pixels {
    if vertical_visible {
        vertical_thickness
    } else {
        px(0.0)
    }
}

/// Size of the corner square where both tracks meet (vertical thickness wide,
/// horizontal thickness tall).
pub fn corner_size(vertical_thickness: Pixels, horizontal_thickness: Pixels) -> Size<Pixels> {
    size(vertical_thickness, horizontal_thickness)
}

/// Fade lifecycle of the bar under the current visibility mode, used by the
/// element to drive repaints: `Solid` schedules at most one idle timer for
/// the remaining delay, `Fading` requests animation frames, `Hidden` does
/// neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarFadePhase {
    /// Fully hidden; nothing to paint or schedule.
    Hidden,
    /// Fully visible; fade begins after the contained remaining idle delay.
    Solid {
        /// Time until the fade would start, absent further activity.
        remaining_delay: Duration,
    },
    /// Currently fading out; opacity changes every frame.
    Fading,
}

/// Interaction state for one scrollbar element: hover facts, drag anchor,
/// last observed scroll offset/time, drag rate limiting, and idle-timer
/// bookkeeping. Lives in keyed window state addressed by the element id.
#[derive(Debug, Clone)]
pub struct ScrollbarRuntime {
    hovered_track: Option<Axis>,
    hovered_thumb: Option<Axis>,
    drag: Option<(Axis, Pixels)>,
    last_offset: Point<Pixels>,
    last_activity: Option<Instant>,
    last_drag_update: Option<Instant>,
    idle_timer_scheduled: bool,
}

impl Default for ScrollbarRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollbarRuntime {
    /// A runtime that has never seen scroll activity: hidden in `Scrolling`
    /// mode until the first offset change or hover-while-visible.
    pub fn new() -> Self {
        Self {
            hovered_track: None,
            hovered_thumb: None,
            drag: None,
            last_offset: Point::default(),
            last_activity: None,
            last_drag_update: None,
            idle_timer_scheduled: false,
        }
    }

    // ── Commands ────────────────────────────────────────────────────────

    /// Record the offset observed this frame. Returns whether it changed;
    /// a change counts as scroll activity and resets the idle clock.
    pub fn observe_offset(&mut self, offset: Point<Pixels>, now: Instant) -> bool {
        if offset == self.last_offset {
            return false;
        }
        self.last_offset = offset;
        self.last_activity = Some(now);
        true
    }

    /// Update whether the pointer is over `axis`' track. Hovering while the
    /// bar is visible counts as activity (resets the idle clock). Returns
    /// whether the hover fact changed.
    pub fn set_track_hovered(
        &mut self,
        axis: Axis,
        hovered: bool,
        visibility: ScrollbarVisibility,
        now: Instant,
    ) -> bool {
        let next = if hovered { Some(axis) } else { None };
        let changed = if hovered {
            self.hovered_track != next
        } else {
            self.hovered_track == Some(axis)
        };
        if hovered {
            if self.opacity(visibility, now) > 0.0 {
                self.last_activity = Some(now);
            }
            self.hovered_track = next;
        } else if self.hovered_track == Some(axis) {
            self.hovered_track = None;
        }
        changed
    }

    /// Update whether the pointer is over `axis`' thumb. Same activity rule
    /// as [`Self::set_track_hovered`].
    pub fn set_thumb_hovered(
        &mut self,
        axis: Axis,
        hovered: bool,
        visibility: ScrollbarVisibility,
        now: Instant,
    ) -> bool {
        let next = if hovered { Some(axis) } else { None };
        let changed = if hovered {
            self.hovered_thumb != next
        } else {
            self.hovered_thumb == Some(axis)
        };
        if hovered {
            if self.opacity(visibility, now) > 0.0 {
                self.last_activity = Some(now);
            }
            self.hovered_thumb = next;
        } else if self.hovered_thumb == Some(axis) {
            self.hovered_thumb = None;
        }
        changed
    }

    /// Keep the idle clock fresh while the pointer rests on a visible bar.
    /// Called once per prepaint; returns whether the clock was reset.
    pub fn refresh_hover_activity(
        &mut self,
        visibility: ScrollbarVisibility,
        now: Instant,
    ) -> bool {
        let hovered = self.hovered_track.is_some() || self.hovered_thumb.is_some();
        if hovered && self.opacity(visibility, now) > 0.0 {
            self.last_activity = Some(now);
            true
        } else {
            false
        }
    }

    /// Start dragging `axis`' thumb, remembering where inside the thumb the
    /// pointer grabbed it. Dragging keeps the bar visible.
    pub fn begin_drag(&mut self, axis: Axis, grab_offset: Pixels, now: Instant) {
        self.drag = Some((axis, grab_offset));
        self.last_activity = Some(now);
    }

    /// End any drag. Returns whether a drag was in progress.
    pub fn end_drag(&mut self, now: Instant) -> bool {
        if self.drag.take().is_some() {
            self.last_activity = Some(now);
            true
        } else {
            false
        }
    }

    /// Rate limiter for `set_offset` during drag: claims an update slot if at
    /// least `1 / MAX_DRAG_UPDATES_PER_SECOND` has passed since the last one.
    pub fn try_claim_drag_update(&mut self, now: Instant) -> bool {
        let min_interval = Duration::from_secs(1) / MAX_DRAG_UPDATES_PER_SECOND;
        let ready = match self.last_drag_update {
            None => true,
            Some(last) => now.saturating_duration_since(last) >= min_interval,
        };
        if ready {
            self.last_drag_update = Some(now);
        }
        ready
    }

    /// Record whether the one-shot idle fade timer is currently scheduled.
    pub fn set_idle_timer_scheduled(&mut self, scheduled: bool) {
        self.idle_timer_scheduled = scheduled;
    }

    // ── Queries ─────────────────────────────────────────────────────────

    /// Whether the one-shot idle fade timer is currently scheduled.
    pub fn idle_timer_scheduled(&self) -> bool {
        self.idle_timer_scheduled
    }

    /// The active drag, as `(axis, grab offset inside the thumb)`.
    pub fn drag(&self) -> Option<(Axis, Pixels)> {
        self.drag
    }

    /// Current bar opacity in `0.0..=1.0` under `visibility`. Dragging is
    /// always fully visible; `Scrolling` fades linearly to zero between
    /// [`FADE_OUT_DELAY`] and [`FADE_OUT_DELAY`]` + `[`FADE_OUT_DURATION`]
    /// after the last activity.
    pub fn opacity(&self, visibility: ScrollbarVisibility, now: Instant) -> f32 {
        if self.drag.is_some() {
            return 1.0;
        }
        match visibility {
            ScrollbarVisibility::Always => 1.0,
            ScrollbarVisibility::Hover => {
                if self.hovered_track.is_some() || self.hovered_thumb.is_some() {
                    1.0
                } else {
                    0.0
                }
            }
            ScrollbarVisibility::Scrolling => match self.last_activity {
                None => 0.0,
                Some(last) => {
                    let elapsed = now.saturating_duration_since(last).as_secs_f32();
                    if elapsed < FADE_OUT_DELAY {
                        1.0
                    } else if elapsed < FADE_OUT_DELAY + FADE_OUT_DURATION {
                        1.0 - (elapsed - FADE_OUT_DELAY) / FADE_OUT_DURATION
                    } else {
                        0.0
                    }
                }
            },
        }
    }

    /// Fade lifecycle under `visibility`, for repaint scheduling.
    pub fn fade_phase(&self, visibility: ScrollbarVisibility, now: Instant) -> ScrollbarFadePhase {
        if visibility != ScrollbarVisibility::Scrolling || self.drag.is_some() {
            return if self.opacity(visibility, now) > 0.0 {
                ScrollbarFadePhase::Solid {
                    remaining_delay: Duration::MAX,
                }
            } else {
                ScrollbarFadePhase::Hidden
            };
        }
        match self.last_activity {
            None => ScrollbarFadePhase::Hidden,
            Some(last) => {
                let elapsed = now.saturating_duration_since(last).as_secs_f32();
                if elapsed < FADE_OUT_DELAY {
                    ScrollbarFadePhase::Solid {
                        remaining_delay: Duration::from_secs_f32(FADE_OUT_DELAY - elapsed),
                    }
                } else if elapsed < FADE_OUT_DELAY + FADE_OUT_DURATION {
                    ScrollbarFadePhase::Fading
                } else {
                    ScrollbarFadePhase::Hidden
                }
            }
        }
    }

    /// Whether pointer interaction should engage: a fully faded-out bar must
    /// not swallow clicks, but `Hover` mode stays interactable so pointer
    /// arrival can reveal it.
    pub fn is_interactable(&self, visibility: ScrollbarVisibility, now: Instant) -> bool {
        visibility == ScrollbarVisibility::Hover || self.opacity(visibility, now) > 0.0
    }

    /// Style state for one axis, from runtime facts plus the axis facts the
    /// element derives from geometry.
    pub fn style_state(
        &self,
        axis: Axis,
        visibility: ScrollbarVisibility,
        now: Instant,
        has_overflow: bool,
        at_start: bool,
        at_end: bool,
    ) -> ScrollbarStyleState {
        let scrolling = self
            .last_activity
            .is_some_and(|last| now.saturating_duration_since(last).as_secs_f32() < FADE_OUT_DELAY);
        ScrollbarStyleState {
            axis,
            hovering_track: self.hovered_track == Some(axis),
            hovering_thumb: self.hovered_thumb == Some(axis),
            scrolling,
            dragging: self.drag.is_some_and(|(drag_axis, _)| drag_axis == axis),
            has_overflow,
            at_start,
            at_end,
            opacity: self.opacity(visibility, now),
        }
    }
}
