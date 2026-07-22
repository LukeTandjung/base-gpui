use gpui::{px, TestAppContext};

use super::support::{debug_bounds, open_progress, read_observations, ProgressTestConfig};

#[gpui::test]
fn determinate_indicator_fills_relative_width(cx: &mut TestAppContext) {
    let window = open_progress(
        cx,
        ProgressTestConfig {
            value: Some(50.0),
            ..Default::default()
        },
    );
    read_observations(cx, window);

    let indicator =
        debug_bounds(cx, window, "progress-indicator").expect("indicator bounds observed");
    // Track is 200px wide; 50% fill is 100px.
    assert_eq!(indicator.size.width, px(100.0));
}

#[gpui::test]
fn indicator_has_zero_width_at_min(cx: &mut TestAppContext) {
    let window = open_progress(
        cx,
        ProgressTestConfig {
            value: Some(0.0),
            ..Default::default()
        },
    );
    read_observations(cx, window);

    let indicator =
        debug_bounds(cx, window, "progress-indicator").expect("indicator bounds observed");
    assert_eq!(indicator.size.width, px(0.0));
}

/// Empty-style parity with Base UI: when indeterminate, the indicator gets
/// no default fill width, so it lays out with auto width (stretching to the
/// track) exactly as a DOM div with an empty style object would.
#[gpui::test]
fn indeterminate_indicator_has_no_default_fill(cx: &mut TestAppContext) {
    let window = open_progress(cx, ProgressTestConfig::default());
    read_observations(cx, window);

    let indicator =
        debug_bounds(cx, window, "progress-indicator").expect("indicator bounds observed");
    let track = debug_bounds(cx, window, "progress-track").expect("track bounds observed");
    assert_eq!(indicator.size.width, track.size.width);
}
