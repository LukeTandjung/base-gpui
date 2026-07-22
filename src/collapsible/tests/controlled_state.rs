use gpui::TestAppContext;

use super::support::{
    click_trigger, open_collapsible, read_observations, update_config, CollapsibleTestConfig,
};

#[gpui::test]
fn controlled_open_state_reflects_external_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            controlled_open: Some(true),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.open);
    assert!(!state.closed);
}

#[gpui::test]
fn external_controlled_value_changes_update_style_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            controlled_open: Some(false),
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.controlled_open = Some(true);
    });

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().open);
    assert!(observations.last_trigger_state().open);
    assert!(observations.last_panel_state().expect("panel state").open);
}

#[gpui::test]
fn controlled_activation_requests_change_without_mutating_internal_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            controlled_open: Some(false),
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
}
