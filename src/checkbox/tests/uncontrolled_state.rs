use gpui::TestAppContext;

use super::support::{open_checkbox, read_observations, CheckboxTestConfig};

#[gpui::test]
fn uncontrolled_initial_state_is_unchecked(cx: &mut TestAppContext) {
    let window = open_checkbox(cx, CheckboxTestConfig::default());

    let state = read_observations(cx, window).last_root_state();
    assert!(!state.checked);
    assert!(state.unchecked);
}

#[gpui::test]
fn uncontrolled_default_checked_initial_state_is_checked(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            default_checked: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.checked);
    assert!(!state.unchecked);
}
