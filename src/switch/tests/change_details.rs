use gpui::TestAppContext;

use crate::switch::{SwitchCheckedChangeReason, SwitchCheckedChangeSource};

use super::support::{
    click_switch, focus_switch, open_switch, read_observations, simulate_keys, SwitchTestConfig,
};

#[gpui::test]
fn pointer_change_details_include_reason_source_and_cancelability(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    click_switch(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons,
        vec![SwitchCheckedChangeReason::None]
    );
    assert_eq!(
        observations.change_sources,
        vec![SwitchCheckedChangeSource::Pointer]
    );
    assert_eq!(observations.change_cancelable, vec![true]);
    assert_eq!(observations.change_canceled, vec![false]);
}

#[gpui::test]
fn keyboard_change_details_include_keyboard_source(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    focus_switch(cx, window);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons,
        vec![SwitchCheckedChangeReason::None]
    );
    assert_eq!(
        observations.change_sources,
        vec![SwitchCheckedChangeSource::Keyboard]
    );
    assert_eq!(observations.change_cancelable, vec![true]);
}
