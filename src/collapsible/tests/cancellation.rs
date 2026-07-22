use gpui::TestAppContext;

use super::support::{
    click_trigger, focus_trigger, open_collapsible, read_observations, simulate_keys,
    CollapsibleTestConfig,
};

#[gpui::test]
fn canceled_uncontrolled_pointer_activation_does_not_mutate_open_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_uncontrolled_keyboard_activation_does_not_mutate_open_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    focus_trigger(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_controlled_activation_calls_handler_without_mutating_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            controlled_open: Some(false),
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}
