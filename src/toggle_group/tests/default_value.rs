use gpui::TestAppContext;

use super::support::{open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn default_value_presses_the_matching_toggle(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            default_value: vec![shared("italic")],
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, true, false]);
}
