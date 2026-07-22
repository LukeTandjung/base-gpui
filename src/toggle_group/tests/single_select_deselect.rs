use gpui::TestAppContext;

use super::support::{click_item, open_group, read_observations, shared, GroupTestConfig};

#[gpui::test]
fn pressing_the_pressed_toggle_empties_the_group_value(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            default_value: vec![shared("bold")],
            ..Default::default()
        },
    );

    click_item(cx, window, 0);

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, false, false]);
    assert_eq!(
        observations.value_changes,
        vec![Vec::<gpui::SharedString>::new()]
    );
}
