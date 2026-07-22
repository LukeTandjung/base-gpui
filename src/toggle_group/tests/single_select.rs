use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn pressing_a_toggle_unpresses_the_previous_one(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            default_value: vec![shared("bold")],
            ..Default::default()
        },
    );

    click_item(cx, window, 1);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, true, false]);
    assert_eq!(observations.value_changes, vec![vec![shared("italic")]]);
}
