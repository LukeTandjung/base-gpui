use gpui::TestAppContext;

use super::support::{
    click_item, focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn enabled_button_fires_on_click_from_pointer_and_keyboard(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    click_item(cx, window, 0);
    assert_eq!(read_observations(cx, window).clicks, vec![0]);

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "enter");
    simulate_keys(cx, window, "space");
    assert_eq!(read_observations(cx, window).clicks, vec![0, 0, 0]);
}

#[gpui::test]
fn disabled_button_never_fires_on_click(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            second_disabled: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 1);
    let observations = read_observations(cx, window);
    assert!(observations.clicks.is_empty());
    assert_eq!(observations.focused_item(), Some(1));

    simulate_keys(cx, window, "enter space");
    assert!(read_observations(cx, window).clicks.is_empty());
}
