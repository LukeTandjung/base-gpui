use gpui::TestAppContext;

use super::support::{
    click_trigger, open_accordion, read_observations, AccordionTestConfig, FIRST, SECOND,
};

#[gpui::test]
fn single_open_mode_opens_only_one_item_at_a_time(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    click_trigger(cx, window, "first");
    click_trigger(cx, window, "second");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([SECOND]));
    assert!(observations.item_state_at(0).closed);
    assert!(observations.item_state_at(1).open);
    assert_eq!(
        observations.root_value_changes,
        vec![Vec::from([FIRST]), Vec::from([SECOND])]
    );
}

#[gpui::test]
fn single_open_mode_can_close_current_item_to_empty_value(cx: &mut TestAppContext) {
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
}

#[gpui::test]
fn multiple_open_mode_allows_more_than_one_open_item(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            multiple: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");
    click_trigger(cx, window, "second");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().values,
        Vec::from([FIRST, SECOND])
    );
    assert!(observations.item_state_at(0).open);
    assert!(observations.item_state_at(1).open);
}

#[gpui::test]
fn multiple_open_mode_closes_one_item_without_closing_siblings(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            multiple: true,
            default_values: Vec::from([FIRST, SECOND]),
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().values, Vec::from([SECOND]));
    assert!(observations.item_state_at(0).closed);
    assert!(observations.item_state_at(1).open);
}

#[gpui::test]
fn duplicate_item_values_behave_as_one_open_group(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            second_value: Some(FIRST),
            default_values: Vec::from([FIRST]),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.item_state_at(0).open);
    assert!(observations.item_state_at(1).open);
}
