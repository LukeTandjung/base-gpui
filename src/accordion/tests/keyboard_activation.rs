use gpui::TestAppContext;

use super::support::{
    focus_trigger, open_accordion, read_observations, simulate_keys, AccordionTestConfig, FIRST,
};

#[gpui::test]
fn space_toggles_when_trigger_is_focused(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    focus_trigger(cx, window, 0);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert_eq!(observations.root_value_changes, vec![Vec::from([FIRST])]);
}

#[gpui::test]
fn enter_toggles_when_trigger_is_focused(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    focus_trigger(cx, window, 0);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert_eq!(observations.root_value_changes, vec![Vec::from([FIRST])]);
}
