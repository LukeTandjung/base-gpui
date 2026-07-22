use gpui::{px, ElementId, Point};

use crate::dialog::{DialogBackdropStyleState, DialogViewportStyleState};
use crate::drawer::{
    DrawerBackdropStyleState, DrawerContentStyleState, DrawerPopupFacts, DrawerSnapPoint,
    DrawerSwipeAreaStyleState, DrawerSwipeDirection, DrawerViewportStyleState,
};

// NOTE: Base UI implements this gesture engine as the shared `useSwipeDismiss`
// hook, also used by Toast. Once a GPUI Toast port exists this is a candidate for
// extraction into a shared primitive; until the concept actually repeats it stays
// drawer-internal (see the architecture doc on avoiding speculative shared
// primitives).

const REVERSE_CANCEL_THRESHOLD: f32 = 10.0;
const MIN_SWIPE_THRESHOLD: f32 = 10.0;
const MIN_GESTURE_DURATION_MS: f32 = 50.0;
const MIN_SAMPLE_DURATION_MS: f32 = 16.0;
const RELEASE_SAMPLE_MAX_AGE_MS: f32 = 80.0;
const DISMISS_VELOCITY: f32 = 0.5;
const SNAP_VELOCITY_THRESHOLD: f32 = 0.5;
const SNAP_VELOCITY_CLAMP: f32 = 4.0;
const SNAP_VELOCITY_MULTIPLIER: f32 = 300.0;
const SNAP_DEDUPE_EPSILON: f32 = 1.0;
const OPEN_KEEP_VELOCITY: f32 = 0.1;
const OPEN_FALLBACK_THRESHOLD: f32 = 40.0;
const RELEASE_MIN_VELOCITY: f32 = 0.2;
const RELEASE_MAX_VELOCITY: f32 = 4.0;
const RELEASE_MIN_DURATION_MS: f32 = 80.0;
const RELEASE_MAX_DURATION_MS: f32 = 360.0;
const NESTED_SWIPING_THRESHOLD: f32 = 10.0;

/// Signed square-root damping applied to movement along disallowed directions.
pub fn signed_sqrt_damping(delta: f32) -> f32 {
    delta.signum() * delta.abs().sqrt()
}

/// Total popup offset for a vertical snap-point drag: in range the offset tracks
/// the drag linearly; overshoot beyond the fully-open edge (offset 0) maps to
/// `-sqrt(-next_offset)`.
pub fn snap_point_swipe_offset(base_offset: f32, delta: f32) -> f32 {
    let next_offset = base_offset + delta;
    if next_offset >= 0.0 {
        next_offset
    } else {
        -(-next_offset).sqrt()
    }
}

/// A snap point resolved against the current viewport and popup measurements.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedSnapPoint {
    pub snap_point: DrawerSnapPoint,
    pub height: f32,
    pub offset: f32,
}

/// Resolves snap points to concrete heights/offsets. Fractions clamp to `0..=1`
/// of the viewport height, pixel and rem values resolve directly; all heights
/// clamp to `min(popup_height, viewport_height)`; non-finite values are skipped;
/// entries within 1px of a later entry are deduped keeping the later entry. Each
/// resolved point carries `offset = max(0, popup_height - height)`.
pub fn resolve_snap_points(
    snap_points: &[DrawerSnapPoint],
    viewport_height: f32,
    popup_height: f32,
    rem_size: f32,
) -> Vec<ResolvedSnapPoint> {
    let max_height = popup_height.min(viewport_height);
    let mut resolved: Vec<ResolvedSnapPoint> = Vec::new();
    for snap_point in snap_points {
        let raw_height = match snap_point {
            DrawerSnapPoint::Fraction(fraction) => fraction.clamp(0.0, 1.0) * viewport_height,
            DrawerSnapPoint::Px(pixels) => f32::from(*pixels),
            DrawerSnapPoint::Rems(rems) => rems.0 * rem_size,
        };
        if !raw_height.is_finite() {
            continue;
        }
        let height = raw_height.clamp(0.0, max_height);
        resolved.push(ResolvedSnapPoint {
            snap_point: *snap_point,
            height,
            offset: (popup_height - height).max(0.0),
        });
    }

    let mut deduped: Vec<ResolvedSnapPoint> = Vec::new();
    for (index, candidate) in resolved.iter().enumerate() {
        let duplicated_later = resolved
            .iter()
            .skip(index + 1)
            .any(|later| (later.height - candidate.height).abs() <= SNAP_DEDUPE_EPSILON);
        if !duplicated_later {
            deduped.push(*candidate);
        }
    }
    deduped
}

/// Finds the resolved snap point matching a value: exact member first, otherwise
/// the closest resolved snap point by height.
pub fn closest_resolved_snap_point(
    resolved: &[ResolvedSnapPoint],
    value: &DrawerSnapPoint,
    viewport_height: f32,
    rem_size: f32,
) -> Option<ResolvedSnapPoint> {
    if let Some(exact) = resolved
        .iter()
        .find(|candidate| candidate.snap_point == *value)
    {
        return Some(*exact);
    }
    let target_height = match value {
        DrawerSnapPoint::Fraction(fraction) => fraction.clamp(0.0, 1.0) * viewport_height,
        DrawerSnapPoint::Px(pixels) => f32::from(*pixels),
        DrawerSnapPoint::Rems(rems) => rems.0 * rem_size,
    };
    resolved
        .iter()
        .min_by(|left, right| {
            (left.height - target_height)
                .abs()
                .total_cmp(&(right.height - target_height).abs())
        })
        .copied()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GestureKind {
    Dismiss,
    Open { direction: DrawerSwipeDirection },
}

#[derive(Clone, Copy, Debug)]
struct VelocitySample {
    delta: f32,
    duration: f32,
    end_ms: f32,
}

#[derive(Clone, Debug)]
struct SwipeGesture {
    kind: GestureKind,
    start: (f32, f32),
    start_ms: f32,
    last: (f32, f32),
    last_ms: f32,
    locked_horizontal: Option<bool>,
    directional: f32,
    max_directional: f32,
    past_threshold: bool,
    canceled: bool,
    sample: Option<VelocitySample>,
    sample_start: (f32, f32),
    sample_start_ms: f32,
    base_offset: f32,
    requested_open: bool,
}

/// The decision made when a dismiss-direction swipe is released.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawerSwipeReleaseOutcome {
    /// The popup animates back to rest; nothing else happens.
    Rest,
    /// The drawer should be closed with the `Swipe` reason.
    Dismiss {
        strength: f32,
        restore_snap_point: Option<DrawerSnapPoint>,
    },
    /// The drawer should move to a different snap point.
    Snap { snap_point: DrawerSnapPoint },
}

/// The decision made when a swipe-to-open gesture is released.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawerOpenSwipeRelease {
    pub keep_open: bool,
    pub opened_by_gesture: bool,
}

#[derive(Clone, Debug)]
struct NestedReportEntry {
    id: ElementId,
    frontmost_height: Option<f32>,
    swiping_movement: f32,
    swipe_progress: f32,
}

/// The single owner of drawer-specific state: the swipe gesture state machine,
/// snap-point resolution/selection, measured sizes, nested-drawer reporting, and
/// release-animation parameters. Pure `&mut self` commands and queries; never
/// calls user callbacks; unit-testable without a window.
pub struct DrawerRuntime {
    swipe_direction: DrawerSwipeDirection,
    snap_points: Vec<DrawerSnapPoint>,
    snap_to_sequential_points: bool,
    active_snap_point: Option<DrawerSnapPoint>,
    observed_snap_point: Option<DrawerSnapPoint>,
    snap_point_controlled: bool,
    popup_width: Option<f32>,
    popup_height: Option<f32>,
    viewport_height: Option<f32>,
    rem_size: f32,
    gesture: Option<SwipeGesture>,
    swipe_movement: (f32, f32),
    swipe_progress: f32,
    swipe_strength: f32,
    swipe_dismissed: bool,
    dismissal_suppressed: bool,
    last_observed_open: bool,
    nested: bool,
    nested_reports: Vec<NestedReportEntry>,
}

impl DrawerRuntime {
    /// Creates the runtime for a drawer swiping toward `swipe_direction`.
    pub fn new(swipe_direction: DrawerSwipeDirection) -> Self {
        Self {
            swipe_direction,
            snap_points: Vec::new(),
            snap_to_sequential_points: false,
            active_snap_point: None,
            observed_snap_point: None,
            snap_point_controlled: false,
            popup_width: None,
            popup_height: None,
            viewport_height: None,
            rem_size: 16.0,
            gesture: None,
            swipe_movement: (0.0, 0.0),
            swipe_progress: 0.0,
            swipe_strength: 0.0,
            swipe_dismissed: false,
            dismissal_suppressed: false,
            last_observed_open: false,
            nested: false,
            nested_reports: Vec::new(),
        }
    }

    /// Synchronizes root-configured props observed in the current render pass.
    pub fn sync_props(
        &mut self,
        swipe_direction: DrawerSwipeDirection,
        snap_points: Vec<DrawerSnapPoint>,
        snap_to_sequential_points: bool,
    ) {
        self.swipe_direction = swipe_direction;
        self.snap_points = snap_points;
        self.snap_to_sequential_points = snap_to_sequential_points;
    }

    /// Reconciles the snap-point value: a controlled value is observed as-is;
    /// an uncontrolled value falls back to the resolved default when it is
    /// missing or no longer present in `snap_points`.
    pub fn reconcile_snap_point(
        &mut self,
        controlled: Option<Option<DrawerSnapPoint>>,
        default_snap_point: Option<DrawerSnapPoint>,
    ) {
        match controlled {
            Some(value) => {
                self.snap_point_controlled = true;
                self.observed_snap_point = value;
            }
            None => {
                self.snap_point_controlled = false;
                if self.active_snap_point.is_none() && !self.snap_points.is_empty() {
                    self.active_snap_point = self.resolved_default_snap_point(default_snap_point);
                } else if let Some(active) = self.active_snap_point {
                    if !self.snap_points.contains(&active) {
                        self.active_snap_point =
                            self.resolved_default_snap_point(default_snap_point);
                    }
                }
            }
        }
    }

    /// The resolved default snap point: the explicit default, else the first
    /// entry of `snap_points`, else none.
    pub fn resolved_default_snap_point(
        &self,
        default_snap_point: Option<DrawerSnapPoint>,
    ) -> Option<DrawerSnapPoint> {
        default_snap_point.or_else(|| self.snap_points.first().copied())
    }

    /// Resets the uncontrolled snap point to the resolved default after an
    /// accepted close.
    pub fn reset_snap_point_to_default(&mut self, default_snap_point: Option<DrawerSnapPoint>) {
        if !self.snap_point_controlled {
            self.active_snap_point = self.resolved_default_snap_point(default_snap_point);
        }
    }

    /// Commits an accepted uncontrolled snap-point change.
    pub fn set_snap_point_uncontrolled(&mut self, snap_point: Option<DrawerSnapPoint>) {
        self.active_snap_point = snap_point;
    }

    /// The currently effective snap-point value.
    pub fn snap_point_value(&self) -> Option<DrawerSnapPoint> {
        if self.snap_point_controlled {
            self.observed_snap_point
        } else {
            self.active_snap_point
        }
    }

    /// Reports the popup's measured size. Held while a nested drawer is present
    /// so a stretched layout is not re-measured. Returns whether anything changed.
    pub fn set_popup_size(&mut self, width: f32, height: f32) -> bool {
        if self.nested_open_drawer_count() > 0 && self.popup_height.is_some() {
            return false;
        }
        let changed = self.popup_width != Some(width) || self.popup_height != Some(height);
        self.popup_width = Some(width);
        self.popup_height = Some(height);
        changed
    }

    /// Reports the drawer viewport height (falls back to window content height).
    pub fn set_viewport_height(&mut self, height: f32) -> bool {
        let changed = self.viewport_height != Some(height);
        self.viewport_height = Some(height);
        changed
    }

    /// Reports the window rem size used for `Rems` snap points.
    pub fn set_rem_size(&mut self, rem_size: f32) {
        self.rem_size = rem_size;
    }

    /// Snap points resolved against the current measurements. Snap-point offsets
    /// only apply to vertical swipe directions; horizontal drawers ignore them.
    pub fn resolved_snap_points(&self) -> Vec<ResolvedSnapPoint> {
        if !self.swipe_direction.is_vertical() {
            return Vec::new();
        }
        let (Some(viewport_height), Some(popup_height)) = (self.viewport_height, self.popup_height)
        else {
            return Vec::new();
        };
        resolve_snap_points(
            &self.snap_points,
            viewport_height,
            popup_height,
            self.rem_size,
        )
    }

    /// The base offset of the active snap point (0 when none apply).
    pub fn active_snap_offset(&self) -> f32 {
        let resolved = self.resolved_snap_points();
        let Some(value) = self.snap_point_value() else {
            return 0.0;
        };
        closest_resolved_snap_point(
            &resolved,
            &value,
            self.viewport_height.unwrap_or(0.0),
            self.rem_size,
        )
        .map(|snap_point| snap_point.offset)
        .unwrap_or(0.0)
    }

    /// Whether the active snap point is the full `Fraction(1.0)` point.
    pub fn expanded(&self) -> bool {
        matches!(self.snap_point_value(), Some(DrawerSnapPoint::Fraction(fraction)) if fraction >= 1.0)
    }

    fn size_along_axis(&self) -> f32 {
        if self.swipe_direction.is_vertical() {
            self.popup_height.unwrap_or(0.0)
        } else {
            self.popup_width.unwrap_or(0.0)
        }
    }

    /// The per-direction swipe threshold: `max(size * 0.5, 10px)`.
    pub fn swipe_threshold(&self) -> f32 {
        (self.size_along_axis() * 0.5).max(MIN_SWIPE_THRESHOLD)
    }

    fn snap_points_active(&self) -> bool {
        self.swipe_direction.is_vertical() && !self.resolved_snap_points().is_empty()
    }

    /// Starts a dismiss-direction swipe. Refuses non-primary buttons, presses
    /// over `DrawerContent`/interactive children, and any press while a nested
    /// drawer is open. Returns whether the gesture started.
    pub fn begin_swipe(
        &mut self,
        x: f32,
        y: f32,
        time_ms: f32,
        primary_button: bool,
        over_content: bool,
    ) -> bool {
        if !primary_button
            || over_content
            || self.gesture.is_some()
            || self.nested_open_drawer_count() > 0
        {
            return false;
        }
        self.gesture = Some(SwipeGesture {
            kind: GestureKind::Dismiss,
            start: (x, y),
            start_ms: time_ms,
            last: (x, y),
            last_ms: time_ms,
            locked_horizontal: None,
            directional: 0.0,
            max_directional: 0.0,
            past_threshold: false,
            canceled: false,
            sample: None,
            sample_start: (x, y),
            sample_start_ms: time_ms,
            base_offset: self.active_snap_offset(),
            requested_open: false,
        });
        true
    }

    /// Advances the active swipe with a new pointer position. Computes the axis
    /// lock, damped movement, live progress, and reverse-cancel state. Returns
    /// whether visual state changed.
    pub fn move_swipe(&mut self, x: f32, y: f32, time_ms: f32) -> bool {
        let swipe_direction = self.swipe_direction;
        let snap_points_active = self.snap_points_active();
        let size = self.size_along_axis();
        let threshold = self.swipe_threshold();
        let popup_height = self.popup_height.unwrap_or(0.0);
        let Some(gesture) = self.gesture.as_mut() else {
            return false;
        };
        let delta_x = x - gesture.start.0;
        let delta_y = y - gesture.start.1;
        // Axis lock: after >= 1px of movement the dominant axis wins; drags whose
        // dominant axis disagrees with the drawer axis only produce damped
        // cross-axis movement below.
        if gesture.locked_horizontal.is_none() && delta_x.abs().max(delta_y.abs()) >= 1.0 {
            gesture.locked_horizontal = Some(delta_x.abs() > delta_y.abs());
        }

        // Velocity sampling: a new sample every >= 16ms.
        let sample_elapsed = time_ms - gesture.sample_start_ms;
        if sample_elapsed >= MIN_SAMPLE_DURATION_MS {
            let unit = match gesture.kind {
                GestureKind::Dismiss => swipe_direction.dismiss_unit(),
                GestureKind::Open { direction } => direction.dismiss_unit(),
            };
            let sample_delta =
                (x - gesture.sample_start.0) * unit.0 + (y - gesture.sample_start.1) * unit.1;
            gesture.sample = Some(VelocitySample {
                delta: sample_delta,
                duration: sample_elapsed,
                end_ms: time_ms,
            });
            gesture.sample_start = (x, y);
            gesture.sample_start_ms = time_ms;
        }
        gesture.last = (x, y);
        gesture.last_ms = time_ms;

        match gesture.kind {
            GestureKind::Dismiss => {
                let unit = swipe_direction.dismiss_unit();
                let axis_matches = gesture
                    .locked_horizontal
                    .map(|horizontal| horizontal == swipe_direction.is_horizontal())
                    .unwrap_or(true);
                let directional = if axis_matches {
                    delta_x * unit.0 + delta_y * unit.1
                } else {
                    0.0
                };
                gesture.directional = directional;
                gesture.max_directional = gesture.max_directional.max(directional);
                if directional > threshold {
                    gesture.past_threshold = true;
                }
                if gesture.past_threshold
                    && gesture.max_directional - directional >= REVERSE_CANCEL_THRESHOLD
                {
                    gesture.canceled = true;
                }

                let base_offset = gesture.base_offset;
                let cross = if swipe_direction.is_vertical() {
                    delta_x
                } else {
                    delta_y
                };
                let along = if snap_points_active {
                    snap_point_swipe_offset(base_offset, directional) - base_offset
                } else if directional >= 0.0 {
                    directional
                } else {
                    signed_sqrt_damping(directional)
                };
                let damped_cross = signed_sqrt_damping(cross);
                self.swipe_movement = if swipe_direction.is_vertical() {
                    (damped_cross, along * unit.1)
                } else {
                    (along * unit.0, damped_cross)
                };
                self.swipe_progress = if snap_points_active && popup_height > 0.0 {
                    ((base_offset + directional) / popup_height).clamp(0.0, 1.0)
                } else if size > 0.0 {
                    (directional / size).clamp(0.0, 1.0)
                } else {
                    0.0
                };
            }
            GestureKind::Open { direction } => {
                let unit = direction.dismiss_unit();
                let displacement = delta_x * unit.0 + delta_y * unit.1;
                gesture.directional = displacement;
                gesture.max_directional = gesture.max_directional.max(displacement);
                if displacement >= 1.0 {
                    gesture.requested_open = true;
                }
                let closed_offset = if size > 0.0 {
                    size
                } else {
                    OPEN_FALLBACK_THRESHOLD
                };
                let remaining = if displacement <= closed_offset {
                    closed_offset - displacement
                } else {
                    -((displacement - closed_offset).sqrt())
                };
                let dismiss_unit = self.swipe_direction.dismiss_unit();
                self.swipe_movement = (remaining * dismiss_unit.0, remaining * dismiss_unit.1);
                self.swipe_progress = (1.0 - displacement / closed_offset).clamp(0.0, 1.0);
            }
        }
        true
    }

    /// Cancels the active gesture (non-primary button, button loss, disable,
    /// unmount) and restores rest position and dismissal.
    pub fn cancel_swipe(&mut self) {
        self.gesture = None;
        self.swipe_movement = (0.0, 0.0);
        self.swipe_progress = 0.0;
        self.dismissal_suppressed = false;
    }

    /// Whether a swipe gesture is currently tracked.
    pub fn swiping(&self) -> bool {
        self.gesture.is_some()
    }

    /// Whether the active gesture opened the drawer through a swipe area.
    pub fn open_gesture_active(&self) -> bool {
        matches!(
            self.gesture.as_ref().map(|gesture| gesture.kind),
            Some(GestureKind::Open { .. })
        )
    }

    fn release_velocity(gesture: &SwipeGesture, release_ms: f32) -> f32 {
        let overall_duration = (gesture.last_ms - gesture.start_ms).max(MIN_GESTURE_DURATION_MS);
        let overall = gesture.directional / overall_duration;
        match gesture.sample {
            Some(sample)
                if release_ms - sample.end_ms <= RELEASE_SAMPLE_MAX_AGE_MS
                    && sample.duration >= MIN_SAMPLE_DURATION_MS =>
            {
                sample.delta / sample.duration
            }
            _ => overall,
        }
    }

    fn overall_velocity(gesture: &SwipeGesture) -> f32 {
        let duration = (gesture.last_ms - gesture.start_ms).max(MIN_GESTURE_DURATION_MS);
        gesture.directional / duration
    }

    /// The release-strength scalar: remaining travel over clamped velocity maps
    /// to a duration clamped to 80..360ms, normalized to 0.1..1.
    pub fn release_strength(remaining: f32, velocity: f32) -> f32 {
        let velocity = velocity
            .abs()
            .clamp(RELEASE_MIN_VELOCITY, RELEASE_MAX_VELOCITY);
        let duration =
            (remaining / velocity).clamp(RELEASE_MIN_DURATION_MS, RELEASE_MAX_DURATION_MS);
        let normalized = (duration - RELEASE_MIN_DURATION_MS)
            / (RELEASE_MAX_DURATION_MS - RELEASE_MIN_DURATION_MS);
        (0.1 + normalized * 0.9).clamp(0.1, 1.0)
    }

    /// Releases the dismiss swipe and decides between rest, snap selection, and
    /// dismissal.
    pub fn release_swipe(&mut self, release_ms: f32) -> DrawerSwipeReleaseOutcome {
        let Some(gesture) = self.gesture.take() else {
            return DrawerSwipeReleaseOutcome::Rest;
        };
        self.swipe_movement = (0.0, 0.0);
        self.swipe_progress = 0.0;

        if gesture.canceled {
            return DrawerSwipeReleaseOutcome::Rest;
        }

        let displacement = gesture.directional;
        let mut velocity = Self::release_velocity(&gesture, release_ms);
        // A release velocity contradicting the drag direction falls back to the
        // overall gesture velocity.
        if displacement != 0.0 && velocity != 0.0 && velocity.signum() != displacement.signum() {
            velocity = Self::overall_velocity(&gesture);
        }

        let resolved = self.resolved_snap_points();
        if resolved.is_empty() {
            let threshold = self.swipe_threshold();
            let dismiss =
                displacement > threshold || (velocity >= DISMISS_VELOCITY && displacement > 0.0);
            if dismiss {
                let remaining = (self.size_along_axis() - displacement).max(0.0);
                let strength = Self::release_strength(remaining, velocity);
                self.swipe_strength = strength;
                self.swipe_dismissed = true;
                return DrawerSwipeReleaseOutcome::Dismiss {
                    strength,
                    restore_snap_point: self.snap_point_value(),
                };
            }
            return DrawerSwipeReleaseOutcome::Rest;
        }

        let popup_height = self.popup_height.unwrap_or(0.0);
        let base_offset = gesture.base_offset;
        let restore_snap_point = self.snap_point_value();

        // Fast dismissal: directional velocity toward dismissal closes directly.
        if velocity >= DISMISS_VELOCITY && displacement > 0.0 {
            let remaining = (popup_height - base_offset - displacement).max(0.0);
            let strength = Self::release_strength(remaining, velocity);
            self.swipe_strength = strength;
            self.swipe_dismissed = true;
            return DrawerSwipeReleaseOutcome::Dismiss {
                strength,
                restore_snap_point,
            };
        }

        let mut target = (base_offset + displacement).clamp(0.0, popup_height);
        if !self.snap_to_sequential_points && velocity.abs() >= SNAP_VELOCITY_THRESHOLD {
            target += velocity.clamp(-SNAP_VELOCITY_CLAMP, SNAP_VELOCITY_CLAMP)
                * SNAP_VELOCITY_MULTIPLIER;
        }

        let candidates: Vec<&ResolvedSnapPoint> = if self.snap_to_sequential_points {
            let mut sorted = resolved.iter().collect::<Vec<_>>();
            sorted.sort_by(|left, right| left.offset.total_cmp(&right.offset));
            let current_index = sorted
                .iter()
                .position(|candidate| (candidate.offset - base_offset).abs() <= 0.5)
                .unwrap_or(0);
            let velocity_confirms = velocity == 0.0
                || displacement == 0.0
                || velocity.signum() == displacement.signum();
            let step: isize = if !velocity_confirms || displacement == 0.0 {
                0
            } else if displacement > 0.0 {
                1
            } else {
                -1
            };
            let adjacent_index = current_index as isize + step;
            if adjacent_index >= sorted.len() as isize && displacement > 0.0 {
                // Advancing past the last point toward dismissal closes.
                let remaining = (popup_height - base_offset - displacement).max(0.0);
                let strength = Self::release_strength(remaining, velocity);
                self.swipe_strength = strength;
                self.swipe_dismissed = true;
                return DrawerSwipeReleaseOutcome::Dismiss {
                    strength,
                    restore_snap_point,
                };
            }
            let adjacent_index = adjacent_index.clamp(0, sorted.len() as isize - 1) as usize;
            vec![sorted[current_index], sorted[adjacent_index]]
        } else {
            resolved.iter().collect()
        };

        let best = candidates.iter().min_by(|left, right| {
            (left.offset - target)
                .abs()
                .total_cmp(&(right.offset - target).abs())
        });
        let Some(best) = best else {
            return DrawerSwipeReleaseOutcome::Rest;
        };

        let close_distance = (popup_height - target).abs();
        if !self.snap_to_sequential_points && close_distance < (best.offset - target).abs() {
            let remaining = (popup_height - base_offset - displacement).max(0.0);
            let strength = Self::release_strength(remaining, velocity.max(RELEASE_MIN_VELOCITY));
            self.swipe_strength = strength;
            self.swipe_dismissed = true;
            return DrawerSwipeReleaseOutcome::Dismiss {
                strength,
                restore_snap_point,
            };
        }

        DrawerSwipeReleaseOutcome::Snap {
            snap_point: best.snap_point,
        }
    }

    /// Reverts a rejected/canceled dismissal, restoring position and the pending
    /// snap point.
    pub fn revert_dismiss(&mut self, restore_snap_point: Option<DrawerSnapPoint>) {
        self.swipe_dismissed = false;
        self.swipe_strength = 0.0;
        self.swipe_movement = (0.0, 0.0);
        self.swipe_progress = 0.0;
        if !self.snap_point_controlled {
            self.active_snap_point = restore_snap_point;
        }
    }

    /// Observes the drawer's open value each render; a closed-to-open
    /// transition resets any in-flight gesture and release state. Returns
    /// whether the drawer just opened.
    pub fn observe_open(&mut self, open: bool) -> bool {
        let became_open = open && !self.last_observed_open;
        self.last_observed_open = open;
        if became_open && !self.open_gesture_active() {
            self.reset_on_open();
        }
        became_open
    }

    /// Resets in-flight gesture and release state when the drawer opens.
    pub fn reset_on_open(&mut self) {
        self.gesture = None;
        self.swipe_movement = (0.0, 0.0);
        self.swipe_progress = 0.0;
        self.swipe_strength = 0.0;
        self.swipe_dismissed = false;
        self.dismissal_suppressed = false;
    }

    /// Starts a swipe-to-open gesture from a swipe area toward `direction`,
    /// suppressing outside-press dismissal for the duration of the gesture.
    pub fn begin_open_swipe(
        &mut self,
        x: f32,
        y: f32,
        time_ms: f32,
        direction: DrawerSwipeDirection,
    ) -> bool {
        if self.gesture.is_some() {
            return false;
        }
        self.dismissal_suppressed = true;
        self.gesture = Some(SwipeGesture {
            kind: GestureKind::Open { direction },
            start: (x, y),
            start_ms: time_ms,
            last: (x, y),
            last_ms: time_ms,
            locked_horizontal: None,
            directional: 0.0,
            max_directional: 0.0,
            past_threshold: false,
            canceled: false,
            sample: None,
            sample_start: (x, y),
            sample_start_ms: time_ms,
            base_offset: 0.0,
            requested_open: false,
        });
        true
    }

    /// Whether the open gesture crossed 1px and the drawer should be opened
    /// optimistically. Consumes the request flag.
    pub fn take_open_request(&mut self) -> bool {
        if let Some(gesture) = self.gesture.as_mut() {
            if gesture.requested_open && !gesture.past_threshold {
                gesture.past_threshold = true;
                return true;
            }
        }
        false
    }

    /// Releases the swipe-to-open gesture: keeps the drawer open past 50% of the
    /// popup size (40px fallback when unmeasured) or with release velocity >=
    /// 0.1 px/ms, otherwise a gesture-opened drawer closes again.
    pub fn release_open_swipe(&mut self, release_ms: f32) -> DrawerOpenSwipeRelease {
        let Some(gesture) = self.gesture.take() else {
            self.dismissal_suppressed = false;
            return DrawerOpenSwipeRelease {
                keep_open: false,
                opened_by_gesture: false,
            };
        };
        self.dismissal_suppressed = false;
        self.swipe_movement = (0.0, 0.0);
        self.swipe_progress = 0.0;

        let size = self.size_along_axis();
        let threshold = if size > 0.0 {
            size * 0.5
        } else {
            OPEN_FALLBACK_THRESHOLD
        };
        let velocity = Self::release_velocity(&gesture, release_ms);
        let keep_open = gesture.directional >= threshold || velocity >= OPEN_KEEP_VELOCITY;
        DrawerOpenSwipeRelease {
            keep_open,
            opened_by_gesture: gesture.past_threshold,
        }
    }

    /// Whether outside-press dismissal is currently suppressed by a swipe-area
    /// gesture.
    pub fn dismissal_suppressed(&self) -> bool {
        self.dismissal_suppressed
    }

    /// Marks this drawer as nested inside another drawer.
    pub fn mark_nested(&mut self, nested: bool) {
        self.nested = nested;
    }

    /// Whether this drawer is nested inside another drawer.
    pub fn nested(&self) -> bool {
        self.nested
    }

    /// Records a nested drawer's report: presence (open or transitioning out),
    /// frontmost height, live directional movement, and swipe progress. Nested
    /// swiping only reports after 10px of directional movement.
    pub fn report_nested(
        &mut self,
        id: ElementId,
        present: bool,
        frontmost_height: Option<f32>,
        swiping_movement: f32,
        swipe_progress: f32,
    ) {
        self.nested_reports.retain(|entry| entry.id != id);
        if present {
            self.nested_reports.push(NestedReportEntry {
                id,
                frontmost_height,
                swiping_movement,
                swipe_progress,
            });
        }
    }

    /// Clears a nested drawer's swipe reporting on gesture end/unmount.
    pub fn finish_nested_swipe(&mut self, id: &ElementId) {
        if let Some(entry) = self.nested_reports.iter_mut().find(|entry| entry.id == *id) {
            entry.swiping_movement = 0.0;
            entry.swipe_progress = 0.0;
        }
    }

    /// The number of currently present nested drawers.
    pub fn nested_open_drawer_count(&self) -> usize {
        self.nested_reports.len()
    }

    /// Whether any nested drawer is actively swiping (past 10px of movement).
    pub fn nested_drawer_swiping(&self) -> bool {
        self.nested_reports
            .iter()
            .any(|entry| entry.swiping_movement >= NESTED_SWIPING_THRESHOLD)
    }

    /// The live swipe progress reported by nested drawers.
    pub fn nested_swipe_progress(&self) -> f32 {
        self.nested_reports
            .last()
            .map(|entry| entry.swipe_progress)
            .unwrap_or(0.0)
    }

    /// The deepest open drawer's popup height, falling back to this drawer's own
    /// popup height when no nested drawer reports one.
    pub fn frontmost_height(&self) -> Option<f32> {
        self.nested_reports
            .iter()
            .rev()
            .find_map(|entry| entry.frontmost_height)
            .or(self.popup_height)
    }

    /// The live swipe movement applied to the popup while dragging.
    pub fn swipe_movement(&self) -> (f32, f32) {
        self.swipe_movement
    }

    /// The live swipe progress (0..1 toward dismissal).
    pub fn swipe_progress(&self) -> f32 {
        self.swipe_progress
    }

    /// The directional displacement of the active gesture.
    pub fn gesture_directional_movement(&self) -> f32 {
        self.gesture
            .as_ref()
            .map(|gesture| gesture.directional.abs())
            .unwrap_or(0.0)
    }

    /// Whether the ending phase follows a swipe dismissal.
    pub fn swipe_dismissed(&self) -> bool {
        self.swipe_dismissed
    }

    /// The configured swipe direction.
    pub fn swipe_direction(&self) -> DrawerSwipeDirection {
        self.swipe_direction
    }

    /// Drawer facts merged into the reused dialog popup state by the popup layer.
    pub fn popup_facts(&self) -> DrawerPopupFacts {
        let offset = self.active_snap_offset();
        let signed_offset = match self.swipe_direction {
            DrawerSwipeDirection::Up => -offset,
            _ => offset,
        };
        DrawerPopupFacts {
            expanded: self.expanded(),
            nested: self.nested,
            nested_drawer_count: self.nested_open_drawer_count(),
            nested_drawer_swiping: self.nested_drawer_swiping(),
            nested_swipe_progress: self.nested_swipe_progress(),
            swipe_direction: self.swipe_direction,
            swiping: self.swiping(),
            swipe_movement: Point::new(px(self.swipe_movement.0), px(self.swipe_movement.1)),
            snap_point_offset: px(signed_offset),
            popup_height: self.popup_height.map(px),
            frontmost_height: self.frontmost_height().map(px),
            swipe_strength: self.swipe_strength,
            swipe_dismissed: self.swipe_dismissed,
        }
    }

    /// The backdrop style state derived from the reused dialog backdrop state.
    pub fn backdrop_state(
        &self,
        dialog: DialogBackdropStyleState,
        force_rendered: bool,
    ) -> DrawerBackdropStyleState {
        DrawerBackdropStyleState::from_dialog(
            dialog,
            self.nested,
            force_rendered,
            self.swipe_progress,
            self.frontmost_height().map(px),
            self.swiping(),
            self.swipe_dismissed,
        )
    }

    /// The viewport style state derived from the reused dialog viewport state,
    /// with drawer nested facts replacing the generic dialog nested flag.
    pub fn viewport_state<P: Clone + 'static>(
        &self,
        dialog: DialogViewportStyleState<P>,
    ) -> DrawerViewportStyleState<P> {
        DrawerViewportStyleState::from_dialog(
            dialog,
            self.nested,
            self.nested_open_drawer_count(),
            self.nested_drawer_swiping(),
            self.swiping(),
            self.swipe_progress,
            self.swipe_direction,
        )
    }

    /// The swipe-area style state.
    pub fn swipe_area_state(
        &self,
        open: bool,
        disabled: bool,
        direction: Option<DrawerSwipeDirection>,
    ) -> DrawerSwipeAreaStyleState {
        DrawerSwipeAreaStyleState::new(
            open,
            self.open_gesture_active(),
            direction.unwrap_or_else(|| self.swipe_direction.opposite()),
            disabled,
        )
    }

    /// The content marker style state.
    pub fn content_state(&self, open: bool) -> DrawerContentStyleState {
        DrawerContentStyleState { open }
    }
}

/// Milliseconds since an arbitrary process-local epoch, used by layers to feed
/// gesture timestamps into runtime commands.
pub fn drawer_now_ms() -> f32 {
    use std::sync::OnceLock;
    use std::time::Instant;
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    EPOCH.get_or_init(Instant::now).elapsed().as_secs_f64() as f32 * 1000.0
}
