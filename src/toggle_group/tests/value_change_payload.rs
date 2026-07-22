use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn on_value_change_receives_the_full_next_value_once_per_activation(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            multiple: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 0);
    click_item(cx, window, 1);

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes,
        vec![vec![shared("bold")], vec![shared("bold"), shared("italic")]]
    );
    assert_eq!(observations.pressed_changes, vec![(0, true), (1, true)]);
}
