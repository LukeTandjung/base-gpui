use gpui::TestAppContext;

use super::support::{
    focus_switch, open_switch, read_observations, simulate_keys, SwitchTestConfig,
};

#[gpui::test]
fn disabled_space_does_not_toggle_or_call_change_handler(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_switch(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn disabled_enter_does_not_toggle_or_call_change_handler(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_switch(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations.value_changes.is_empty());
}
