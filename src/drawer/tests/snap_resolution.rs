use gpui::{px, Rems};

use crate::drawer::{
    closest_resolved_snap_point, resolve_snap_points, snap_point_swipe_offset, DrawerRuntime,
    DrawerSnapPoint, DrawerSwipeDirection,
};

#[test]
fn fractions_resolve_against_viewport_height_and_clamp() {
    let resolved = resolve_snap_points(&[DrawerSnapPoint::Fraction(0.5)], 800.0, 1000.0, 16.0);
    assert_eq!(resolved[0].height, 400.0);
    assert_eq!(resolved[0].offset, 600.0);

    let clamped = resolve_snap_points(&[DrawerSnapPoint::Fraction(1.5)], 800.0, 1000.0, 16.0);
    assert_eq!(clamped[0].height, 800.0);
}

#[test]
fn pixel_and_rem_values_resolve_and_clamp_to_min_popup_viewport() {
    let resolved = resolve_snap_points(
        &[
            DrawerSnapPoint::Px(px(500.0)),
            DrawerSnapPoint::Rems(Rems(10.0)),
        ],
        800.0,
        400.0,
        16.0,
    );
    // 500px clamps to min(popup, viewport) = 400.
    assert_eq!(resolved[0].height, 400.0);
    assert_eq!(resolved[0].offset, 0.0);
    // 10rem * 16 = 160.
    assert_eq!(resolved[1].height, 160.0);
    assert_eq!(resolved[1].offset, 240.0);
}

#[test]
fn unresolvable_values_skip_and_near_duplicates_dedupe_keeping_later() {
    let resolved = resolve_snap_points(
        &[
            DrawerSnapPoint::Fraction(f32::NAN),
            DrawerSnapPoint::Px(px(200.0)),
            DrawerSnapPoint::Px(px(200.5)),
            DrawerSnapPoint::Fraction(0.25),
        ],
        800.0,
        1000.0,
        16.0,
    );
    assert_eq!(resolved.len(), 1);
    assert_eq!(resolved[0].snap_point, DrawerSnapPoint::Fraction(0.25));
    assert_eq!(resolved[0].height, 200.0);
}

#[test]
fn default_snap_point_falls_back_to_first_entry() {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Down);
    runtime.sync_props(
        DrawerSwipeDirection::Down,
        vec![
            DrawerSnapPoint::Fraction(0.25),
            DrawerSnapPoint::Fraction(0.5),
        ],
        false,
    );

    runtime.reconcile_snap_point(None, None);
    assert_eq!(
        runtime.snap_point_value(),
        Some(DrawerSnapPoint::Fraction(0.25))
    );
}

#[test]
fn missing_uncontrolled_active_value_resolves_to_default() {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Down);
    runtime.sync_props(
        DrawerSwipeDirection::Down,
        vec![
            DrawerSnapPoint::Fraction(0.25),
            DrawerSnapPoint::Fraction(0.5),
        ],
        false,
    );
    runtime.set_snap_point_uncontrolled(Some(DrawerSnapPoint::Fraction(0.9)));

    runtime.reconcile_snap_point(None, None);

    assert_eq!(
        runtime.snap_point_value(),
        Some(DrawerSnapPoint::Fraction(0.25))
    );
}

#[test]
fn controlled_value_not_in_snap_points_resolves_to_closest_point() {
    let resolved = resolve_snap_points(
        &[
            DrawerSnapPoint::Px(px(200.0)),
            DrawerSnapPoint::Px(px(400.0)),
        ],
        800.0,
        1000.0,
        16.0,
    );

    let closest =
        closest_resolved_snap_point(&resolved, &DrawerSnapPoint::Px(px(390.0)), 800.0, 16.0)
            .unwrap();

    assert_eq!(closest.snap_point, DrawerSnapPoint::Px(px(400.0)));
}

#[test]
fn snap_point_swipe_offset_applies_sqrt_overshoot_damping() {
    // In-range: tracks linearly.
    assert_eq!(snap_point_swipe_offset(200.0, -100.0), 100.0);
    // Overshoot beyond fully open: -sqrt(-next_offset).
    assert_eq!(snap_point_swipe_offset(200.0, -300.0), -10.0);
}

#[test]
fn horizontal_drawers_ignore_snap_points() {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Right);
    runtime.sync_props(
        DrawerSwipeDirection::Right,
        vec![DrawerSnapPoint::Fraction(0.5)],
        false,
    );
    runtime.set_viewport_height(800.0);
    runtime.set_popup_size(300.0, 400.0);

    assert!(runtime.resolved_snap_points().is_empty());
    assert_eq!(runtime.active_snap_offset(), 0.0);
}
