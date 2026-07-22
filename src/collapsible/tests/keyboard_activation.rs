use gpui::TestAppContext;

use super::support::{
    focus_trigger, open_collapsible, read_observations, simulate_keys, CollapsibleTestConfig,
};

#[gpui::test]
fn space_toggles_when_trigger_is_focused(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    focus_trigger(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn enter_toggles_when_trigger_is_focused(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    focus_trigger(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn disabled_keyboard_activation_does_not_toggle(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_trigger(cx, window);
    simulate_keys(cx, window, "space enter");

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert!(observations.value_changes.is_empty());
}
