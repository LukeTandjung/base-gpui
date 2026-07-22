use gpui::TestAppContext;

use super::support::{
    open_tabs, read_observations, simulate_keys, TabsTestConfig, ACCOUNT, PROJECTS,
};

#[gpui::test]
fn activate_on_focus_selects_tabs_during_keyboard_navigation(cx: &mut TestAppContext) {
    let window = open_tabs(
        cx,
        TabsTestConfig {
            activate_on_focus: true,
            ..TabsTestConfig::default()
        },
    );

    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(PROJECTS));
    assert_eq!(observations.value_changes, vec![Some(PROJECTS)]);

    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.active_value(), Some(ACCOUNT));
    assert_eq!(
        observations.value_changes,
        vec![Some(PROJECTS), Some(ACCOUNT)]
    );
}
