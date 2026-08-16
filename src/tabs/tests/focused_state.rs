use gpui::TestAppContext;

use crate::tabs::tests::support::{
    click_tab, open_tabs, read_observations, TabsTestConfig, OVERVIEW, PROJECTS,
};

#[gpui::test]
fn tab_style_state_exposes_actual_focus(cx: &mut TestAppContext) {
    let window = open_tabs(cx, TabsTestConfig::default());

    let initial = read_observations(cx, window);
    assert!(initial.tab_state(OVERVIEW).unwrap().focused);
    assert!(!initial.tab_state(PROJECTS).unwrap().focused);

    click_tab(cx, window, PROJECTS);

    let after_click = read_observations(cx, window);
    assert!(!after_click.tab_state(OVERVIEW).unwrap().focused);
    assert!(after_click.tab_state(PROJECTS).unwrap().focused);
}
