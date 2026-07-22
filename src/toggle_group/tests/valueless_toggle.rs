use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn a_toggle_without_a_value_never_joins_the_group_value(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            omit_first_value: true,
            default_value: vec![shared("italic")],
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_changes, vec![(0, true)]);
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.pressed_flags(), [false, true, false]);
}
