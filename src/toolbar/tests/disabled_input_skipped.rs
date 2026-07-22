use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig, LINK_INDEX,
};

#[gpui::test]
fn disabled_non_focusable_input_is_skipped_without_blocking_traversal(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            input_disabled: true,
            input_focusable: false,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "left");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(LINK_INDEX)
    );

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));
}
