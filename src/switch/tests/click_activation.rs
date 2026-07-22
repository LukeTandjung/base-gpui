use gpui::TestAppContext;

use super::support::{click_switch, open_switch, read_observations, SwitchTestConfig};

#[gpui::test]
fn click_toggles_unchecked_to_checked(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn click_toggles_checked_to_unchecked(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            default_checked: true,
            ..Default::default()
        },
    );

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![false]);
}

#[gpui::test]
fn disabled_click_does_not_toggle_or_call_change_handler(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn read_only_click_does_not_toggle_or_call_change_handler(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            read_only: true,
            ..Default::default()
        },
    );

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations.value_changes.is_empty());
}
