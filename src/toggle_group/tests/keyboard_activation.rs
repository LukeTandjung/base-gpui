use gpui::TestAppContext;

use super::support::{
    focus_group, open_group, read_observations, shared, simulate_keys, GroupTestConfig,
};
use crate::toggle_group::ToggleGroupValueChangeSource;

#[gpui::test]
fn space_and_enter_toggle_the_focused_item_with_keyboard_source(cx: &mut TestAppContext) {
    let window = open_group(cx, GroupTestConfig::default());

    focus_group(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![vec![shared("bold")]]);
    assert_eq!(observations.pressed_flags(), [true, false, false]);

    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes,
        vec![vec![shared("bold")], Vec::<gpui::SharedString>::new()]
    );
    assert_eq!(
        observations.value_change_sources,
        vec![
            ToggleGroupValueChangeSource::Keyboard,
            ToggleGroupValueChangeSource::Keyboard
        ]
    );
}
