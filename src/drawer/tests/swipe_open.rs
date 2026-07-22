use crate::drawer::{DrawerRuntime, DrawerSwipeDirection};

fn down_runtime() -> DrawerRuntime {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Down);
    runtime.sync_props(DrawerSwipeDirection::Down, Vec::new(), false);
    runtime.set_viewport_height(800.0);
    runtime.set_popup_size(300.0, 400.0);
    runtime
}

#[test]
fn drag_in_open_direction_requests_open_after_one_pixel() {
    let mut runtime = down_runtime();
    assert!(runtime.begin_open_swipe(0.0, 500.0, 0.0, DrawerSwipeDirection::Up));
    runtime.move_swipe(0.0, 480.0, 20.0);

    assert!(runtime.take_open_request());
    // The request is consumed once.
    assert!(!runtime.take_open_request());
}

#[test]
fn release_below_thresholds_closes_a_gesture_opened_drawer() {
    let mut runtime = down_runtime();
    runtime.begin_open_swipe(0.0, 500.0, 0.0, DrawerSwipeDirection::Up);
    // 5px over 500ms: below 50% of the 400px popup and below 0.1 px/ms.
    runtime.move_swipe(0.0, 495.0, 500.0);
    assert!(runtime.take_open_request());

    let release = runtime.release_open_swipe(500.0);

    assert!(!release.keep_open);
    assert!(release.opened_by_gesture);
}

#[test]
fn release_past_half_popup_size_keeps_open() {
    let mut runtime = down_runtime();
    runtime.begin_open_swipe(0.0, 500.0, 0.0, DrawerSwipeDirection::Up);
    runtime.move_swipe(0.0, 280.0, 2000.0);

    let release = runtime.release_open_swipe(2000.0);

    assert!(release.keep_open);
}

#[test]
fn fast_release_velocity_keeps_open() {
    let mut runtime = down_runtime();
    runtime.begin_open_swipe(0.0, 500.0, 0.0, DrawerSwipeDirection::Up);
    // 40px in 100ms = 0.4 px/ms >= 0.1 px/ms.
    runtime.move_swipe(0.0, 460.0, 100.0);

    let release = runtime.release_open_swipe(100.0);

    assert!(release.keep_open);
}

#[test]
fn dismissal_is_suppressed_during_the_gesture_and_reenabled_after() {
    let mut runtime = down_runtime();
    assert!(!runtime.dismissal_suppressed());

    runtime.begin_open_swipe(0.0, 500.0, 0.0, DrawerSwipeDirection::Up);
    assert!(runtime.dismissal_suppressed());

    runtime.release_open_swipe(100.0);
    assert!(!runtime.dismissal_suppressed());
}

#[test]
fn canceling_a_mid_gesture_swipe_area_resets_state_and_reenables_dismissal() {
    let mut runtime = down_runtime();
    runtime.begin_open_swipe(0.0, 500.0, 0.0, DrawerSwipeDirection::Up);
    runtime.move_swipe(0.0, 450.0, 100.0);

    runtime.cancel_swipe();

    assert!(!runtime.swiping());
    assert_eq!(runtime.swipe_movement(), (0.0, 0.0));
    assert!(!runtime.dismissal_suppressed());
}
