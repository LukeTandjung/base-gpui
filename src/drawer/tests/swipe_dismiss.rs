use gpui::ElementId;

use crate::drawer::{
    DrawerRuntime, DrawerSnapPoint, DrawerSwipeDirection, DrawerSwipeReleaseOutcome,
};

fn down_runtime() -> DrawerRuntime {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Down);
    runtime.sync_props(DrawerSwipeDirection::Down, Vec::new(), false);
    runtime.set_viewport_height(800.0);
    runtime.set_rem_size(16.0);
    runtime.set_popup_size(300.0, 400.0);
    runtime
}

#[test]
fn drag_past_threshold_dismisses() {
    let mut runtime = down_runtime();
    assert!(runtime.begin_swipe(0.0, 0.0, 0.0, true, false));
    runtime.move_swipe(0.0, 250.0, 1000.0);

    let outcome = runtime.release_swipe(1000.0);

    assert!(matches!(outcome, DrawerSwipeReleaseOutcome::Dismiss { .. }));
    assert!(runtime.swipe_dismissed());
}

#[test]
fn fast_flick_dismisses_below_distance_threshold() {
    let mut runtime = down_runtime();
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    // 50px in 20ms = 2.5 px/ms, far above the 0.5 px/ms velocity threshold.
    runtime.move_swipe(0.0, 50.0, 20.0);

    let outcome = runtime.release_swipe(30.0);

    assert!(matches!(outcome, DrawerSwipeReleaseOutcome::Dismiss { .. }));
}

#[test]
fn drag_below_threshold_releases_back_to_rest() {
    let mut runtime = down_runtime();
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    // 50px over a full second: slow and short of the 200px threshold.
    runtime.move_swipe(0.0, 50.0, 1000.0);

    let outcome = runtime.release_swipe(1000.0);

    assert_eq!(outcome, DrawerSwipeReleaseOutcome::Rest);
    assert!(!runtime.swipe_dismissed());
}

#[test]
fn reversal_from_max_displacement_cancels_dismissal() {
    let mut runtime = down_runtime();
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    runtime.move_swipe(0.0, 250.0, 100.0);
    // Back off 15px from the max displacement (>= 10px reverse threshold).
    runtime.move_swipe(0.0, 235.0, 200.0);

    let outcome = runtime.release_swipe(200.0);

    assert_eq!(outcome, DrawerSwipeReleaseOutcome::Rest);
}

#[test]
fn disallowed_direction_produces_sqrt_damped_movement_and_no_dismissal() {
    let mut runtime = down_runtime();
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    // Dragging a bottom drawer upward: 100px maps to sqrt-damped -10px.
    runtime.move_swipe(0.0, -100.0, 100.0);

    assert_eq!(runtime.swipe_movement(), (0.0, -10.0));
    assert_eq!(
        runtime.release_swipe(100.0),
        DrawerSwipeReleaseOutcome::Rest
    );
}

#[test]
fn canceled_swipe_close_restores_position_and_pending_snap_point() {
    let mut runtime = down_runtime();
    runtime.sync_props(
        DrawerSwipeDirection::Down,
        vec![DrawerSnapPoint::Fraction(0.25)],
        false,
    );
    runtime.reconcile_snap_point(None, None);
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    runtime.move_swipe(0.0, 250.0, 20.0);
    let outcome = runtime.release_swipe(30.0);
    let DrawerSwipeReleaseOutcome::Dismiss {
        restore_snap_point, ..
    } = outcome
    else {
        panic!("expected dismissal");
    };

    runtime.revert_dismiss(restore_snap_point);

    assert!(!runtime.swipe_dismissed());
    assert_eq!(runtime.swipe_movement(), (0.0, 0.0));
    assert_eq!(
        runtime.snap_point_value(),
        Some(DrawerSnapPoint::Fraction(0.25))
    );
}

#[test]
fn release_strength_maps_to_clamp_boundaries() {
    // Duration clamps to the 80ms floor -> minimum strength 0.1.
    assert_eq!(DrawerRuntime::release_strength(0.0, 4.0), 0.1);
    // Duration clamps to the 360ms ceiling -> maximum strength 1.0.
    assert_eq!(DrawerRuntime::release_strength(10_000.0, 0.01), 1.0);
    // Velocity clamps to 0.2..4 px/ms.
    let strength = DrawerRuntime::release_strength(400.0, 100.0);
    assert!(strength > 0.1 && strength < 1.0);
}

#[test]
fn press_over_content_does_not_start_a_swipe() {
    let mut runtime = down_runtime();
    assert!(!runtime.begin_swipe(0.0, 0.0, 0.0, true, true));
    assert!(!runtime.swiping());
}

#[test]
fn non_primary_button_does_not_start_and_cancel_restores() {
    let mut runtime = down_runtime();
    assert!(!runtime.begin_swipe(0.0, 0.0, 0.0, false, false));

    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    runtime.move_swipe(0.0, 100.0, 50.0);
    runtime.cancel_swipe();

    assert!(!runtime.swiping());
    assert_eq!(runtime.swipe_movement(), (0.0, 0.0));
    assert_eq!(runtime.swipe_progress(), 0.0);
}

#[test]
fn gesture_is_inert_while_a_nested_drawer_is_open() {
    let mut runtime = down_runtime();
    runtime.report_nested(ElementId::from("nested"), true, None, 0.0, 0.0);

    assert!(!runtime.begin_swipe(0.0, 0.0, 0.0, true, false));
}

#[test]
fn opening_resets_in_flight_gesture_and_release_state() {
    let mut runtime = down_runtime();
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    runtime.move_swipe(0.0, 250.0, 20.0);
    runtime.release_swipe(30.0);
    assert!(runtime.swipe_dismissed());

    assert!(runtime.observe_open(true));

    assert!(!runtime.swiping());
    assert!(!runtime.swipe_dismissed());
}
