use gpui::TestAppContext;

use super::support::{focus_group, open_group, read_observations, simulate_keys, GroupTestConfig};

#[gpui::test]
fn rtl_flips_horizontal_arrow_navigation(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            rtl: true,
            ..Default::default()
        },
    );

    focus_group(cx, window);
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));

    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(1));

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(0));
}
