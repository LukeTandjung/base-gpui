use gpui::TestAppContext;

use super::support::{
    click_radio, open_radio_group, read_observations, simulate_keys, RadioGroupTestConfig, EXPRESS,
    OVERNIGHT,
};

#[gpui::test]
fn disabled_group_click_does_not_select_or_call_handler(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn disabled_radio_click_does_not_select_or_call_handler(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            express_disabled: true,
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn disabled_radios_are_skipped_by_roving_focus(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            express_disabled: true,
            ..Default::default()
        },
    );

    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(OVERNIGHT));
    assert_eq!(observations.selected_value(), Some(OVERNIGHT));
}

#[gpui::test]
fn disabled_selected_radio_remains_checked_but_not_tab_stop(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            express_disabled: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(EXPRESS));
    assert_eq!(
        observations.tab_stop_value(),
        Some(super::support::STANDARD)
    );
}
