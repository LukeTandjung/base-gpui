use gpui::TestAppContext;

use super::support::{
    click_switch, focus_switch, open_switch, read_observations, simulate_keys, SwitchTestConfig,
};

#[gpui::test]
fn canceled_uncontrolled_pointer_activation_does_not_mutate_checked_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_uncontrolled_keyboard_activation_does_not_mutate_checked_state(
    cx: &mut TestAppContext,
) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    focus_switch(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(observations.change_canceled, vec![true]);
}
