use gpui::TestAppContext;

use super::support::{
    open_radio_group, read_observations, RadioGroupTestConfig, EXPRESS, STANDARD,
};

#[gpui::test]
fn radio_state_exposes_checked_and_unchecked(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(
        observations
            .radio_state(EXPRESS)
            .expect("express state")
            .checked
    );
    assert!(
        observations
            .radio_state(STANDARD)
            .expect("standard state")
            .unchecked
    );
}

#[gpui::test]
fn radio_state_exposes_disabled_read_only_and_required(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            express_disabled: true,
            express_read_only: true,
            express_required: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .radio_state(EXPRESS)
        .expect("express state");
    assert!(state.disabled);
    assert!(state.read_only);
    assert!(state.required);
}

#[gpui::test]
fn root_style_with_state_receives_group_state(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            disabled: true,
            read_only: true,
            required: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.disabled);
    assert!(state.read_only);
    assert!(state.required);
}
