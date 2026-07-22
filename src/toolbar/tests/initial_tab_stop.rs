use gpui::TestAppContext;

use super::support::{focus_toolbar, open_toolbar, read_observations, ToolbarTestConfig};

#[gpui::test]
fn initial_tab_stop_moves_off_a_disabled_non_focusable_first_item(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            first_disabled: true,
            first_focusable: false,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    assert_eq!(read_observations(cx, window).focused_item(), Some(1));
}
