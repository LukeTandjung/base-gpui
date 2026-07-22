use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn rtl_flips_horizontal_arrow_navigation(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            rtl: true,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));

    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_item(), Some(1));

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));
}
