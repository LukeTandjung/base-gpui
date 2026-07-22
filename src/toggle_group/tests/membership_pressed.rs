use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn grouped_toggle_reports_pressed_iff_the_group_value_contains_its_value(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            multiple: true,
            default_value: vec![shared("bold"), shared("underline")],
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [true, false, true]);

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, false, true]);
}
