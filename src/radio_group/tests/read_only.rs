use gpui::TestAppContext;

use super::support::{
    click_radio, open_radio_group, read_observations, RadioGroupTestConfig, EXPRESS,
};

#[gpui::test]
fn read_only_group_click_does_not_select_or_call_handler(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            read_only: true,
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn read_only_radio_click_does_not_select_or_call_handler(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            express_read_only: true,
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert!(observations.value_changes.is_empty());
}
