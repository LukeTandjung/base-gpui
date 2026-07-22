use gpui::TestAppContext;

use super::support::{
    blur_trigger, focus_trigger, open_accordion, read_observations, AccordionTestConfig,
};

#[gpui::test]
fn trigger_focused_state_updates_on_focus_and_blur(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    assert!(!read_observations(cx, window).trigger_state_at(0).focused);

    focus_trigger(cx, window, 0);
    assert!(read_observations(cx, window).trigger_state_at(0).focused);

    blur_trigger(cx, window);
    assert!(!read_observations(cx, window).trigger_state_at(0).focused);
}
