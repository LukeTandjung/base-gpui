use crate::drawer::{
    DrawerRuntime, DrawerSnapPoint, DrawerSwipeDirection, DrawerSwipeReleaseOutcome,
};

fn snap_runtime(sequential: bool) -> DrawerRuntime {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Down);
    runtime.sync_props(
        DrawerSwipeDirection::Down,
        vec![
            DrawerSnapPoint::Fraction(0.25),
            DrawerSnapPoint::Fraction(0.5),
            DrawerSnapPoint::Fraction(1.0),
        ],
        sequential,
    );
    runtime.set_viewport_height(800.0);
    runtime.set_rem_size(16.0);
    // Popup as tall as the viewport: offsets are 600 / 400 / 0.
    runtime.set_popup_size(300.0, 800.0);
    runtime.reconcile_snap_point(None, None);
    runtime
}

#[test]
fn closest_snap_point_wins_by_drag_distance() {
    let mut runtime = snap_runtime(false);
    // Base offset 600; slow drag up 150 -> target 450, closest offset 400.
    runtime.begin_swipe(0.0, 600.0, 0.0, true, false);
    runtime.move_swipe(0.0, 450.0, 2000.0);

    let outcome = runtime.release_swipe(2000.0);

    assert_eq!(
        outcome,
        DrawerSwipeReleaseOutcome::Snap {
            snap_point: DrawerSnapPoint::Fraction(0.5)
        }
    );
}

#[test]
fn velocity_offset_skips_to_a_further_snap_point() {
    let mut runtime = snap_runtime(false);
    // Fast upward drag of only 150px: velocity extends the target past 400 to 0.
    runtime.begin_swipe(0.0, 600.0, 0.0, true, false);
    runtime.move_swipe(0.0, 450.0, 50.0);

    let outcome = runtime.release_swipe(50.0);

    assert_eq!(
        outcome,
        DrawerSwipeReleaseOutcome::Snap {
            snap_point: DrawerSnapPoint::Fraction(1.0)
        }
    );
}

#[test]
fn contradicting_release_velocity_falls_back_to_gesture_velocity() {
    let mut runtime = snap_runtime(false);
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    // Overall slow downward drag of 100px, but the last sample moves upward.
    runtime.move_swipe(0.0, 120.0, 500.0);
    runtime.move_swipe(0.0, 100.0, 540.0);

    let outcome = runtime.release_swipe(550.0);

    // With the raw -0.5 px/ms sample the velocity skip would target offset 0;
    // the overall-velocity fallback keeps the closest point instead.
    assert_eq!(
        outcome,
        DrawerSwipeReleaseOutcome::Snap {
            snap_point: DrawerSnapPoint::Fraction(0.25)
        }
    );
}

#[test]
fn sequential_mode_advances_at_most_one_adjacent_point() {
    let mut runtime = snap_runtime(true);
    // Base offset 600; large upward drag targeting offset 100 may only advance
    // to the adjacent 400 offset.
    runtime.begin_swipe(0.0, 600.0, 0.0, true, false);
    runtime.move_swipe(0.0, 100.0, 2000.0);

    let outcome = runtime.release_swipe(2000.0);

    assert_eq!(
        outcome,
        DrawerSwipeReleaseOutcome::Snap {
            snap_point: DrawerSnapPoint::Fraction(0.5)
        }
    );
}

#[test]
fn sequential_mode_past_last_point_toward_dismissal_closes() {
    let mut runtime = snap_runtime(true);
    // Base offset 600 is the last point before close; dragging further down
    // dismisses.
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    runtime.move_swipe(0.0, 120.0, 2000.0);

    let outcome = runtime.release_swipe(2000.0);

    assert!(matches!(outcome, DrawerSwipeReleaseOutcome::Dismiss { .. }));
}

#[test]
fn target_closer_to_close_offset_than_any_snap_point_closes() {
    let mut runtime = snap_runtime(false);
    // Base offset 600; slow drag down 150 -> target 750, closer to the 800
    // close offset than to 600.
    runtime.begin_swipe(0.0, 0.0, 0.0, true, false);
    runtime.move_swipe(0.0, 150.0, 2000.0);

    let outcome = runtime.release_swipe(2000.0);

    assert!(matches!(outcome, DrawerSwipeReleaseOutcome::Dismiss { .. }));
}

#[test]
fn accepted_close_resets_snap_point_and_reverted_close_restores_it() {
    let mut runtime = snap_runtime(false);
    runtime.set_snap_point_uncontrolled(Some(DrawerSnapPoint::Fraction(0.5)));

    // Accepted close: root resets to the resolved default.
    runtime.reset_snap_point_to_default(None);
    assert_eq!(
        runtime.snap_point_value(),
        Some(DrawerSnapPoint::Fraction(0.25))
    );

    // Rejected close: the pre-close snap point is restored.
    runtime.revert_dismiss(Some(DrawerSnapPoint::Fraction(0.5)));
    assert_eq!(
        runtime.snap_point_value(),
        Some(DrawerSnapPoint::Fraction(0.5))
    );
}

#[test]
fn controlled_snap_point_does_not_self_mutate_on_release() {
    let mut runtime = snap_runtime(false);
    runtime.reconcile_snap_point(Some(Some(DrawerSnapPoint::Fraction(0.5))), None);
    runtime.begin_swipe(0.0, 600.0, 0.0, true, false);
    runtime.move_swipe(0.0, 450.0, 2000.0);

    let _outcome = runtime.release_swipe(2000.0);

    assert_eq!(
        runtime.snap_point_value(),
        Some(DrawerSnapPoint::Fraction(0.5))
    );
}
