use crate::meter::MeterRuntime;

#[test]
fn value_below_min_clamps_to_min() {
    let state = MeterRuntime::new(-20.0, 0.0, 100.0, None).state();
    assert_eq!(state.value, -20.0);
    assert_eq!(state.clamped_value, 0.0);
    assert_eq!(state.percentage, 0.0);
}

#[test]
fn value_above_max_clamps_to_max() {
    let state = MeterRuntime::new(150.0, 0.0, 100.0, None).state();
    assert_eq!(state.value, 150.0);
    assert_eq!(state.clamped_value, 100.0);
    assert_eq!(state.percentage, 100.0);
}
