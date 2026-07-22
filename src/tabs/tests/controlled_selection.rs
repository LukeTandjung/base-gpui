use gpui::TestAppContext;

use super::support::{
    open_tabs, read_observations, simulate_keys, update_config, TabsTestConfig, OVERVIEW, PROJECTS,
};

#[gpui::test]
fn controlled_tabs_notify_without_mutating_internal_selection(cx: &mut TestAppContext) {
    let window = open_tabs(
        cx,
        TabsTestConfig {
            controlled_value: Some(Some(OVERVIEW)),
            ..TabsTestConfig::default()
        },
    );

    simulate_keys(cx, window, "right enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(PROJECTS)]);
    assert_eq!(observations.active_value(), Some(OVERVIEW));

    update_config(cx, window, |config| {
        config.controlled_value = Some(Some(PROJECTS));
    });

    assert_eq!(read_observations(cx, window).active_value(), Some(PROJECTS));
}
