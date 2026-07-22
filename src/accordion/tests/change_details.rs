use gpui::TestAppContext;

use crate::accordion::{AccordionChangeReason, AccordionChangeSource};

use super::support::{
    click_trigger, focus_trigger, open_accordion, read_observations, simulate_keys,
    AccordionTestConfig,
};

#[gpui::test]
fn pointer_activation_reports_trigger_press_reason_and_pointer_source(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    click_trigger(cx, window, "first");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons,
        vec![AccordionChangeReason::TriggerPress]
    );
    assert_eq!(
        observations.change_sources,
        vec![AccordionChangeSource::Pointer]
    );
    assert_eq!(observations.change_cancelable, vec![true]);
    assert_eq!(observations.change_canceled, vec![false]);
}

#[gpui::test]
fn keyboard_activation_reports_trigger_press_reason_and_keyboard_source(cx: &mut TestAppContext) {
    let window = open_accordion(cx, AccordionTestConfig::default());

    focus_trigger(cx, window, 0);
    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons,
        vec![AccordionChangeReason::TriggerPress]
    );
    assert_eq!(
        observations.change_sources,
        vec![AccordionChangeSource::Keyboard]
    );
}
