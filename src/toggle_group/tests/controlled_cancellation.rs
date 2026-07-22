use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, GroupTestConfig};

#[gpui::test]
fn canceled_controlled_activation_calls_both_callbacks_without_state_change(
    cx: &mut TestAppContext,
) {
    let window = open_group(
        cx,
        GroupTestConfig {
            controlled_value: Some(Vec::new()),
            cancel_group: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_changes.len(), 1);
    assert_eq!(observations.value_changes.len(), 1);
    assert_eq!(observations.pressed_flags(), [false, false, false]);
}
