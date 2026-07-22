use gpui::TestAppContext;

use super::support::{
    focus_checkbox, open_checkbox, read_observations, simulate_keys, CheckboxTestConfig,
};

#[gpui::test]
fn enter_does_not_toggle_when_focused(cx: &mut TestAppContext) {
    let window = open_checkbox(cx, CheckboxTestConfig::default());

    focus_checkbox(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations.value_changes.is_empty());
}
