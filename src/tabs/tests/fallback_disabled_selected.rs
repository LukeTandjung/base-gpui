use gpui::TestAppContext;

use super::support::{
    open_tabs, read_observations, update_config, TabsTestConfig, OVERVIEW, PROJECTS,
};

#[gpui::test]
fn uncontrolled_tabs_fall_back_when_selected_tab_becomes_disabled(cx: &mut TestAppContext) {
    let window = open_tabs(
        cx,
        TabsTestConfig {
            default_value: Some(PROJECTS),
            ..TabsTestConfig::default()
        },
    );

    assert_eq!(read_observations(cx, window).active_value(), Some(PROJECTS));

    update_config(cx, window, |config| {
        config.projects_disabled = true;
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(OVERVIEW));
    assert!(observations.value_changes.is_empty());
}
