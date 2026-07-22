use gpui::TestAppContext;

use super::support::{click_trigger, open_collapsible, read_observations, CollapsibleTestConfig};

#[gpui::test]
fn click_toggles_closed_to_open(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn click_toggles_open_to_closed(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            default_open: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert_eq!(observations.value_changes, vec![false]);
}

#[gpui::test]
fn disabled_click_does_not_toggle_or_call_change_handler(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().open);
    assert!(observations.value_changes.is_empty());
}
