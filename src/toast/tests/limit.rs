use std::time::{Duration, Instant};

use crate::toast::{ToastId, ToastOptions, ToastRuntime};

fn add(runtime: &mut ToastRuntime<()>, name: &str) -> ToastId {
    runtime
        .add_toast(
            ToastOptions::new().id(ToastId::new(name.to_owned())),
            Instant::now(),
        )
        .id
}

#[test]
fn toasts_beyond_limit_are_flagged_limited_not_removed() {
    let mut runtime = ToastRuntime::<()>::new();
    let first = add(&mut runtime, "1");
    add(&mut runtime, "2");
    add(&mut runtime, "3");
    add(&mut runtime, "4");

    let facts = runtime.toasts();
    assert_eq!(facts.len(), 4);
    // Oldest active toast beyond the limit of 3 is limited.
    assert!(runtime.root_state(&first).limited);
    assert!(!runtime.root_state(&ToastId::new("4")).limited);
    assert!(!runtime.root_state(&ToastId::new("2")).limited);
}

#[test]
fn limited_flags_recompute_on_close() {
    let mut runtime = ToastRuntime::<()>::new();
    let first = add(&mut runtime, "1");
    add(&mut runtime, "2");
    add(&mut runtime, "3");
    let newest = add(&mut runtime, "4");
    assert!(runtime.root_state(&first).limited);

    runtime.close_toast(Some(&newest));
    assert!(!runtime.root_state(&first).limited);
}

#[test]
fn ending_toasts_are_excluded_from_the_limit_count() {
    let mut runtime = ToastRuntime::<()>::new();
    add(&mut runtime, "1");
    add(&mut runtime, "2");
    add(&mut runtime, "3");
    let newest = add(&mut runtime, "4");
    runtime.close_toast(Some(&newest));
    // Only three active toasts remain: none limited, ending one unflagged.
    assert!(runtime.toasts().iter().all(|facts| !facts.limited));
}

#[test]
fn limit_change_recomputes_flags() {
    let mut runtime = ToastRuntime::<()>::new();
    let first = add(&mut runtime, "1");
    add(&mut runtime, "2");
    add(&mut runtime, "3");
    assert!(!runtime.root_state(&first).limited);

    runtime.sync_provider_props(Duration::from_millis(5000), 2);
    assert!(runtime.root_state(&first).limited);

    runtime.sync_provider_props(Duration::from_millis(5000), 3);
    assert!(!runtime.root_state(&first).limited);
}
