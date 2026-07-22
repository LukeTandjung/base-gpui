use gpui::TestAppContext;

use crate::toolbar::ToolbarOrientation;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn vertical_arrows_move_focus_while_horizontal_arrows_are_inert(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            orientation: ToolbarOrientation::Vertical,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));

    simulate_keys(cx, window, "down");
    assert_eq!(read_observations(cx, window).focused_item(), Some(1));

    simulate_keys(cx, window, "up");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));

    simulate_keys(cx, window, "left right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));
}
