use gpui::TestAppContext;

use super::support::{
    focus_checkbox, open_checkbox, read_observations, simulate_keys, CheckboxTestConfig,
};

#[gpui::test]
fn space_toggles_when_focused(cx: &mut TestAppContext) {
    let window = open_checkbox(cx, CheckboxTestConfig::default());

    focus_checkbox(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
}
