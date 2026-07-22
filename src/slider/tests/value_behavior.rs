use gpui::{point, px, TestAppContext};

use crate::slider::{SliderChangeReason, SliderValues};

use super::support::{
    debug_bounds, mouse_down_at, mouse_move_to, mouse_up_at, open_slider, read_observations,
    SliderTestConfig,
};

#[gpui::test]
fn uncontrolled_default_initializes_to_single_min(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            min: 10.0,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().values,
        SliderValues::Single(10.0)
    );
}

#[gpui::test]
fn provided_default_value_is_used(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(30.0)),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().values,
        SliderValues::Single(30.0)
    );
    assert_eq!(observations.last_indicator_state().end_fraction, 0.3);
}

#[gpui::test]
fn unsorted_controlled_range_renders_sorted(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            controlled_value: Some(SliderValues::Range(Vec::from([80.0, 20.0]))),
            thumb_count: 2,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().values,
        SliderValues::Range(Vec::from([20.0, 80.0]))
    );
}

#[gpui::test]
fn slider_value_renders_default_joined_display(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Range(Vec::from([20.0, 80.0]))),
            thumb_count: 2,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    let value_state = observations.last_value_state();
    assert_eq!(value_state.values, Vec::from([20.0, 80.0]));
    assert_eq!(
        value_state
            .formatted_values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>(),
        Vec::from([String::from("20"), String::from("80")])
    );
}

#[gpui::test]
fn track_press_applies_closest_thumb_value(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            ..Default::default()
        },
    );

    let bounds = debug_bounds(cx, window, "slider-control").expect("control should render");
    let target = point(
        bounds.origin.x + px(bounds.size.width.as_f32() * 0.75),
        bounds.center().y,
    );
    mouse_down_at(cx, window, target);

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.change_reasons.first(),
        Some(&SliderChangeReason::TrackPress)
    );
    assert_eq!(
        observations.value_changes.first(),
        Some(&SliderValues::Single(75.0))
    );

    mouse_up_at(cx, window, target);
    let observations = read_observations(cx, window);
    assert_eq!(observations.committed_values.len(), 1);
    assert_eq!(
        observations.committed_values.first(),
        Some(&SliderValues::Single(75.0))
    );
}

#[gpui::test]
fn canceled_change_is_not_applied_and_not_committed(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            cancel_changes: true,
            ..Default::default()
        },
    );

    let bounds = debug_bounds(cx, window, "slider-control").expect("control should render");
    let target = point(
        bounds.origin.x + px(bounds.size.width.as_f32() * 0.75),
        bounds.center().y,
    );
    mouse_down_at(cx, window, target);
    mouse_up_at(cx, window, target);

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes.len(), 1);
    assert!(observations.committed_values.is_empty());
    assert_eq!(
        observations.last_root_state().values,
        SliderValues::Single(20.0)
    );
}

#[gpui::test]
fn drag_moves_value_and_commits_once_on_release(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            ..Default::default()
        },
    );

    let bounds = debug_bounds(cx, window, "slider-control").expect("control should render");
    let start = point(
        bounds.origin.x + px(bounds.size.width.as_f32() * 0.5),
        bounds.center().y,
    );
    let end = point(
        bounds.origin.x + px(bounds.size.width.as_f32() * 0.9),
        bounds.center().y,
    );
    mouse_down_at(cx, window, start);
    mouse_move_to(cx, window, end);
    mouse_up_at(cx, window, end);

    let observations = read_observations(cx, window);
    assert!(observations
        .change_reasons
        .contains(&SliderChangeReason::Drag));
    assert_eq!(observations.committed_values.len(), 1);
    assert_eq!(
        observations.committed_values.first(),
        Some(&SliderValues::Single(90.0))
    );
}

#[gpui::test]
fn disabled_slider_ignores_pointer(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            disabled: true,
            ..Default::default()
        },
    );

    let bounds = debug_bounds(cx, window, "slider-control").expect("control should render");
    let target = point(
        bounds.origin.x + px(bounds.size.width.as_f32() * 0.75),
        bounds.center().y,
    );
    mouse_down_at(cx, window, target);
    mouse_up_at(cx, window, target);

    let observations = read_observations(cx, window);
    assert!(observations.value_changes.is_empty());
    assert!(observations.committed_values.is_empty());
}

#[gpui::test]
fn indicator_matches_range_value_percents(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Range(Vec::from([20.0, 80.0]))),
            thumb_count: 2,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    let indicator = observations.last_indicator_state();
    assert_eq!(indicator.start_fraction, 0.2);
    assert_eq!(indicator.end_fraction, 0.8);
}
