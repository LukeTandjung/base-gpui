use gpui::TestAppContext;

use super::support::{
    open_tabs, read_observations, update_config, TabsTestConfig, OVERVIEW, PROJECTS,
};

#[gpui::test]
fn uncontrolled_tabs_fall_back_when_selected_tab_is_removed(cx: &mut TestAppContext) {
    let window = open_tabs(
        cx,
        TabsTestConfig {
            default_value: Some(PROJECTS),
            ..TabsTestConfig::default()
        },
    );

    assert_eq!(read_observations(cx, window).active_value(), Some(PROJECTS));

    update_config(cx, window, |config| {
        config.include_projects = false;
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(OVERVIEW));
    assert_eq!(
        observations.tab_state(PROJECTS).map(|state| state.active),
        None
    );
    assert!(observations.value_changes.is_empty());
}
