use gpui::TestAppContext;

use super::support::{open_meter, read_observations, MeterTestConfig};

#[gpui::test]
fn all_five_parts_observe_the_same_derived_percentage(cx: &mut TestAppContext) {
    let window = open_meter(
        cx,
        MeterTestConfig {
            value: 30.0,
            min: 20.0,
            max: 40.0,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    for states in [
        &observations.root_states,
        &observations.track_states,
        &observations.indicator_states,
        &observations.value_states,
        &observations.label_states,
    ] {
        let state = states.last().cloned().expect("part state observed");
        assert_eq!(state.percentage, 50.0);
        assert_eq!(state.formatted, "50%");
        assert_eq!(state.value, 30.0);
    }
}
