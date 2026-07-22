use gpui::TestAppContext;

use super::support::{
    open_tabs, read_observations, simulate_keys, TabsTestConfig, ACCOUNT, OVERVIEW, PROJECTS,
};

#[gpui::test]
fn keyboard_navigation_moves_highlight_and_enter_activates_it(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());

    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(OVERVIEW));
    assert_eq!(observations.highlighted_value(), Some(PROJECTS));

    simulate_keys(cx, window, "enter");
    assert_eq!(read_observations(cx, window).active_value(), Some(PROJECTS));

    simulate_keys(cx, window, "end enter");
    assert_eq!(read_observations(cx, window).active_value(), Some(ACCOUNT));

    simulate_keys(cx, window, "home enter");
    assert_eq!(read_observations(cx, window).active_value(), Some(OVERVIEW));
}
