use gpui::TestAppContext;

use super::support::{click_toggle, open_toggle, read_observations, ToggleTestConfig};

#[gpui::test]
fn style_with_state_receives_pressed_and_disabled_state(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            disabled: true,
            default_pressed: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_state();
    assert!(state.pressed);
    assert!(state.disabled);
}

#[gpui::test]
fn style_with_state_reflects_pressed_transition(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    assert!(!read_observations(cx, window).last_state().pressed);

    click_toggle(cx, window);

    assert!(read_observations(cx, window).last_state().pressed);
}
