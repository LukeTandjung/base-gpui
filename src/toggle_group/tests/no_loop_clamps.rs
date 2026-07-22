use gpui::TestAppContext;

use super::support::{focus_group, open_group, read_observations, simulate_keys, GroupTestConfig};

#[gpui::test]
fn loop_focus_false_clamps_at_both_ends(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            loop_focus: false,
            ..Default::default()
        },
    );

    focus_group(cx, window);
    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));

    simulate_keys(cx, window, "right right right");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(2));
}
