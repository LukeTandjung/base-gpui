use gpui::TestAppContext;

use super::support::{
    click_trigger, focus_trigger, open_accordion, read_observations, simulate_keys,
    AccordionTestConfig,
};

#[gpui::test]
fn disabled_root_disables_all_items_and_prevents_pointer_changes(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            disabled_root: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().disabled);
    assert!(observations.item_state_at(0).disabled);
    assert!(observations.trigger_state_at(1).item.disabled);
    assert!(observations.root_value_changes.is_empty());
    assert!(observations.item_open_changes.is_empty());
}

#[gpui::test]
fn disabled_item_prevents_its_changes_without_disabling_siblings(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            disabled_first: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window, "first");
    click_trigger(cx, window, "second");

    let observations = read_observations(cx, window);
    assert!(observations.item_state_at(0).disabled);
    assert!(!observations.item_state_at(1).disabled);
    assert!(observations.item_state_at(0).closed);
    assert!(observations.item_state_at(1).open);
    assert_eq!(observations.root_value_changes.len(), 1);
}

#[gpui::test]
fn disabled_root_keyboard_activation_does_not_toggle(cx: &mut TestAppContext) {
    let window = open_accordion(
        cx,
        AccordionTestConfig {
            disabled_root: true,
            ..Default::default()
        },
    );

    focus_trigger(cx, window, 0);
    simulate_keys(cx, window, "space enter");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().values.is_empty());
    assert!(observations.root_value_changes.is_empty());
}
