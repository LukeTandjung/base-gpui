use std::time::{Duration, Instant};

use crate::toast::{ToastId, ToastOptions, ToastRuntime, ToastTimerOp, ToastType};

#[test]
fn default_timeout_is_5000_ms() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(ToastOptions::new(), Instant::now());
    assert_eq!(
        outcome.timer_ops,
        vec![ToastTimerOp::Schedule {
            id: outcome.id.clone(),
            generation: 1,
            duration: Duration::from_millis(5000),
        }]
    );
}

#[test]
fn per_toast_timeout_overrides_provider_default() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(
        ToastOptions::new().timeout(Duration::from_millis(1000)),
        Instant::now(),
    );
    let ToastTimerOp::Schedule { duration, .. } = &outcome.timer_ops[0] else {
        panic!("expected schedule");
    };
    assert_eq!(*duration, Duration::from_millis(1000));
}

#[test]
fn zero_timeout_is_sticky_no_timer() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(ToastOptions::new().timeout(Duration::ZERO), Instant::now());
    assert!(outcome.timer_ops.is_empty());
    assert_eq!(runtime.remaining_timeout(&outcome.id), None);
}

#[test]
fn loading_toast_never_schedules_a_timer() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(
        ToastOptions::new().toast_type(ToastType::Loading),
        Instant::now(),
    );
    assert!(outcome.timer_ops.is_empty());
}

#[test]
fn timer_expiry_closes_the_toast_not_instant_removal() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(ToastOptions::new(), Instant::now());
    let close = runtime
        .timer_fired(&outcome.id, 1)
        .expect("live timer closes");
    assert_eq!(close.closed, vec![outcome.id.clone()]);
    // Still mounted (ending) until removal.
    assert_eq!(runtime.toasts().len(), 1);
    assert!(runtime.toasts()[0].ending);
}

#[test]
fn stale_timer_generation_is_a_no_op() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(
        ToastOptions::new().id(ToastId::new("stale")),
        Instant::now(),
    );
    // Upsert resets the timer; the old generation becomes stale.
    runtime.add_toast(
        ToastOptions::new().id(ToastId::new("stale")),
        Instant::now(),
    );
    assert!(runtime.timer_fired(&outcome.id, 1).is_none());
    assert!(!runtime.toasts()[0].ending);
}

#[test]
fn stale_generation_after_close_is_a_no_op() {
    let mut runtime = ToastRuntime::<()>::new();
    let outcome = runtime.add_toast(ToastOptions::new(), Instant::now());
    runtime.close_toast(Some(&outcome.id));
    assert!(runtime.timer_fired(&outcome.id, 1).is_none());
}
