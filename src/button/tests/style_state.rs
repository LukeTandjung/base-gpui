use gpui::TestAppContext;

use super::support::{blur_button, focus_button, open_button, read_observations, ButtonTestConfig};

#[gpui::test]
fn style_state_reflects_disabled(cx: &mut TestAppContext) {
    let window = open_button(
        cx,
        ButtonTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_state().disabled);

    let window = open_button(cx, ButtonTestConfig::default());
    assert!(!read_observations(cx, window).last_state().disabled);
}

#[gpui::test]
fn style_state_reflects_focus_and_blur(cx: &mut TestAppContext) {
    let window = open_button(cx, ButtonTestConfig::default());

    assert!(!read_observations(cx, window).last_state().focused);

    focus_button(cx, window);
    assert!(read_observations(cx, window).last_state().focused);

    blur_button(cx, window);
    assert!(!read_observations(cx, window).last_state().focused);
}
