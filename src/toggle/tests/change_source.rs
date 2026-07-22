use gpui::TestAppContext;

use super::support::{
    click_toggle, focus_toggle, open_toggle, read_observations, simulate_keys, ToggleTestConfig,
};
use crate::toggle::TogglePressedChangeSource;

#[gpui::test]
fn pointer_activation_reports_pointer_source(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_sources,
        vec![TogglePressedChangeSource::Pointer]
    );
}

#[gpui::test]
fn keyboard_activation_reports_keyboard_source(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    focus_toggle(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_sources,
        vec![TogglePressedChangeSource::Keyboard]
    );
}
