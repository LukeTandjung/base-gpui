use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig, INPUT_INDEX,
};

#[gpui::test]
fn arrows_leave_the_input_only_at_the_matching_caret_edge(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "left");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );

    // Entering the input selected all text; the first left collapses the
    // caret to position 0 (native caret movement, no navigation) and the
    // second leaves backward.
    simulate_keys(cx, window, "left");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );

    simulate_keys(cx, window, "left");
    assert_eq!(read_observations(cx, window).focused_item(), Some(3));

    // Back into the input; the first right collapses the caret to the end
    // and the second leaves forward, wrapping to the first item.
    simulate_keys(cx, window, "right");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );

    simulate_keys(cx, window, "right");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(0));
}
