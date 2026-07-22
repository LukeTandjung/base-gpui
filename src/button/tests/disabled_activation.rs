use gpui::TestAppContext;

use super::support::{
    click_button, focus_button, open_button, read_observations, simulate_keys, ButtonTestConfig,
};

#[gpui::test]
fn disabled_click_never_calls_on_click(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_button(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 0);
}

#[gpui::test]
fn disabled_space_and_enter_never_call_on_click(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_button(cx, window);
    simulate_keys(cx, window, "space");
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 0);
}
