use crate::meter::MeterRuntime;

#[test]
fn mid_range_percentage_with_default_range() {
    let state = MeterRuntime::new(30.0, 0.0, 100.0, None).state();
    assert_eq!(state.percentage, 30.0);
}

#[test]
fn mid_range_percentage_with_custom_range() {
    let state = MeterRuntime::new(30.0, 20.0, 40.0, None).state();
    assert_eq!(state.percentage, 50.0);
}
