use gpui::TestAppContext;

use super::support::{open_collapsible, read_observations, CollapsibleTestConfig};

#[gpui::test]
fn uncontrolled_initial_state_defaults_to_closed(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    let state = read_observations(cx, window).last_root_state();
    assert!(!state.open);
    assert!(state.closed);
}

#[gpui::test]
fn uncontrolled_default_open_initializes_to_open(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            default_open: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.open);
    assert!(!state.closed);
}
