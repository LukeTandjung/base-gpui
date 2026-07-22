use gpui::TestAppContext;

use super::support::{focus_group, open_group, read_observations, simulate_keys, GroupTestConfig};
use crate::toggle_group::ToggleGroupOrientation;

#[gpui::test]
fn vertical_arrows_move_focus_and_horizontal_arrows_are_ignored(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            orientation: ToggleGroupOrientation::Vertical,
            ..Default::default()
        },
    );

    focus_group(cx, window);
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));

    simulate_keys(cx, window, "down");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(1));

    simulate_keys(cx, window, "left right");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(1));

    simulate_keys(cx, window, "up");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));
}
