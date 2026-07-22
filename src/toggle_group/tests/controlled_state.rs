use gpui::TestAppContext;

use super::support::{open_group, read_observations, shared, update_config, GroupTestConfig};

#[gpui::test]
fn controlled_group_reflects_external_value_changes(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            controlled_value: Some(vec![shared("bold")]),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [true, false, false]);

    update_config(cx, window, |config| {
        config.controlled_value = Some(vec![shared("underline")]);
    });

    let observations = read_observations(cx, window);
    assert_eq!(observations.pressed_flags(), [false, false, true]);
}
