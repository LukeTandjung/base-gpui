use gpui::TestAppContext;

use super::support::{
    click_switch, open_switch, read_observations, update_config, SwitchTestConfig,
};

#[gpui::test]
fn controlled_checked_state_reflects_external_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            controlled_checked: Some(true),
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_root_state().checked);

    update_config(cx, window, |config| {
        config.controlled_checked = Some(false);
    });

    assert!(!read_observations(cx, window).last_root_state().checked);
}

#[gpui::test]
fn external_controlled_value_changes_update_root_and_thumb_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            controlled_checked: Some(false),
            include_thumb: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations
        .last_thumb_state()
        .is_some_and(|state| !state.root.checked));

    update_config(cx, window, |config| {
        config.controlled_checked = Some(true);
    });

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().checked);
    assert!(observations
        .last_thumb_state()
        .is_some_and(|state| state.root.checked));
}

#[gpui::test]
fn controlled_click_calls_change_without_mutating_source_of_truth(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            controlled_checked: Some(false),
            ..Default::default()
        },
    );

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn canceled_controlled_activation_still_calls_handler_without_internal_mutation(
    cx: &mut TestAppContext,
) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            controlled_checked: Some(false),
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
