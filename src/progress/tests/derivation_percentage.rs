use crate::progress::ProgressRuntime;

#[test]
fn mid_range_percentage_with_default_range() {
    let state = ProgressRuntime::new(Some(30.0), 0.0, 100.0, None).state();
    assert_eq!(state.percentage, Some(30.0));
}

#[test]
fn mid_range_percentage_with_custom_range() {
    let state = ProgressRuntime::new(Some(30.0), 20.0, 40.0, None).state();
    assert_eq!(state.percentage, Some(50.0));
}
