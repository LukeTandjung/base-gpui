use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn pressing_a_second_toggle_keeps_the_first_pressed(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            multiple: true,
            default_value: vec![shared("bold")],
            ..Default::default()
        },
    );

    click_item(cx, window, 1);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [true, true, false]);
    assert_eq!(
        observations.value_changes,
        vec![vec![shared("bold"), shared("italic")]]
    );
}
