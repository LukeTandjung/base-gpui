use gpui::TestAppContext;

use super::support::{click_toggle, open_toggle, read_observations, ToggleTestConfig};

#[gpui::test]
fn click_flips_unpressed_to_pressed(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![true]);
}

#[gpui::test]
fn click_flips_pressed_to_unpressed(cx: &mut TestAppContext) {
    let window = open_toggle(
        cx,
        ToggleTestConfig {
            default_pressed: true,
            ..Default::default()
        },
    );

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().pressed);
    assert_eq!(observations.value_changes, vec![false]);
}
