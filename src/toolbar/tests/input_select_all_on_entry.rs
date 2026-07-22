use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig, INPUT_INDEX,
};

#[gpui::test]
fn roving_focus_into_the_input_selects_all_its_text(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "left");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(INPUT_INDEX)
    );
    assert_eq!(
        read_observations(cx, window).last_input_state().input.value,
        "abc"
    );

    // With the whole text selected, backspace clears it in one press.
    simulate_keys(cx, window, "backspace");
    assert_eq!(
        read_observations(cx, window).last_input_state().input.value,
        ""
    );
}
