use gpui::TestAppContext;

use super::support::{
    click_item, focus_group, open_group, read_observations, shared, simulate_keys, GroupTestConfig,
};

#[gpui::test]
fn disabled_toggle_is_skipped_by_roving_focus_and_cannot_activate(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            second_disabled: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 1);
    let observations = read_observations(cx, window);
    assert!(observations.pressed_changes.is_empty());
    assert!(observations.value_changes.is_empty());

    click_item(cx, window, 0);
    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![vec![shared("bold")]]);

    focus_group(cx, window);
    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_toggle(), Some(2));
}
