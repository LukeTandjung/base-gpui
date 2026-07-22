use gpui::TestAppContext;

use super::support::{
    focus_group, open_group, read_observations, shared, simulate_keys, GroupTestConfig,
};

#[gpui::test]
fn arrow_navigation_never_changes_pressed_state_or_fires_callbacks(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            default_value: vec![shared("bold")],
            ..Default::default()
        },
    );

    focus_group(cx, window);
    simulate_keys(cx, window, "right left home end");

    let observations = read_observations(cx, window);
    assert!(observations.pressed_changes.is_empty());
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.pressed_flags(), [true, false, false]);
}
