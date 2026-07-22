use gpui::TestAppContext;

use super::support::{
    click_trigger, open_accordion, read_observations, AccordionTestConfig, FIRST,
};

#[gpui::test]
fn item_open_change_cancellation_prevents_root_callback_and_uncontrolled_mutation(
    cx: &mut TestAppContext,
) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            cancel_item_changes: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert_eq!(observations.item_open_changes, vec![(FIRST, true)]);
    assert!(observations.root_value_changes.is_empty());
}

#[gpui::test]
fn root_value_change_cancellation_prevents_uncontrolled_mutation(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            cancel_root_changes: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert_eq!(observations.item_open_changes, vec![(FIRST, true)]);
    assert_eq!(observations.root_value_changes, vec![Vec::from([FIRST])]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn controlled_cancellation_calls_handlers_without_mutating_internal_state(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            controlled_values: Some(Vec::new()),
            cancel_root_changes: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert_eq!(observations.item_open_changes, vec![(FIRST, true)]);
    assert_eq!(observations.root_value_changes, vec![Vec::from([FIRST])]);
}
