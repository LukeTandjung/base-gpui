use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn controlled_activation_notifies_without_mutating_internal_value(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            controlled_value: Some(Vec::new()),
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![vec![shared("bold")]]);
    assert_eq!(observations.pressed_flags(), [false, false, false]);
}
