use gpui::{px, TestAppContext};

use crate::number_field::NumberFieldCommitReason;

use super::support::{
    assert_float_eq, drag_scrub, end_scrub_at, open_number_field, read_observations, start_scrub,
    NumberFieldTestConfig,
};

#[gpui::test]
fn scrub_drag_changes_value_by_step_units(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            include_scrub_area: true,
            default_value: Some(1.0),
            step: 2.0,
            pixel_sensitivity: 2.0,
            ..Default::default()
        },
    );

    drag_scrub(cx, window, px(4.0));

    let observations = read_observations(cx, window);
    assert_float_eq(observations.last_root_state().value, Some(5.0));
    assert_eq!(observations.committed_values.last(), Some(&Some(5.0)));
    assert_eq!(
        observations.commit_reasons.last(),
        Some(&NumberFieldCommitReason::Scrub)
    );
}

#[gpui::test]
fn scrub_state_and_cursor_present_state_are_exposed(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            include_scrub_area: true,
            default_value: Some(1.0),
            ..Default::default()
        },
    );

    let start = start_scrub(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().scrubbing);
    assert!(observations
        .scrub_area_states
        .iter()
        .any(|state| state.root.scrubbing));
    assert!(observations
        .scrub_cursor_states
        .iter()
        .any(|state| state.present));

    end_scrub_at(cx, window, start);
    assert!(!read_observations(cx, window).last_root_state().scrubbing);
}
