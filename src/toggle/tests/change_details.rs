use gpui::TestAppContext;

use super::support::{click_toggle, open_toggle, read_observations, ToggleTestConfig};
use crate::toggle::TogglePressedChangeReason;

#[gpui::test]
fn on_pressed_change_is_called_exactly_once_with_next_pressed_value(cx: &mut TestAppContext) {
    let window = open_toggle(cx, ToggleTestConfig::default());

    click_toggle(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![true]);
    assert_eq!(
        observations.change_reasons,
        vec![TogglePressedChangeReason::None]
    );
    assert_eq!(observations.change_cancelable, vec![true]);
}
