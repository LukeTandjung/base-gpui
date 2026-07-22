use gpui::TestAppContext;

use super::support::{click_button, open_button, read_observations, ButtonTestConfig};

#[gpui::test]
fn click_invokes_on_click_exactly_once(cx: &mut TestAppContext) {
    let window = open_button(cx, ButtonTestConfig::default());

    click_button(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 1);
}

#[gpui::test]
fn click_does_not_double_fire_through_the_keyboard_action_path(cx: &mut TestAppContext) {
    let window = open_button(cx, ButtonTestConfig::default());

    click_button(cx, window);
    click_button(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 2);
}
