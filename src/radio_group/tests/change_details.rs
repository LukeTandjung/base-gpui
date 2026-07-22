use gpui::TestAppContext;

use crate::radio_group::{RadioGroupValueChangeReason, RadioGroupValueChangeSource};

use super::support::{
    click_radio, open_radio_group, read_observations, RadioGroupTestConfig, EXPRESS,
};

#[gpui::test]
fn on_value_change_receives_next_value_and_pointer_details(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(EXPRESS)]);
    assert_eq!(
        observations.change_reasons,
        vec![RadioGroupValueChangeReason::None]
    );
    assert_eq!(
        observations.change_sources,
        vec![RadioGroupValueChangeSource::Pointer]
    );
    assert_eq!(observations.change_cancelable, vec![true]);
    assert_eq!(observations.change_canceled, vec![false]);
}
