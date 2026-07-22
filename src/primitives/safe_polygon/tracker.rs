use std::time::Duration;

use gpui::{Bounds, Pixels, Point};

use super::config::SafePolygonConfig;
use super::geometry::{
    exit_is_opposite_side, point_in_quadrilateral, point_in_trough, safe_polygon_quadrilateral,
};

/// Where the popup sits relative to the trigger. Local to the primitive;
/// consumers map their own side types (e.g. `TooltipSide`, a future
/// `MenuSide`) into it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafePolygonSide {
    Top,
    Bottom,
    Left,
    Right,
}

/// What the consumer should do with its pending close after feeding a pointer
/// position to [`SafePolygon::evaluate`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafePolygonVerdict {
    /// The pointer is credibly traveling toward the popup. Cancel the pending
    /// close generation and reschedule a short grace close
    /// ([`SafePolygonConfig::inside_grace`]). This is only a stay of
    /// execution — the tracker never suppresses a close indefinitely.
    Inside,
    /// Hover intent is gone (outside the region, opposite-side exit, or the
    /// cursor parked mid-flight). Let the pending close run now through its
    /// generation check.
    Outside,
    /// The pointer reached the popup. The tracker has disarmed itself; the
    /// consumer's popup-hover keep-open path takes over.
    LandedPopup,
    /// The pointer returned to the trigger. The tracker has disarmed itself;
    /// the consumer's trigger-hover path cancels the close.
    LandedTrigger,
}

#[derive(Clone, Copy, Debug)]
struct ArmedRegion {
    exit: Point<Pixels>,
    trigger: Bounds<Pixels>,
    popup: Bounds<Pixels>,
    side: SafePolygonSide,
}

#[derive(Clone, Copy, Debug)]
struct CursorSample {
    position: Point<Pixels>,
    at: Duration,
}

/// Hover-intent tracker with an arm/evaluate/disarm lifecycle. Pure state and
/// geometry: no timers, no entities, no window. See the module docs for the
/// integration contract.
#[derive(Clone, Debug)]
pub struct SafePolygon {
    config: SafePolygonConfig,
    armed: Option<ArmedRegion>,
    last_sample: Option<CursorSample>,
}

impl SafePolygon {
    pub fn new(config: SafePolygonConfig) -> Self {
        Self {
            config,
            armed: None,
            last_sample: None,
        }
    }

    /// Captures the pointer's exit point, both bounds, and the side. Arming
    /// replaces any previous armed region and resets velocity sample history.
    pub fn arm(
        &mut self,
        exit_point: Point<Pixels>,
        trigger_bounds: Bounds<Pixels>,
        popup_bounds: Bounds<Pixels>,
        side: SafePolygonSide,
    ) {
        self.armed = Some(ArmedRegion {
            exit: exit_point,
            trigger: trigger_bounds,
            popup: popup_bounds,
            side,
        });
        self.last_sample = None;
    }

    /// Clears the armed region and sample history. An unarmed tracker's
    /// `evaluate` is side-effect free and always answers [`SafePolygonVerdict::Outside`]
    /// (the gate is open; plain close-delay behavior applies).
    pub fn disarm(&mut self) {
        self.armed = None;
        self.last_sample = None;
    }

    pub fn is_armed(&self) -> bool {
        self.armed.is_some()
    }

    /// Evaluates a pointer position observed while armed. `now` is a timestamp
    /// measured from any fixed epoch (only differences between consecutive
    /// calls matter), injected so velocity logic is testable without real
    /// time.
    pub fn evaluate(&mut self, pointer: Point<Pixels>, now: Duration) -> SafePolygonVerdict {
        let Some(region) = self.armed else {
            return SafePolygonVerdict::Outside;
        };

        if region.popup.contains(&pointer) {
            self.disarm();
            return SafePolygonVerdict::LandedPopup;
        }
        if region.trigger.contains(&pointer) {
            self.disarm();
            return SafePolygonVerdict::LandedTrigger;
        }

        if exit_is_opposite_side(region.exit, region.trigger, region.side) {
            return SafePolygonVerdict::Outside;
        }

        // The trough between the two boxes is unconditionally safe — no
        // velocity check — so oscillating between trigger and popup never
        // closes.
        if point_in_trough(pointer, region.trigger, region.popup, region.side) {
            return SafePolygonVerdict::Inside;
        }

        let moving_slowly = self.is_cursor_moving_slowly(pointer, now);
        if moving_slowly {
            return SafePolygonVerdict::Outside;
        }

        let quad = safe_polygon_quadrilateral(
            region.exit,
            region.trigger,
            region.popup,
            region.side,
            self.config.polygon_buffer,
        );
        if point_in_quadrilateral(pointer, quad) {
            SafePolygonVerdict::Inside
        } else {
            SafePolygonVerdict::Outside
        }
    }

    fn is_cursor_moving_slowly(&mut self, pointer: Point<Pixels>, now: Duration) -> bool {
        let previous = self.last_sample.replace(CursorSample {
            position: pointer,
            at: now,
        });
        let Some(previous) = previous else {
            // The first sample after arming never fails the velocity check.
            return false;
        };
        let Some(elapsed) = now.checked_sub(previous.at) else {
            return false;
        };
        let elapsed_ms = elapsed.as_secs_f32() * 1000.0;
        if elapsed_ms == 0.0 {
            return false;
        }
        let delta_x = f32::from(pointer.x - previous.position.x);
        let delta_y = f32::from(pointer.y - previous.position.y);
        let distance_squared = delta_x * delta_x + delta_y * delta_y;
        let threshold = elapsed_ms * self.config.cursor_speed_threshold;
        distance_squared < threshold * threshold
    }
}
