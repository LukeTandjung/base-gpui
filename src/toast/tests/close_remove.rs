use std::time::Instant;

use crate::toast::{ToastId, ToastOptions, ToastRuntime};

#[test]
fn close_none_closes_all_and_clears_timers() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let first = runtime.add_toast(ToastOptions::new(), start).id;
    let second = runtime.add_toast(ToastOptions::new(), start).id;

    let outcome = runtime.close_toast(None);
    assert_eq!(outcome.closed.len(), 2);
    assert_eq!(outcome.timer_ops.len(), 2);
    assert_eq!(runtime.remaining_timeout(&first), None);
    assert_eq!(runtime.remaining_timeout(&second), None);
}

#[test]
fn on_close_fires_once_and_not_again_for_ending_toasts() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = runtime
        .add_toast(ToastOptions::new().on_close(|_| {}), start)
        .id;

    let first = runtime.close_toast(Some(&id));
    assert_eq!(first.on_close.len(), 1);
    let second = runtime.close_toast(Some(&id));
    assert!(second.on_close.is_empty());
    assert!(second.closed.is_empty());
}

#[test]
fn on_remove_returned_at_removal_not_close() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = runtime
        .add_toast(ToastOptions::new().on_remove(|_| {}), start)
        .id;
    runtime.close_toast(Some(&id));
    assert_eq!(runtime.toasts().len(), 1);
    let on_remove = runtime.remove_toast(&id);
    assert!(on_remove.is_some());
    assert!(runtime.toasts().is_empty());
}

#[test]
fn upsert_over_ending_skips_on_remove() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(ToastOptions::new().id(id.clone()).on_remove(|_| {}), start);
    runtime.close_toast(Some(&id));
    // The silent removal happens inside add; no on_remove is surfaced.
    let outcome = runtime.add_toast(ToastOptions::new().id(id.clone()), start);
    assert_eq!(outcome.id, id);
    assert_eq!(runtime.toasts().len(), 1);
}

#[test]
fn hover_and_focus_facts_reset_when_no_active_toasts_remain() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = runtime.add_toast(ToastOptions::new(), start).id;
    runtime.set_hovering(true, start);
    runtime.set_focused(true, start);
    assert!(runtime.viewport_state().expanded);

    runtime.close_toast(Some(&id));
    assert!(!runtime.viewport_state().expanded);
}
