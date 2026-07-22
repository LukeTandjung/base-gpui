use gpui::TestAppContext;

use super::support::{
    click_trigger, open_accordion, read_observations, update_config, AccordionTestConfig, FIRST,
    SECOND,
};

#[gpui::test]
fn controlled_value_opens_matching_item_panel(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            controlled_values: Some(Vec::from([FIRST])),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert!(observations.item_state_at(0).open);
    assert!(observations.item_state_at(1).closed);
}

#[gpui::test]
fn external_controlled_value_changes_update_part_style_state(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            controlled_values: Some(Vec::new()),
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.controlled_values = Some(Vec::from([SECOND]));
    });

    let observations = read_observations(cx, window);
    assert!(observations.item_state_at(0).closed);
    assert!(observations.header_state_at(1).item.open);
    assert!(observations.trigger_state_at(1).item.open);
    assert!(
        observations
            .panel_state_at(1)
            .expect("panel state")
            .item
            .open
    );
}

#[gpui::test]
fn controlled_activation_requests_change_without_mutating_internal_state(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            controlled_values: Some(Vec::new()),
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert_eq!(observations.root_value_changes, vec![Vec::from([FIRST])]);
}

#[gpui::test]
fn missing_controlled_values_do_not_render_phantom_panels(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            controlled_values: Some(Vec::from(["missing"])),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().values,
        Vec::from(["missing"])
    );
    assert!(observations.item_state_at(0).closed);
    assert!(observations.item_state_at(1).closed);
}
