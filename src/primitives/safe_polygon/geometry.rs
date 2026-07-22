//! Pure safe-polygon geometry over `gpui::{Point, Pixels, Bounds}`.
//!
//! Translated from Base UI's vendored floating-ui-react `safePolygon.ts`,
//! with DOM rect access replaced by `Bounds<Pixels>` edge queries.

use gpui::{point, px, Bounds, Pixels, Point};

use super::tracker::SafePolygonSide;

/// Rounding tolerance used by the trough insets and the opposite-side test.
const ROUNDING_TOLERANCE: Pixels = px(1.0);

fn edge_crosses(p: Point<Pixels>, a: Point<Pixels>, b: Point<Pixels>) -> bool {
    ((a.y >= p.y) != (b.y >= p.y)) && p.x <= (b.x - a.x) * ((p.y - a.y) / (b.y - a.y)) + a.x
}

/// Ray-casting edge-crossing containment test for an arbitrary quadrilateral.
///
/// Correct for the (possibly non-convex) vertex orderings produced by
/// [`safe_polygon_quadrilateral`]'s corner selection.
pub fn point_in_quadrilateral(p: Point<Pixels>, quad: [Point<Pixels>; 4]) -> bool {
    let mut inside = false;
    for index in 0..4 {
        if edge_crosses(p, quad[index], quad[(index + 1) % 4]) {
            inside = !inside;
        }
    }
    inside
}

fn in_axis_aligned_rect(p: Point<Pixels>, x1: Pixels, y1: Pixels, x2: Pixels, y2: Pixels) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
}

/// Whether the pointer sits in the axis-aligned trough between the trigger's
/// and popup's near edges, clamped on the cross axis to the narrower/shorter
/// of the two boxes, with a 1px inset into each box for rounding error.
///
/// Inside the trough always counts as safe, without any velocity check. This
/// is also the standalone lower-fidelity fallback (the tooltip's
/// `point_in_safe_gap` approximation): a consumer can use this test alone with
/// an extended close delay instead of the full tracker.
pub fn point_in_trough(
    p: Point<Pixels>,
    trigger: Bounds<Pixels>,
    popup: Bounds<Pixels>,
    side: SafePolygonSide,
) -> bool {
    let wider = popup.size.width > trigger.size.width;
    let taller = popup.size.height > trigger.size.height;
    let narrow = if wider { trigger } else { popup };
    let short = if taller { trigger } else { popup };
    let left = narrow.left();
    let right = narrow.right();
    let top = short.top();
    let bottom = short.bottom();
    let one = ROUNDING_TOLERANCE;

    match side {
        SafePolygonSide::Top => {
            in_axis_aligned_rect(p, left, trigger.top() + one, right, popup.bottom() - one)
        }
        SafePolygonSide::Bottom => {
            in_axis_aligned_rect(p, left, popup.top() + one, right, trigger.bottom() - one)
        }
        SafePolygonSide::Left => {
            in_axis_aligned_rect(p, popup.right() - one, bottom, trigger.left() + one, top)
        }
        SafePolygonSide::Right => {
            in_axis_aligned_rect(p, trigger.right() - one, bottom, popup.left() + one, top)
        }
    }
}

/// Whether the recorded exit point lies on the far side of the trigger from
/// the popup (e.g. popup on the right, pointer exited past the trigger's left
/// half). In that case the buffer logic would create a spurious keep-open
/// region, so the polygon never applies. Uses a 1px rounding tolerance.
pub fn exit_is_opposite_side(
    exit: Point<Pixels>,
    trigger: Bounds<Pixels>,
    side: SafePolygonSide,
) -> bool {
    let one = ROUNDING_TOLERANCE;
    match side {
        SafePolygonSide::Top => exit.y >= trigger.bottom() - one,
        SafePolygonSide::Bottom => exit.y <= trigger.top() + one,
        SafePolygonSide::Left => exit.x >= trigger.right() - one,
        SafePolygonSide::Right => exit.x <= trigger.left() + one,
    }
}

/// Builds the safe quadrilateral for a given exit point, trigger/popup bounds,
/// and side: two buffer-offset points near the exit position (spread depending
/// on whether the popup is wider/taller than the trigger and which half the
/// cursor exited from) connected to the popup's near-edge corners.
pub fn safe_polygon_quadrilateral(
    exit: Point<Pixels>,
    trigger: Bounds<Pixels>,
    popup: Bounds<Pixels>,
    side: SafePolygonSide,
    buffer: Pixels,
) -> [Point<Pixels>; 4] {
    let one = ROUNDING_TOLERANCE;
    match side {
        SafePolygonSide::Top | SafePolygonSide::Bottom => {
            let wider = popup.size.width > trigger.size.width;
            let from_right = exit.x > popup.right() - popup.size.width * 0.5;
            let offset = if wider { buffer * 0.5 } else { buffer * 4.0 };
            let cursor_one_x = if wider || from_right {
                exit.x + offset
            } else {
                exit.x - offset
            };
            let cursor_two_x = if wider { exit.x - offset } else { cursor_one_x };
            let (cursor_y, near, far) = match side {
                SafePolygonSide::Top => {
                    (exit.y + buffer + one, popup.bottom() - buffer, popup.top())
                }
                _ => (exit.y - buffer, popup.top() + buffer, popup.bottom()),
            };
            let common_y_left = if from_right || wider { near } else { far };
            let common_y_right = if from_right && !wider { far } else { near };
            [
                point(cursor_one_x, cursor_y),
                point(cursor_two_x, cursor_y),
                point(popup.left(), common_y_left),
                point(popup.right(), common_y_right),
            ]
        }
        SafePolygonSide::Left | SafePolygonSide::Right => {
            let taller = popup.size.height > trigger.size.height;
            let from_bottom = exit.y > popup.bottom() - popup.size.height * 0.5;
            let offset = if taller { buffer * 0.5 } else { buffer * 4.0 };
            let cursor_one_y = if taller || from_bottom {
                exit.y + offset
            } else {
                exit.y - offset
            };
            let cursor_two_y = if taller {
                exit.y - offset
            } else {
                cursor_one_y
            };
            let (cursor_x, near, far) = match side {
                SafePolygonSide::Left => {
                    (exit.x + buffer + one, popup.right() - buffer, popup.left())
                }
                _ => (exit.x - buffer, popup.left() + buffer, popup.right()),
            };
            let common_x_top = if from_bottom || taller { near } else { far };
            let common_x_bottom = if from_bottom && !taller { far } else { near };
            match side {
                SafePolygonSide::Left => [
                    point(common_x_top, popup.top()),
                    point(common_x_bottom, popup.bottom()),
                    point(cursor_x, cursor_one_y),
                    point(cursor_x, cursor_two_y),
                ],
                _ => [
                    point(cursor_x, cursor_one_y),
                    point(cursor_x, cursor_two_y),
                    point(common_x_top, popup.top()),
                    point(common_x_bottom, popup.bottom()),
                ],
            }
        }
    }
}
