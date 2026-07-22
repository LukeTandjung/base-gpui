use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn without_loop_focus_arrow_navigation_clamps_at_both_ends(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            loop_focus: false,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));

    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));

    simulate_keys(cx, window, "right right right right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(4));

    // Entering the input selects all text; the first right collapses the
    // caret to the end, the second consults the edge handler which clamps.
    simulate_keys(cx, window, "right right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(4));
}
