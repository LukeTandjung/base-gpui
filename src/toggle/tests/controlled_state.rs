use gpui::TestAppContext;

use super::support::{
    click_toggle, open_toggle, read_observations, update_config, ToggleTestConfig,
};

#[gpui::test]
fn controlled_pressed_state_reflects_external_value(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            controlled_pressed: Some(true),
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_state().pressed);

    update_config(cx, window, |config| {
        config.controlled_pressed = Some(false);
    });

    assert!(!read_observations(cx, window).last_state().pressed);
}

#[gpui::test]
fn controlled_activation_calls_change_without_mutating_internal_state(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            controlled_pressed: Some(false),
            ..Default::default()
        },
    );

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
}
