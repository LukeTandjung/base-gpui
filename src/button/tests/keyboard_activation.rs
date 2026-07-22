use gpui::TestAppContext;

use super::support::{
    focus_button, open_button, read_observations, simulate_keys, ButtonTestConfig,
};

#[gpui::test]
fn space_invokes_on_click_when_focused(cx: &mut TestAppContext) {
    let window = open_button(cx, ButtonTestConfig::default());

    focus_button(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 1);
}

#[gpui::test]
fn enter_invokes_on_click_when_focused(cx: &mut TestAppContext) {
    let window = open_button(cx, ButtonTestConfig::default());

    focus_button(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 1);
}
