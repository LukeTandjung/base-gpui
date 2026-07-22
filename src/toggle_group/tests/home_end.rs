use gpui::TestAppContext;

use super::support::{focus_group, open_group, read_observations, simulate_keys, GroupTestConfig};

#[gpui::test]
fn home_and_end_focus_the_first_and_last_enabled_toggle(cx: &mut TestAppContext) {
    let window = open_group(cx, GroupTestConfig::default());

    focus_group(cx, window);
    simulate_keys(cx, window, "end");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(2));

    simulate_keys(cx, window, "home");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));
}
