use crate::meter::MeterRuntime;

#[test]
fn value_at_min_is_zero_percent() {
    let state = MeterRuntime::new(20.0, 20.0, 40.0, None).state();
    assert_eq!(state.percentage, 0.0);
    assert_eq!(state.formatted, "0%");
}

#[test]
fn value_at_max_is_hundred_percent() {
    let state = MeterRuntime::new(40.0, 20.0, 40.0, None).state();
    assert_eq!(state.percentage, 100.0);
    assert_eq!(state.formatted, "100%");
}

/// Degenerate range: the NaN percentage falls back to `0` — no panic, no
/// NaN leak.
#[test]
fn degenerate_min_equals_max_has_zero_percentage() {
    let state = MeterRuntime::new(50.0, 30.0, 30.0, None).state();
    assert_eq!(state.clamped_value, 30.0);
    assert_eq!(state.percentage, 0.0);
    assert_eq!(state.formatted, "0%");
}

/// A `NaN` value falls back to `min` before clamping (matches Base UI).
#[test]
fn nan_value_falls_back_to_min() {
    let state = MeterRuntime::new(f64::NAN, 20.0, 40.0, None).state();
    assert_eq!(state.clamped_value, 20.0);
    assert_eq!(state.percentage, 0.0);
    assert_eq!(state.formatted, "0%");
}
