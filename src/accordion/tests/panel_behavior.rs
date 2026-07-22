use gpui::TestAppContext;

use super::support::{
    click_trigger, debug_bounds, open_accordion, read_observations, AccordionTestConfig, FIRST,
    SECOND,
};

#[gpui::test]
fn panel_renders_children_while_open(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            default_values: Vec::from([FIRST]),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .panel_state_at(0)
        .expect("panel state should be observed");
    assert!(state.item.open);
    assert!(state.present);
    assert!(debug_bounds(cx, window, "accordion-panel-first").is_some());
}

#[gpui::test]
fn panel_is_omitted_while_closed_by_default(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    let state = read_observations(cx, window)
        .panel_state_at(0)
        .expect("panel state should be observed");
    assert!(state.item.closed);
    assert!(!state.present);
    assert!(debug_bounds(cx, window, "accordion-panel-first").is_none());
}

#[gpui::test]
fn root_keep_mounted_keeps_closed_panels_rendered_and_hidden(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            keep_mounted_root: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .panel_state_at(0)
        .expect("panel state should be observed");
    assert!(state.item.hidden);
    assert!(state.present);
    assert!(debug_bounds(cx, window, "accordion-panel-first").is_some());
}

#[gpui::test]
fn opening_kept_mounted_panel_updates_panel_state(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            keep_mounted_root: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let state = read_observations(cx, window)
        .panel_state_at(0)
        .expect("panel state should be observed");
    assert!(state.item.open);
    assert!(!state.item.hidden);
    assert!(state.present);
    assert!(debug_bounds(cx, window, "accordion-panel-first").is_some());
}

#[gpui::test]
fn panel_keep_mounted_false_overrides_root_keep_mounted(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            keep_mounted_root: true,
            first_panel_keep_mounted: Some(false),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .panel_state_at(0)
        .expect("panel state should be observed");
    assert!(!state.present);
    assert!(debug_bounds(cx, window, "accordion-panel-first").is_none());
}

#[gpui::test]
fn switching_items_in_single_mode_hides_previous_panel(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            default_values: Vec::from([FIRST]),
            ..Default::default()
        },
    );

    click_trigger(cx, window, "second");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([SECOND]));
    assert!(
        observations
            .panel_state_at(0)
            .expect("first panel")
            .item
            .hidden
    );
    assert!(
        observations
            .panel_state_at(1)
            .expect("second panel")
            .item
            .open
    );
    assert!(debug_bounds(cx, window, "accordion-panel-first").is_none());
}

#[gpui::test]
fn panel_can_be_omitted_without_breaking_trigger_behavior(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            include_first_panel: false,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert!(observations.panel_state_at(0).is_none());
}
