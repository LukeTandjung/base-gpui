use gpui::TestAppContext;

use super::support::{focus_group, open_group, read_observations, simulate_keys, GroupTestConfig};

#[gpui::test]
fn horizontal_arrows_move_focus_and_wrap_while_vertical_arrows_are_ignored(
    cx: &mut TestAppContext,
) {
    let window = open_group(cx, GroupTestConfig::default());

    focus_group(cx, window);
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(1));

    simulate_keys(cx, window, "down");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(1));

    simulate_keys(cx, window, "up");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(1));

    simulate_keys(cx, window, "right right");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));

    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(2));
}
