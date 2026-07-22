use gpui::TestAppContext;

use super::support::{
    click_toggle, focus_toggle, open_toggle, read_observations, simulate_keys, ToggleTestConfig,
};

#[gpui::test]
fn canceled_uncontrolled_pointer_activation_does_not_mutate_pressed_state(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_uncontrolled_keyboard_activation_does_not_mutate_pressed_state(
    cx: &mut TestAppContext,
) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    focus_toggle(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_controlled_activation_still_calls_handler_without_internal_mutation(
    cx: &mut TestAppContext,
) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            controlled_pressed: Some(false),
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}
