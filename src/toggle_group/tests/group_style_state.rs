use gpui::TestAppContext;

use super::support::{open_group, read_observations, GroupTestConfig};
use crate::toggle_group::ToggleGroupOrientation;

#[gpui::test]
fn group_style_state_reports_disabled_orientation_and_multiple(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        GroupTestConfig {
            multiple: true,
            orientation: ToggleGroupOrientation::Vertical,
            disabled: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_group_state();
    assert!(state.disabled);
    assert_eq!(state.orientation, ToggleGroupOrientation::Vertical);
    assert!(state.multiple);
}
