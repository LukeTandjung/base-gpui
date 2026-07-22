use gpui::TestAppContext;

use super::support::{
    click_button, focus_button, open_button, read_observations, ButtonTestConfig,
};

#[gpui::test]
fn disabled_button_is_not_a_tab_stop(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_button(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_state().focused);
}

#[gpui::test]
fn clicking_disabled_button_does_not_focus_it(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_button(cx, window);

    let observations = read_observations(cx, window);
    assert!(
        !observations.last_state().focused,
        "clicking a disabled button must not paint the focus ring"
    );
}
