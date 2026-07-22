use gpui::{px, TestAppContext};

use super::support::{debug_bounds, open_meter, read_observations, MeterTestConfig};

#[gpui::test]
fn indicator_fills_relative_width(cx: &mut TestAppContext) {
    let window = open_meter(
        cx,
        MeterTestConfig {
            value: 50.0,
            ..Default::default()
        },
    );
    read_observations(cx, window);

    let indicator = debug_bounds(cx, window, "meter-indicator").expect("indicator bounds observed");
    // Track is 200px wide; 50% fill is 100px.
    assert_eq!(indicator.size.width, px(100.0));
}

#[gpui::test]
fn indicator_has_zero_width_at_min(cx: &mut TestAppContext) {
    let window = open_meter(cx, MeterTestConfig::default());
    read_observations(cx, window);

    let indicator = debug_bounds(cx, window, "meter-indicator").expect("indicator bounds observed");
    assert_eq!(indicator.size.width, px(0.0));
}
