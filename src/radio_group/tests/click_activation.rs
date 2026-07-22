use gpui::TestAppContext;

use super::support::{
    click_radio, open_radio_group, read_observations, RadioGroupTestConfig, EXPRESS,
};

#[gpui::test]
fn clicking_enabled_unchecked_radio_selects_it(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(EXPRESS));
    assert_eq!(observations.highlighted_value(), Some(EXPRESS));
    assert_eq!(observations.value_changes, vec![Some(EXPRESS)]);
}

#[gpui::test]
fn clicking_already_selected_radio_is_noop(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(EXPRESS));
    assert!(observations.value_changes.is_empty());
}
