use gpui::TestAppContext;

use super::support::{
    click_toggle, focus_toggle, open_toggle, read_observations, simulate_keys, ToggleTestConfig,
};

#[gpui::test]
fn disabled_click_does_not_toggle_or_call_change_handler(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn disabled_keyboard_activation_does_not_toggle(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_toggle(cx, window);
    simulate_keys(cx, window, "space");
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert!(observations.value_changes.is_empty());
}
