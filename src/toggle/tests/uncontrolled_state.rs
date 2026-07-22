use gpui::TestAppContext;

use super::support::{open_toggle, read_observations, ToggleTestConfig};

#[gpui::test]
fn uncontrolled_toggle_initializes_unpressed(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert!(observations.last_state().unpressed);
}

#[gpui::test]
fn uncontrolled_toggle_initializes_pressed_from_default_pressed(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            default_pressed: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_state().pressed);
}
