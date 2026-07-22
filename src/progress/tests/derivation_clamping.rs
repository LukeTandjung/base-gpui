use crate::progress::{ProgressRuntime, ProgressStatus};

#[test]
fn value_below_min_clamps_to_min() {
    let state = ProgressRuntime::new(Some(-20.0), 0.0, 100.0, None).state();
    assert_eq!(state.value, Some(-20.0));
    assert_eq!(state.clamped_value, Some(0.0));
    assert_eq!(state.percentage, Some(0.0));
    assert_eq!(state.status, ProgressStatus::Progressing);
}

#[test]
fn value_above_max_clamps_to_max() {
    let state = ProgressRuntime::new(Some(150.0), 0.0, 100.0, None).state();
    assert_eq!(state.value, Some(150.0));
    assert_eq!(state.clamped_value, Some(100.0));
    assert_eq!(state.percentage, Some(100.0));
    assert_eq!(state.status, ProgressStatus::Complete);
}
