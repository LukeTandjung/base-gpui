use gpui::TestAppContext;

use super::support::{open_tabs, read_observations, simulate_keys, TabsTestConfig, PROJECTS};

#[gpui::test]
fn indicator_state_tracks_selected_tab_position_and_size(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());

    let initial = read_observations(cx, window)
        .last_indicator_state()
        .expect("indicator state should be recorded");
    let initial_position = initial
        .active_tab_position
        .expect("initial indicator position should be measured");
    let initial_size = initial
        .active_tab_size
        .expect("initial indicator size should be measured");

    simulate_keys(cx, window, "right enter");

    let next_observations = read_observations(cx, window);
    assert_eq!(next_observations.active_value(), Some(PROJECTS));

    let next = next_observations
        .last_indicator_state()
        .expect("indicator state should be recorded after selection");
    let next_position = next
        .active_tab_position
        .expect("next indicator position should be measured");
    let next_size = next
        .active_tab_size
        .expect("next indicator size should be measured");

    assert_ne!(initial_position.left, next_position.left);
    assert_eq!(initial_size.height, next_size.height);
    assert_eq!(initial_size.width, next_size.width);
}
