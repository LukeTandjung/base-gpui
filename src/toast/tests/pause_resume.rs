use std::time::{Duration, Instant};

use crate::toast::{ToastOptions, ToastRuntime, ToastTimerOp, ToastType};

#[test]
fn pause_records_remaining_and_resume_restarts_with_remaining() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let outcome = runtime.add_toast(ToastOptions::new(), start);

    runtime.set_hovering(true, start + Duration::from_millis(2000));
    assert_eq!(
        runtime.remaining_timeout(&outcome.id),
        Some(Duration::from_millis(3000))
    );

    let ops = runtime.set_hovering(false, start + Duration::from_millis(4000));
    assert_eq!(
        ops,
        vec![ToastTimerOp::Schedule {
            id: outcome.id.clone(),
            generation: 2,
            duration: Duration::from_millis(3000),
        }]
    );
}

#[test]
fn pause_and_resume_are_idempotent() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let outcome = runtime.add_toast(ToastOptions::new(), start);

    runtime.pause_timers(start + Duration::from_millis(1000));
    runtime.pause_timers(start + Duration::from_millis(2500));
    assert_eq!(
        runtime.remaining_timeout(&outcome.id),
        Some(Duration::from_millis(4000))
    );

    let first_resume = runtime.resume_timers(start + Duration::from_millis(3000));
    assert_eq!(first_resume.len(), 1);
    let second_resume = runtime.resume_timers(start + Duration::from_millis(3000));
    assert!(second_resume.is_empty());
}

#[test]
fn remaining_clamps_at_zero_when_paused_after_expiry_time() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let outcome = runtime.add_toast(ToastOptions::new(), start);
    runtime.pause_timers(start + Duration::from_millis(9000));
    assert_eq!(runtime.remaining_timeout(&outcome.id), Some(Duration::ZERO));
}

#[test]
fn toast_added_while_hovered_starts_paused_with_full_duration() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    runtime.set_hovering(true, start);
    let outcome = runtime.add_toast(ToastOptions::new(), start);
    assert!(outcome.timer_ops.is_empty());
    assert_eq!(
        runtime.remaining_timeout(&outcome.id),
        Some(Duration::from_millis(5000))
    );
    // Unhover starts the timer with the full duration.
    let ops = runtime.set_hovering(false, start + Duration::from_millis(500));
    assert_eq!(ops.len(), 1);
}

#[test]
fn toast_added_while_window_unfocused_starts_paused() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    runtime.set_window_focused(false, start);
    let outcome = runtime.add_toast(ToastOptions::new(), start);
    assert!(outcome.timer_ops.is_empty());
    let ops = runtime.set_window_focused(true, start + Duration::from_millis(100));
    assert_eq!(ops.len(), 1);
}

#[test]
fn repause_works_after_last_toast_closed() {
    // store.test.ts re-pause regression: after the last toast closes, hover
    // facts reset so a fresh toast's running timer pauses again on hover.
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let first = runtime.add_toast(ToastOptions::new(), start);
    runtime.set_hovering(true, start);
    runtime.close_toast(Some(&first.id));
    runtime.remove_toast(&first.id);

    // Hover fact was reset; a new toast starts running.
    let second = runtime.add_toast(ToastOptions::new(), start + Duration::from_millis(100));
    assert_eq!(second.timer_ops.len(), 1);
    // And can be paused again by a fresh hover.
    runtime.set_hovering(true, start + Duration::from_millis(1100));
    assert_eq!(
        runtime.remaining_timeout(&second.id),
        Some(Duration::from_millis(4000))
    );
}

#[test]
fn repause_works_after_all_toasts_closed() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    runtime.add_toast(ToastOptions::new(), start);
    runtime.add_toast(ToastOptions::new(), start);
    runtime.set_hovering(true, start);
    runtime.close_toast(None);

    let fresh = runtime.add_toast(ToastOptions::new(), start + Duration::from_millis(100));
    assert_eq!(fresh.timer_ops.len(), 1);
    runtime.set_hovering(true, start + Duration::from_millis(600));
    assert_eq!(
        runtime.remaining_timeout(&fresh.id),
        Some(Duration::from_millis(4500))
    );
}

#[test]
fn repause_works_when_last_active_closes_while_ending_toasts_remain() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let lingering = runtime.add_toast(ToastOptions::new(), start);
    runtime.close_toast(Some(&lingering.id));
    // The ending toast stays mounted (exit transition pending).
    let active = runtime.add_toast(ToastOptions::new(), start);
    runtime.set_hovering(true, start);
    runtime.close_toast(Some(&active.id));

    // No active toasts remain: hover fact resets even with ending toasts.
    let fresh = runtime.add_toast(ToastOptions::new(), start + Duration::from_millis(100));
    assert_eq!(fresh.timer_ops.len(), 1);
}

#[test]
fn repause_works_when_last_timed_closes_while_untimed_toasts_remain() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    runtime.add_toast(ToastOptions::new().timeout(Duration::ZERO), start);
    let timed = runtime.add_toast(ToastOptions::new(), start);
    runtime.set_hovering(true, start);
    runtime.close_toast(Some(&timed.id));
    runtime.remove_toast(&timed.id);

    // The sticky toast keeps the stack alive; hovering stays true, so a new
    // toast starts paused and resumes on unhover.
    let fresh = runtime.add_toast(ToastOptions::new(), start + Duration::from_millis(100));
    assert!(fresh.timer_ops.is_empty());
    let ops = runtime.set_hovering(false, start + Duration::from_millis(200));
    assert_eq!(ops.len(), 1);
}

#[test]
fn repause_works_when_last_timed_toast_becomes_untimed() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let outcome = runtime.add_toast(ToastOptions::new(), start);
    // Becomes loading: timer cleared.
    let ops = runtime.update_toast(
        &outcome.id,
        ToastOptions::new().toast_type(ToastType::Loading),
        start,
    );
    assert_eq!(
        ops,
        vec![ToastTimerOp::Cancel {
            id: outcome.id.clone()
        }]
    );
    // A later hover/unhover cycle still behaves for a new timed toast.
    runtime.set_hovering(true, start);
    let second = runtime.add_toast(ToastOptions::new(), start);
    assert!(second.timer_ops.is_empty());
    let ops = runtime.set_hovering(false, start);
    assert_eq!(ops.len(), 1);
}

#[test]
fn focus_pauses_and_blur_resumes() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    let outcome = runtime.add_toast(ToastOptions::new(), start);
    runtime.set_focused(true, start + Duration::from_millis(1000));
    assert_eq!(
        runtime.remaining_timeout(&outcome.id),
        Some(Duration::from_millis(4000))
    );
    let ops = runtime.set_focused(false, start + Duration::from_millis(2000));
    assert_eq!(ops.len(), 1);
}

#[test]
fn resume_while_window_unfocused_does_not_restart_timers() {
    let mut runtime = ToastRuntime::<()>::new();
    let start = Instant::now();
    runtime.add_toast(ToastOptions::new(), start);
    runtime.set_window_focused(false, start);
    // Hover leave while window unfocused stays paused.
    runtime.set_hovering(true, start);
    let ops = runtime.set_hovering(false, start);
    assert!(ops.is_empty());
    assert!(runtime.paused());
}
