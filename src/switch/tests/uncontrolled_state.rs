use gpui::TestAppContext;

use super::support::{open_switch, read_observations, SwitchTestConfig};

#[gpui::test]
fn uncontrolled_initial_state_defaults_to_unchecked(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    let state = read_observations(cx, window).last_root_state();
    assert!(!state.checked);
    assert!(state.unchecked);
}

#[gpui::test]
fn uncontrolled_default_checked_initializes_to_checked(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            default_checked: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.checked);
    assert!(!state.unchecked);
}
