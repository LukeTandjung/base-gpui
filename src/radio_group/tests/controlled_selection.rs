use gpui::TestAppContext;

use super::support::{
    click_radio, open_radio_group, read_observations, update_config, RadioGroupTestConfig, EXPRESS,
    OVERNIGHT, STANDARD,
};

#[gpui::test]
fn controlled_selected_state_reflects_external_value(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            controlled_value: Some(Some(EXPRESS)),
            ..Default::default()
        },
    );

    assert_eq!(
        read_observations(cx, window).selected_value(),
        Some(EXPRESS)
    );
}

#[gpui::test]
fn controlled_none_value_clears_selection(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            controlled_value: Some(Some(EXPRESS)),
            ..Default::default()
        },
    );
    assert_eq!(
        read_observations(cx, window).selected_value(),
        Some(EXPRESS)
    );

    update_config(cx, window, |config| {
        config.controlled_value = Some(None);
    });

    assert_eq!(read_observations(cx, window).selected_value(), None);
}

#[gpui::test]
fn controlled_click_calls_handler_without_mutating_internal_source_of_truth(
    cx: &mut TestAppContext,
) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            controlled_value: Some(Some(STANDARD)),
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(STANDARD));
    assert_eq!(observations.value_changes, vec![Some(EXPRESS)]);
}

#[gpui::test]
fn canceled_controlled_activation_calls_handler_without_mutating_selected_value(
    cx: &mut TestAppContext,
) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            controlled_value: Some(Some(STANDARD)),
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(STANDARD));
    assert_eq!(observations.value_changes, vec![Some(EXPRESS)]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn external_controlled_value_changes_update_radio_and_indicator_state(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            controlled_value: Some(Some(EXPRESS)),
            ..Default::default()
        },
    );
    assert_eq!(
        read_observations(cx, window).selected_value(),
        Some(EXPRESS)
    );

    update_config(cx, window, |config| {
        config.controlled_value = Some(Some(OVERNIGHT));
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(OVERNIGHT));
    assert!(
        observations
            .indicator_state(OVERNIGHT)
            .expect("overnight indicator should render")
            .present
    );
}
