use crate::progress::{ProgressRuntime, ProgressStatus};

#[test]
fn value_at_min_is_zero_percent_progressing() {
    let state = ProgressRuntime::new(Some(20.0), 20.0, 40.0, None).state();
    assert_eq!(state.percentage, Some(0.0));
    assert_eq!(state.status, ProgressStatus::Progressing);
}

#[test]
fn value_at_max_is_hundred_percent_complete() {
    let state = ProgressRuntime::new(Some(40.0), 20.0, 40.0, None).state();
    assert_eq!(state.percentage, Some(100.0));
    assert_eq!(state.status, ProgressStatus::Complete);
}

/// Degenerate range: the value clamps to `max`, so status is `Complete`,
/// while the NaN percentage falls back to `0` — no panic, no NaN leak.
#[test]
fn degenerate_min_equals_max_is_complete_with_zero_percentage() {
    let state = ProgressRuntime::new(Some(50.0), 30.0, 30.0, None).state();
    assert_eq!(state.clamped_value, Some(30.0));
    assert_eq!(state.percentage, Some(0.0));
    assert_eq!(state.status, ProgressStatus::Complete);
    assert_eq!(state.formatted.as_deref(), Some("0%"));
}
