use gpui::TestAppContext;

use super::support::{
    click_item, focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
    LINK_INDEX,
};

#[gpui::test]
fn link_fires_on_click_from_pointer_and_keyboard(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    click_item(cx, window, LINK_INDEX);
    assert_eq!(read_observations(cx, window).clicks, vec![LINK_INDEX]);

    simulate_keys(cx, window, "enter space");
    assert_eq!(
        read_observations(cx, window).clicks,
        vec![LINK_INDEX, LINK_INDEX, LINK_INDEX]
    );
}

#[gpui::test]
fn link_stays_navigable_inside_a_disabled_toolbar(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            toolbar_disabled: true,
            second_focusable: false,
            second_disabled: false,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    // Two lefts traverse the (disabled but focusable) input: entry selects
    // all, the next left collapses the caret to 0, and the third leaves
    // backward onto the link.
    simulate_keys(cx, window, "left left left");
    assert_eq!(
        read_observations(cx, window).focused_item(),
        Some(LINK_INDEX)
    );
}
