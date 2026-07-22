use gpui::TestAppContext;

use super::support::{blur_switch, focus_switch, open_switch, read_observations, SwitchTestConfig};

#[gpui::test]
fn focused_state_is_exposed_and_clears_on_blur(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    assert!(!read_observations(cx, window).last_root_state().focused);

    focus_switch(cx, window);
    assert!(read_observations(cx, window).last_root_state().focused);

    blur_switch(cx, window);
    assert!(!read_observations(cx, window).last_root_state().focused);
}
