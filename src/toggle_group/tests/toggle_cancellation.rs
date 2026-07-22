use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, GroupTestConfig};

#[gpui::test]
fn canceled_toggle_callback_prevents_the_group_commit(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            cancel_toggle: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_changes, vec![(0, true)]);
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.pressed_flags(), [false, false, false]);
}
