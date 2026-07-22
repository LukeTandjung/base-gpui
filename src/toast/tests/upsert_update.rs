use std::time::{Duration, Instant};

use crate::toast::{ToastId, ToastOptions, ToastRuntime, ToastType};

#[test]
fn upsert_by_id_resets_timer_increments_generation_keeps_position() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    runtime.add_toast(ToastOptions::new().id(ToastId::new("a")), start);
    runtime.add_toast(ToastOptions::new().id(ToastId::new("b")), start);

    let id = ToastId::new("a");
    let outcome = runtime.add_toast(
        ToastOptions::new().id(id.clone()).title("updated"),
        start + Duration::from_millis(1000),
    );
    assert_eq!(outcome.id, id);
    assert_eq!(runtime.update_generation(&id), Some(1));
    // Position unchanged: "b" is still newest.
    assert_eq!(runtime.toasts()[0].id, ToastId::new("b"));
    assert_eq!(runtime.toasts()[1].title.as_deref(), Some("updated"));
    // Timer reset to the full duration.
    assert_eq!(
        runtime.remaining_timeout(&id),
        Some(Duration::from_millis(5000))
    );
}

#[test]
fn upsert_over_ending_toast_removes_silently_and_adds_fresh() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(ToastOptions::new().id(id.clone()), start);
    runtime.close_toast(Some(&id));

    let outcome = runtime.add_toast(ToastOptions::new().id(id.clone()), start);
    assert_eq!(outcome.id, id);
    let facts = runtime.toasts();
    assert_eq!(facts.len(), 1);
    assert!(!facts[0].ending);
    assert_eq!(runtime.update_generation(&id), Some(0));
}

#[test]
fn update_on_ending_toast_is_ignored() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(ToastOptions::new().id(id.clone()).title("before"), start);
    runtime.close_toast(Some(&id));
    let ops = runtime.update_toast(&id, ToastOptions::new().title("after"), start);
    assert!(ops.is_empty());
    assert_eq!(runtime.toasts()[0].title.as_deref(), Some("before"));
}

#[test]
fn update_applies_partial_fields_and_increments_generation() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(
        ToastOptions::new()
            .id(id.clone())
            .title("title")
            .description("desc"),
        start,
    );
    runtime.update_toast(&id, ToastOptions::new().title("new title"), start);
    let facts = &runtime.toasts()[0];
    assert_eq!(facts.title.as_deref(), Some("new title"));
    assert_eq!(facts.description.as_deref(), Some("desc"));
    assert_eq!(runtime.update_generation(&id), Some(1));
}

#[test]
fn update_timeout_change_reschedules() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(ToastOptions::new().id(id.clone()), start);
    let ops = runtime.update_toast(
        &id,
        ToastOptions::new().timeout(Duration::from_millis(1000)),
        start,
    );
    assert!(!ops.is_empty());
    assert_eq!(
        runtime.remaining_timeout(&id),
        Some(Duration::from_millis(1000))
    );
}

#[test]
fn update_to_loading_clears_timer_and_from_loading_schedules() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(ToastOptions::new().id(id.clone()), start);

    runtime.update_toast(
        &id,
        ToastOptions::new().toast_type(ToastType::Loading),
        start,
    );
    assert_eq!(runtime.remaining_timeout(&id), None);

    let ops = runtime.update_toast(
        &id,
        ToastOptions::new().toast_type(ToastType::Success),
        start,
    );
    assert!(!ops.is_empty());
    assert_eq!(
        runtime.remaining_timeout(&id),
        Some(Duration::from_millis(5000))
    );
}

#[test]
fn update_to_sticky_timeout_clears_timer() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let id = ToastId::new("a");
    runtime.add_toast(ToastOptions::new().id(id.clone()), start);
    runtime.update_toast(&id, ToastOptions::new().timeout(Duration::ZERO), start);
    assert_eq!(runtime.remaining_timeout(&id), None);
}
