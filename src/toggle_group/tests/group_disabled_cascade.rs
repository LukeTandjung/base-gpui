use gpui::TestAppContext;

use super::support::{
    click_item, focus_group, open_group, read_observations, simulate_keys, GroupTestConfig,
};

#[gpui::test]
fn group_disabled_cascades_to_every_toggle(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    click_item(cx, window, 0);
    focus_group(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert!(observations.pressed_changes.is_empty());
    assert!(observations.value_changes.is_empty());
    assert!((0..3).all(|index| observations.last_toggle_state(index).disabled));
    assert_eq!(observations.pressed_flags(), [false, false, false]);
}
