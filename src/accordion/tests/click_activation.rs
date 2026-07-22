use gpui::TestAppContext;

use super::support::{
    click_trigger, open_accordion, read_observations, AccordionTestConfig, FIRST,
};

#[gpui::test]
fn click_opens_closed_item(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([FIRST]));
    assert!(observations.item_state_at(0).open);
    assert_eq!(observations.root_value_changes, vec![Vec::from([FIRST])]);
    assert_eq!(observations.item_open_changes, vec![(FIRST, true)]);
}

#[gpui::test]
fn click_closes_open_item(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            default_values: Vec::from([FIRST]),
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert!(observations.item_state_at(0).closed);
    assert_eq!(observations.root_value_changes, vec![Vec::<&str>::new()]);
    assert_eq!(observations.item_open_changes, vec![(FIRST, false)]);
}
