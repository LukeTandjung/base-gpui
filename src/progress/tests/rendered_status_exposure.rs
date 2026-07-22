use gpui::TestAppContext;

use crate::progress::ProgressStatus;

use super::support::{open_progress, read_observations, ProgressTestConfig};

#[gpui::test]
fn all_five_parts_observe_the_same_status_and_percentage(cx: &mut TestAppContext) {
    let window = open_progress(
        cx,
        ProgressTestConfig {
            value: Some(30.0),
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
        assert_eq!(state.status, ProgressStatus::Progressing);
        assert_eq!(state.percentage, Some(30.0));
    }
}

#[gpui::test]
fn all_five_parts_observe_indeterminate(cx: &mut TestAppContext) {
    let window = open_progress(cx, ProgressTestConfig::default());

    let observations = read_observations(cx, window);
    for states in [
        &observations.root_states,
        &observations.track_states,
        &observations.indicator_states,
        &observations.value_states,
        &observations.label_states,
    ] {
        let state = states.last().cloned().expect("part state observed");
        assert_eq!(state.status, ProgressStatus::Indeterminate);
        assert_eq!(state.percentage, None);
    }
}
