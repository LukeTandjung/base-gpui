use gpui::TestAppContext;

use super::support::{
    click_button, focus_button, open_button, read_observations, simulate_keys, ButtonTestConfig,
};

#[gpui::test]
fn focusable_when_disabled_button_remains_a_tab_stop(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            focusable_when_disabled: true,
        },
    );

    focus_button(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_state().focused);
}

#[gpui::test]
fn focusable_when_disabled_button_never_calls_on_click(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            focusable_when_disabled: true,
        },
    );

    focus_button(cx, window);
    simulate_keys(cx, window, "space");
    simulate_keys(cx, window, "enter");
    click_button(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.click_count, 0);
}
