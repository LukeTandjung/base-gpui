use gpui::TestAppContext;

use super::support::{
    open_radio_group, read_observations, RadioGroupTestConfig, EXPRESS, STANDARD,
};

#[gpui::test]
fn radio_can_be_used_without_indicator_child(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            include_indicators: false,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(EXPRESS));
    assert!(observations.indicator_state(EXPRESS).is_none());
}

#[gpui::test]
fn indicator_is_absent_by_default_when_unchecked(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    let observations = read_observations(cx, window);
    assert!(observations.indicator_state(STANDARD).is_none());
}

#[gpui::test]
fn indicator_renders_when_checked(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    let indicator = observations
        .indicator_state(EXPRESS)
        .expect("checked indicator should render");
    assert!(indicator.present);
    assert!(indicator.radio.checked);
}

#[gpui::test]
fn indicator_remains_rendered_with_keep_mounted(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            keep_mounted_indicators: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    let indicator = observations
        .indicator_state(STANDARD)
        .expect("keep mounted indicator should render");
    assert!(indicator.present);
    assert!(indicator.radio.unchecked);
}
