use gpui::TestAppContext;

use super::support::{
    click_trigger, debug_bounds, open_collapsible, read_observations, CollapsibleTestConfig,
};

#[gpui::test]
fn panel_renders_children_while_open(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            default_open: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .last_panel_state()
        .expect("panel state should be observed");
    assert!(state.open);
    assert!(state.present);
    assert!(debug_bounds(cx, window, "collapsible-panel").is_some());
}

#[gpui::test]
fn panel_is_omitted_while_closed_by_default(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    let state = read_observations(cx, window)
        .last_panel_state()
        .expect("panel state should be observed");
    assert!(state.closed);
    assert!(!state.present);
    assert!(debug_bounds(cx, window, "collapsible-panel").is_none());
}

#[gpui::test]
fn panel_remains_rendered_while_closed_when_keep_mounted(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            keep_mounted_panel: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .last_panel_state()
        .expect("panel state should be observed");
    assert!(state.closed);
    assert!(state.present);
    assert!(state.mounted);
    assert!(debug_bounds(cx, window, "collapsible-panel").is_some());
}

#[gpui::test]
fn opening_kept_mounted_panel_updates_panel_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            keep_mounted_panel: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let state = read_observations(cx, window)
        .last_panel_state()
        .expect("panel state should be observed");
    assert!(state.open);
    assert!(!state.closed);
    assert!(state.present);
    assert!(debug_bounds(cx, window, "collapsible-panel").is_some());
}

#[gpui::test]
fn panel_can_be_omitted_without_breaking_trigger_behavior(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            include_panel: false,
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().open);
    assert!(observations.panel_states.is_empty());
}
