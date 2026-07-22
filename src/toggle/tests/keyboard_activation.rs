use gpui::TestAppContext;

use super::support::{
    focus_toggle, open_toggle, read_observations, simulate_keys, ToggleTestConfig,
};

#[gpui::test]
fn space_toggles_focused_toggle(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    focus_toggle(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn enter_toggles_focused_toggle(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    focus_toggle(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
}
