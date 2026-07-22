use gpui::TestAppContext;

use super::support::{debug_bounds, open_checkbox, read_observations, CheckboxTestConfig};

#[gpui::test]
fn indicator_is_absent_by_default_when_unchecked(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            include_indicator: true,
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "checkbox-indicator").is_none());
    assert!(read_observations(cx, window)
        .last_indicator_state()
        .is_none());
}

#[gpui::test]
fn indicator_renders_when_checked(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            default_checked: true,
            include_indicator: true,
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "checkbox-indicator").is_some());
    let state = read_observations(cx, window)
        .last_indicator_state()
        .expect("indicator state should be observed");
    assert!(state.present);
    assert!(state.root.checked);
}

#[gpui::test]
fn indicator_renders_when_indeterminate(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            indeterminate: true,
            include_indicator: true,
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "checkbox-indicator").is_some());
    let state = read_observations(cx, window)
        .last_indicator_state()
        .expect("indicator state should be observed");
    assert!(state.present);
    assert!(state.root.indeterminate);
}

#[gpui::test]
fn indicator_remains_rendered_with_keep_mounted(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            include_indicator: true,
            keep_mounted_indicator: true,
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "checkbox-indicator").is_some());
    let state = read_observations(cx, window)
        .last_indicator_state()
        .expect("indicator state should be observed");
    assert!(state.present);
    assert!(!state.root.checked);
    assert!(!state.root.indeterminate);
}
