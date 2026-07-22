use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn arrow_navigation_skips_disabled_non_focusable_items(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            second_disabled: true,
            second_focusable: false,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(2));

    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));
}
