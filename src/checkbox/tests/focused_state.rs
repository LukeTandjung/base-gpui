use gpui::TestAppContext;

use super::support::{focus_checkbox, open_checkbox, read_observations, CheckboxTestConfig};

#[gpui::test]
fn focused_state_is_exposed(cx: &mut TestAppContext) {
    let window = open_checkbox(cx, CheckboxTestConfig::default());

    assert!(!read_observations(cx, window).last_root_state().focused);

    focus_checkbox(cx, window);

    assert!(read_observations(cx, window).last_root_state().focused);
}
