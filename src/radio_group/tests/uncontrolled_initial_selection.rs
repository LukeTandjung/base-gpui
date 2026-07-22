use gpui::TestAppContext;

use super::support::{
    click_radio, open_radio_group, read_observations, update_config, RadioGroupTestConfig, EXPRESS,
    STANDARD,
};

#[gpui::test]
fn uncontrolled_initial_no_selection_state(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert_eq!(observations.highlighted_value(), Some(STANDARD));
    assert_eq!(observations.tab_stop_value(), Some(STANDARD));
}

#[gpui::test]
fn uncontrolled_selection_survives_unrelated_prop_rerender(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    click_radio(cx, window, EXPRESS);
    assert_eq!(
        read_observations(cx, window).selected_value(),
        Some(EXPRESS)
    );

    update_config(cx, window, |config| {
        config.required = true;
    });

    assert_eq!(
        read_observations(cx, window).selected_value(),
        Some(EXPRESS)
    );
}

#[gpui::test]
fn uncontrolled_default_value_initial_selected_state(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(EXPRESS));
    assert_eq!(observations.highlighted_value(), Some(EXPRESS));
    assert_eq!(observations.tab_stop_value(), Some(EXPRESS));
}
