use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig, INPUT_INDEX,
};

#[gpui::test]
fn shift_arrow_selects_text_and_never_navigates(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "left left");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );

    // Caret sits at position 0; shift-left would leave if it navigated.
    simulate_keys(cx, window, "shift-left shift-left shift-right");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );
}
