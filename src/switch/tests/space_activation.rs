use gpui::TestAppContext;

use super::support::{
    focus_switch, open_switch, read_observations, simulate_keys, SwitchTestConfig,
};

#[gpui::test]
fn space_toggles_when_focused(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    focus_switch(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
}
