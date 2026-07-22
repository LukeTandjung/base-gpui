use gpui::TestAppContext;

use super::support::{open_group, read_observations, GroupTestConfig};

#[gpui::test]
fn empty_default_value_presses_no_toggle(cx: &mut TestAppContext) {
    let window = open_group(cx, GroupTestConfig::default());

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, false, false]);
    assert!(observations.value_changes.is_empty());
}
