use gpui::TestAppContext;

use super::support::{
    click_trigger, open_accordion, read_observations, update_config, AccordionTestConfig, FIRST,
    SECOND,
};

#[gpui::test]
fn uncontrolled_initial_state_defaults_to_no_open_items(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert!(observations.item_state_at(0).closed);
    assert!(observations.panel_state_at(0).is_some());
    assert!(
        observations
            .panel_state_at(0)
            .expect("panel state")
            .item
            .hidden
    );
}

#[gpui::test]
fn uncontrolled_default_value_opens_matching_item_panel(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            default_values: Vec::from([FIRST]),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert!(observations.item_state_at(0).open);
    assert!(
        observations
            .panel_state_at(0)
            .expect("panel state")
            .item
            .open
    );
}

#[gpui::test]
fn uncontrolled_state_survives_prop_rerenders_until_key_changes(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    click_trigger(cx, window, "first");
    update_config(cx, window, |config| {
        config.default_values = Vec::from([SECOND]);
        config.disabled_second = true;
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert!(observations.item_state_at(0).open);

    update_config(cx, window, |config| {
        config.root_id = "accordion-reset";
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([SECOND]));
    assert!(observations.item_state_at(1).open);
}
