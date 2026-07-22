use gpui::TestAppContext;

use super::support::{
    open_tabs, read_observations, simulate_keys, update_config, TabsTestConfig, ACCOUNT, OVERVIEW,
};

#[gpui::test]
fn loop_focus_wraps_when_enabled_and_clamps_when_disabled(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());

    simulate_keys(cx, window, "left");
    assert_eq!(
        read_observations(cx, window).highlighted_value(),
        Some(ACCOUNT)
    );

    update_config(cx, window, |config| {
        config.loop_focus = false;
    });

    simulate_keys(cx, window, "right");
    assert_eq!(
        read_observations(cx, window).highlighted_value(),
        Some(ACCOUNT)
    );

    simulate_keys(cx, window, "home left");
    assert_eq!(
        read_observations(cx, window).highlighted_value(),
        Some(OVERVIEW)
    );
}
