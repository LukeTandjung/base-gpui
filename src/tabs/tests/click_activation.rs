use gpui::TestAppContext;

use super::support::{click_tab, open_tabs, read_observations, TabsTestConfig, PROJECTS};

#[gpui::test]
fn clicking_enabled_inactive_tab_selects_it(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());

    click_tab(cx, window, PROJECTS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(PROJECTS));
    assert_eq!(observations.highlighted_value(), Some(PROJECTS));
    assert_eq!(observations.value_changes, vec![Some(PROJECTS)]);
}
