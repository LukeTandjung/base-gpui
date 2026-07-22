use std::time::Instant;

use crate::toast::{ToastId, ToastOptions, ToastRuntime};

fn add(runtime: &mut ToastRuntime<()>, name: &str) -> ToastId {
    runtime
        .add_toast(
            ToastOptions::new()
                .id(ToastId::new(name.to_owned()))
                .title(name.to_owned()),
            Instant::now(),
        )
        .id
}

#[test]
fn new_toasts_are_prepended_newest_first() {
    let mut runtime = ToastRuntime::<()>::new();
    let first = add(&mut runtime, "first");
    let second = add(&mut runtime, "second");
    let facts = runtime.toasts();
    assert_eq!(facts[0].id, second);
    assert_eq!(facts[1].id, first);
}

#[test]
fn visible_index_and_offset_track_heights_in_queue_order() {
    let mut runtime = ToastRuntime::<()>::new();
    let first = add(&mut runtime, "first");
    let second = add(&mut runtime, "second");
    runtime.set_toast_height(&first, 40.0);
    runtime.set_toast_height(&second, 60.0);

    let newest = runtime.root_state(&second);
    assert_eq!(newest.visible_index, Some(0));
    assert_eq!(newest.offset_y, 0.0);
    let older = runtime.root_state(&first);
    assert_eq!(older.visible_index, Some(1));
    assert_eq!(older.offset_y, 60.0);
    assert_eq!(runtime.viewport_state().frontmost_height, 60.0);
}

#[test]
fn ending_toasts_lose_visible_index_and_contribute_zero_height() {
    let mut runtime = ToastRuntime::<()>::new();
    let first = add(&mut runtime, "first");
    let second = add(&mut runtime, "second");
    runtime.set_toast_height(&first, 40.0);
    runtime.set_toast_height(&second, 60.0);

    runtime.close_toast(Some(&second));
    let ending = runtime.root_state(&second);
    assert_eq!(ending.visible_index, None);
    // Dom index while ending.
    assert_eq!(ending.index, Some(0));
    let live = runtime.root_state(&first);
    assert_eq!(live.visible_index, Some(0));
    assert_eq!(live.offset_y, 0.0);
    assert_eq!(runtime.viewport_state().frontmost_height, 40.0);
}

#[test]
fn first_height_measurement_clears_starting_transition() {
    let mut runtime = ToastRuntime::<()>::new();
    let id = add(&mut runtime, "toast");
    assert_eq!(
        runtime.root_state(&id).transition_status,
        Some(crate::toast::ToastTransitionStatus::Starting)
    );
    assert!(runtime.set_toast_height(&id, 32.0));
    assert_eq!(runtime.root_state(&id).transition_status, None);
    assert_eq!(runtime.root_state(&id).height, 32.0);
}
