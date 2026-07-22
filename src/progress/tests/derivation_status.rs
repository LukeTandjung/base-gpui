use crate::progress::{ProgressRuntime, ProgressStatus};

#[test]
fn none_value_is_indeterminate() {
    let state = ProgressRuntime::new(None, 0.0, 100.0, None).state();
    assert_eq!(state.status, ProgressStatus::Indeterminate);
    assert_eq!(state.clamped_value, None);
    assert_eq!(state.percentage, None);
    assert_eq!(state.formatted, None);
}

#[test]
fn non_finite_value_is_indeterminate() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let state = ProgressRuntime::new(Some(value), 0.0, 100.0, None).state();
        assert_eq!(state.status, ProgressStatus::Indeterminate);
        assert_eq!(state.value, None);
        assert_eq!(state.clamped_value, None);
        assert_eq!(state.percentage, None);
        assert_eq!(state.formatted, None);
    }
}

#[test]
fn below_max_is_progressing() {
    let state = ProgressRuntime::new(Some(40.0), 0.0, 100.0, None).state();
    assert_eq!(state.status, ProgressStatus::Progressing);
}

#[test]
fn at_max_is_complete() {
    let state = ProgressRuntime::new(Some(100.0), 0.0, 100.0, None).state();
    assert_eq!(state.status, ProgressStatus::Complete);
}

#[test]
fn raw_value_above_max_reads_complete() {
    let state = ProgressRuntime::new(Some(120.0), 0.0, 100.0, None).state();
    assert_eq!(state.status, ProgressStatus::Complete);
}
