use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn home_and_end_do_not_navigate(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(1));

    simulate_keys(cx, window, "home");
    assert_eq!(read_observations(cx, window).focused_item(), Some(1));

    simulate_keys(cx, window, "end");
    assert_eq!(read_observations(cx, window).focused_item(), Some(1));
}
