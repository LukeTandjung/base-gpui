use gpui::TestAppContext;

use crate::accordion::AccordionOrientation;

use super::support::{
    click_trigger, open_accordion, read_observations, update_config, AccordionTestConfig, FIRST,
};

#[gpui::test]
fn style_with_state_receives_correct_root_state(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            multiple: true,
            default_values: Vec::from([FIRST]),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert_eq!(state.values, Vec::from([FIRST]));
    assert!(state.multiple);
    assert!(!state.disabled);
    assert_eq!(state.orientation, AccordionOrientation::Vertical);
}

#[gpui::test]
fn style_with_state_receives_correct_item_header_trigger_and_panel_state(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.item_state_at(0).open);
    assert!(observations.header_state_at(0).item.open);
    assert!(observations.trigger_state_at(0).panel_open);
    assert!(
        observations
            .panel_state_at(0)
            .expect("panel state")
            .item
            .open
    );
    assert_eq!(observations.item_state_at(0).value, FIRST);
    assert_eq!(observations.item_state_at(0).index, 0);
}

#[gpui::test]
fn kept_mounted_closed_panel_receives_hidden_style_state(cx: &mut TestAppContext) {
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
    assert!(state.item.closed);
    assert!(state.present);
}

#[gpui::test]
fn item_indices_are_deterministic_and_update_when_items_change(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    let observations = read_observations(cx, window);
    assert_eq!(observations.item_state_at(0).index, 0);
    assert_eq!(observations.item_state_at(1).index, 1);

    update_config(cx, window, |config| {
        config.include_second_item = false;
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.item_states.len(), 1);
    assert_eq!(observations.item_state_at(0).index, 0);
}
