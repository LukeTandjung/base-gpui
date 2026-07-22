use gpui::TestAppContext;

use super::support::{
    debug_bounds, open_tabs, read_observations, simulate_keys, update_config, TabsTestConfig,
    PROJECTS,
};

#[gpui::test]
fn panels_are_omitted_or_hidden_based_on_keep_mounted(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.panel_state(PROJECTS).map(|state| state.hidden),
        Some(true)
    );
    assert!(debug_bounds(cx, window, "panel-projects").is_none());

    update_config(cx, window, |config| {
        config.keep_mounted_projects = true;
    });

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.panel_state(PROJECTS).map(|state| state.hidden),
        Some(true)
    );
    assert!(debug_bounds(cx, window, "panel-projects").is_some());

    simulate_keys(cx, window, "right enter");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.panel_state(PROJECTS).map(|state| state.hidden),
        Some(false)
    );
    assert!(debug_bounds(cx, window, "panel-projects").is_some());
}
