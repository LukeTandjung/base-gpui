use gpui::TestAppContext;

use super::support::{
    click_tab, open_tabs, read_observations, simulate_keys, TabsTestConfig, ACCOUNT, OVERVIEW,
    PROJECTS,
};

#[gpui::test]
fn disabled_tabs_do_not_activate_and_are_skipped_by_keyboard(cx: &mut TestAppContext) {
    let window = open_tabs(
        cx,
        TabsTestConfig {
            projects_disabled: true,
            ..TabsTestConfig::default()
        },
    );

    click_tab(cx, window, PROJECTS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(OVERVIEW));
    assert_eq!(
        observations.tab_state(PROJECTS).map(|state| state.disabled),
        Some(true)
    );
    assert!(observations.value_changes.is_empty());

    simulate_keys(cx, window, "right enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(ACCOUNT));
    assert_eq!(observations.active_value(), Some(ACCOUNT));
    assert_eq!(observations.value_changes, vec![Some(ACCOUNT)]);
}
