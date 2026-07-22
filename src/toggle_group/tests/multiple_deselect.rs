use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn unpressing_a_toggle_removes_only_its_value(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            multiple: true,
            default_value: vec![shared("bold"), shared("italic")],
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, true, false]);
    assert_eq!(observations.value_changes, vec![vec![shared("italic")]]);
}
