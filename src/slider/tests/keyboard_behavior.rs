use gpui::TestAppContext;

use crate::slider::{SliderChangeReason, SliderValues};
use crate::utils::TextDirection;

use super::support::{
    focus_first_thumb, open_slider, read_observations, simulate_keys, SliderTestConfig,
};

#[gpui::test]
fn arrow_keys_step_by_step_amount(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    simulate_keys(cx, window, "up");
    simulate_keys(cx, window, "right");
    simulate_keys(cx, window, "down");
    simulate_keys(cx, window, "left");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes,
        Vec::from([
            SliderValues::Single(21.0),
            SliderValues::Single(22.0),
            SliderValues::Single(21.0),
            SliderValues::Single(20.0),
        ])
    );
    // Every applied keyboard change commits immediately.
    assert_eq!(observations.committed_values.len(), 4);
    assert_eq!(
        observations.commit_reasons.last(),
        Some(&SliderChangeReason::Keyboard)
    );
}

#[gpui::test]
fn shift_and_page_keys_use_large_step(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    simulate_keys(cx, window, "shift-up");
    simulate_keys(cx, window, "pageup");
    simulate_keys(cx, window, "pagedown");
    simulate_keys(cx, window, "shift-down");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes,
        Vec::from([
            SliderValues::Single(30.0),
            SliderValues::Single(40.0),
            SliderValues::Single(30.0),
            SliderValues::Single(20.0),
        ])
    );
}

#[gpui::test]
fn home_and_end_move_to_boundaries(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    simulate_keys(cx, window, "end");
    simulate_keys(cx, window, "home");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes,
        Vec::from([SliderValues::Single(100.0), SliderValues::Single(0.0)])
    );
}

#[gpui::test]
fn rtl_flips_left_and_right(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            direction: TextDirection::Rtl,
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    simulate_keys(cx, window, "left");
    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes,
        Vec::from([SliderValues::Single(21.0), SliderValues::Single(20.0)])
    );
}

#[gpui::test]
fn decimal_steps_stay_precision_clean(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(0.0)),
            max: 1.0,
            step: 0.1,
            large_step: 0.5,
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    for _ in 0..3 {
        simulate_keys(cx, window, "up");
    }

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes.last(),
        Some(&SliderValues::Single(0.3))
    );
}

#[gpui::test]
fn disabled_slider_ignores_keyboard(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            default_value: Some(SliderValues::Single(20.0)),
            disabled: true,
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    simulate_keys(cx, window, "up");

    let observations = read_observations(cx, window);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn controlled_slider_fires_change_without_self_mutation(cx: &mut TestAppContext) {
    let window = open_slider(
        cx,
        SliderTestConfig {
            controlled_value: Some(SliderValues::Single(20.0)),
            ..Default::default()
        },
    );

    focus_first_thumb(cx, window);
    simulate_keys(cx, window, "up");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.value_changes.first(),
        Some(&SliderValues::Single(21.0))
    );
    assert_eq!(
        observations.last_root_state().values,
        SliderValues::Single(20.0)
    );
}
