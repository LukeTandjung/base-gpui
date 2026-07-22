use gpui::TestAppContext;

use crate::collapsible::{CollapsibleOpenChangeReason, CollapsibleOpenChangeSource};

use super::support::{
    click_trigger, focus_trigger, open_collapsible, read_observations, simulate_keys,
    CollapsibleTestConfig,
};

#[gpui::test]
fn pointer_activation_reports_trigger_press_reason_and_pointer_source(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons,
        vec![CollapsibleOpenChangeReason::TriggerPress]
    );
    assert_eq!(
        observations.change_sources,
        vec![CollapsibleOpenChangeSource::Pointer]
    );
    assert_eq!(observations.change_cancelable, vec![true]);
    assert_eq!(observations.change_canceled, vec![false]);
}

#[gpui::test]
fn keyboard_activation_reports_trigger_press_reason_and_keyboard_source(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    focus_trigger(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons,
        vec![CollapsibleOpenChangeReason::TriggerPress]
    );
    assert_eq!(
        observations.change_sources,
        vec![CollapsibleOpenChangeSource::Keyboard]
    );
}
