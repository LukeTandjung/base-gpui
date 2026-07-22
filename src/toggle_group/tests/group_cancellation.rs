use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};
use crate::toggle_group::ToggleGroupValueChangeSource;

#[gpui::test]
fn canceled_group_callback_leaves_value_and_pressed_state_unchanged(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            cancel_group: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_changes, vec![(0, true)]);
    assert_eq!(observations.value_changes, vec![vec![shared("bold")]]);
    assert_eq!(
        observations.value_change_sources,
        vec![ToggleGroupValueChangeSource::Pointer]
    );
    assert_eq!(observations.pressed_flags(), [false, false, false]);
}
