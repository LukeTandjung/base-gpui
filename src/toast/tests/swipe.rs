use std::time::Instant;

use crate::toast::{ToastId, ToastOptions, ToastRuntime, ToastSwipeDirection};

fn runtime_with_toast() -> (ToastRuntime<()>, ToastId) {
    let mut runtime = ToastRuntime::<()>::new();
    let id = runtime
        .add_toast(
            ToastOptions::new().id(ToastId::new("swipe")),
            Instant::now(),
        )
        .id;
    (runtime, id)
}

#[test]
fn empty_direction_set_disables_swiping() {
    let (mut runtime, id) = runtime_with_toast();
    assert!(!runtime.begin_swipe(&id, 0.0, 0.0, Vec::new()));
}

#[test]
fn dominant_axis_locks_the_drag_direction() {
    let (mut runtime, id) = runtime_with_toast();
    assert!(runtime.begin_swipe(
        &id,
        0.0,
        0.0,
        vec![ToastSwipeDirection::Down, ToastSwipeDirection::Right]
    ));
    runtime.move_swipe(10.0, 3.0);
    let state = runtime.root_state(&id);
    assert!(state.swiping);
    assert_eq!(state.swipe_direction, Some(ToastSwipeDirection::Right));
    // Permitted axis moves 1:1; cross axis is damped.
    assert_eq!(state.swipe_movement_x, 10.0);
    assert!((state.swipe_movement_y - 3.0_f32.sqrt()).abs() < 0.001);
}

#[test]
fn non_permitted_direction_is_damped() {
    let (mut runtime, id) = runtime_with_toast();
    runtime.begin_swipe(&id, 0.0, 0.0, vec![ToastSwipeDirection::Down]);
    // Move up (not permitted on the locked vertical axis).
    runtime.move_swipe(0.0, -25.0);
    let state = runtime.root_state(&id);
    assert!((state.swipe_movement_y + 5.0).abs() < 0.001);
    assert_eq!(state.swipe_direction, None);
}

#[test]
fn release_past_threshold_dismisses_with_direction() {
    let (mut runtime, id) = runtime_with_toast();
    runtime.begin_swipe(&id, 0.0, 0.0, vec![ToastSwipeDirection::Down]);
    runtime.move_swipe(0.0, 50.0);
    let release = runtime.release_swipe().expect("gesture active");
    assert!(release.dismiss);
    assert_eq!(release.direction, Some(ToastSwipeDirection::Down));
}

#[test]
fn release_below_threshold_springs_back() {
    let (mut runtime, id) = runtime_with_toast();
    runtime.begin_swipe(&id, 0.0, 0.0, vec![ToastSwipeDirection::Down]);
    runtime.move_swipe(0.0, 20.0);
    let release = runtime.release_swipe().expect("gesture active");
    assert!(!release.dismiss);
    assert_eq!(release.direction, None);
    let state = runtime.root_state(&id);
    assert!(!state.swiping);
    assert_eq!(state.swipe_movement_y, 0.0);
}

#[test]
fn reverse_cancel_holds_on_release() {
    let (mut runtime, id) = runtime_with_toast();
    runtime.begin_swipe(&id, 0.0, 0.0, vec![ToastSwipeDirection::Down]);
    runtime.move_swipe(0.0, 60.0);
    // Move back toward origin past the 10px reverse-cancel threshold.
    runtime.move_swipe(0.0, 45.0);
    let release = runtime.release_swipe().expect("gesture active");
    assert!(!release.dismiss);
}

#[test]
fn swipe_refused_for_limited_and_ending_toasts() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let oldest = runtime
        .add_toast(ToastOptions::new().id(ToastId::new("old")), start)
        .id;
    for index in 0..3 {
        runtime.add_toast(
            ToastOptions::new().id(ToastId::new(format!("t{index}"))),
            start,
        );
    }
    assert!(runtime.root_state(&oldest).limited);
    assert!(!runtime.begin_swipe(&oldest, 0.0, 0.0, vec![ToastSwipeDirection::Down]));

    let ending = ToastId::new("t0");
    runtime.close_toast(Some(&ending));
    assert!(!runtime.begin_swipe(&ending, 0.0, 0.0, vec![ToastSwipeDirection::Down]));
}
