use gpui::TestAppContext;

use super::support::{
    blur_trigger, focus_trigger, open_collapsible, read_observations, CollapsibleTestConfig,
};

#[gpui::test]
fn trigger_focused_state_updates_on_focus_and_blur(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    assert!(!read_observations(cx, window).last_trigger_state().focused);

    focus_trigger(cx, window);
    assert!(read_observations(cx, window).last_trigger_state().focused);

    blur_trigger(cx, window);
    assert!(!read_observations(cx, window).last_trigger_state().focused);
}
