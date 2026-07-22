use gpui::TestAppContext;

use crate::number_field::{NumberFieldChangeReason, NumberFieldCommitReason};

use super::support::{
    assert_float_eq, click_selector, focus_number_input, open_number_field, read_observations,
    simulate_keys, simulate_wheel, NumberFieldTestConfig,
};

#[gpui::test]
fn increment_button_increments_by_step(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            step: 2.0,
            ..Default::default()
        },
    );

    click_selector(cx, window, "number-increment");

    let observations = read_observations(cx, window);
    assert_float_eq(observations.last_root_state().value, Some(3.0));
    assert_eq!(observations.value_changes, vec![Some(3.0)]);
    assert_eq!(
        observations.change_reasons,
        vec![NumberFieldChangeReason::IncrementPress]
    );
    assert_eq!(observations.committed_values, vec![Some(3.0)]);
    assert_eq!(
        observations.commit_reasons,
        vec![NumberFieldCommitReason::IncrementPress]
    );
}

#[gpui::test]
fn decrement_button_decrements_by_step(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(3.0),
            step: 2.0,
            ..Default::default()
        },
    );

    click_selector(cx, window, "number-decrement");

    let observations = read_observations(cx, window);
    assert_float_eq(observations.last_root_state().value, Some(1.0));
    assert_eq!(observations.value_changes, vec![Some(1.0)]);
    assert_eq!(
        observations.change_reasons,
        vec![NumberFieldChangeReason::DecrementPress]
    );
    assert_eq!(observations.committed_values, vec![Some(1.0)]);
    assert_eq!(
        observations.commit_reasons,
        vec![NumberFieldCommitReason::DecrementPress]
    );
}

#[gpui::test]
fn stepper_states_expose_boundaries(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(10.0),
            min: Some(0.0),
            max: Some(10.0),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(!observations.last_increment_state().can_increment);
    assert!(observations.last_decrement_state().can_decrement);

    click_selector(cx, window, "number-decrement");
    let observations = read_observations(cx, window);
    assert!(observations.last_increment_state().can_increment);
    assert!(observations.last_decrement_state().can_decrement);
}

#[gpui::test]
fn arrow_up_and_down_step_while_input_is_focused(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            step: 2.0,
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
    simulate_keys(cx, window, "up");
    assert_float_eq(
        read_observations(cx, window).last_root_state().value,
        Some(3.0),
    );

    simulate_keys(cx, window, "down");
    assert_float_eq(
        read_observations(cx, window).last_root_state().value,
        Some(1.0),
    );
}

#[gpui::test]
fn shift_and_alt_arrows_use_large_and_small_steps(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            small_step: 0.25,
            large_step: 5.0,
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
    simulate_keys(cx, window, "shift-up");
    assert_float_eq(
        read_observations(cx, window).last_root_state().value,
        Some(6.0),
    );

    simulate_keys(cx, window, "alt-down");
    assert_float_eq(
        read_observations(cx, window).last_root_state().value,
        Some(5.75),
    );
}

#[gpui::test]
fn wheel_is_noop_by_default_and_steps_when_enabled(cx: &mut TestAppContext) {
    let no_wheel = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            step: 2.0,
            ..Default::default()
        },
    );

    simulate_wheel(cx, no_wheel, "number-input", gpui::px(-20.0));
    assert_float_eq(
        read_observations(cx, no_wheel).last_root_state().value,
        Some(1.0),
    );

    let wheel = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            step: 2.0,
            allow_wheel_scrub: true,
            ..Default::default()
        },
    );

    simulate_wheel(cx, wheel, "number-input", gpui::px(-20.0));
    let observations = read_observations(cx, wheel);
    assert_float_eq(observations.last_root_state().value, Some(3.0));
    assert_eq!(
        observations.change_reasons.last(),
        Some(&NumberFieldChangeReason::Wheel)
    );
    assert_eq!(
        observations.commit_reasons.last(),
        Some(&NumberFieldCommitReason::Wheel)
    );
}

#[gpui::test]
fn home_and_end_move_to_min_and_max(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(5.0),
            min: Some(0.0),
            max: Some(10.0),
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
    simulate_keys(cx, window, "home");
    assert_float_eq(
        read_observations(cx, window).last_root_state().value,
        Some(0.0),
    );

    simulate_keys(cx, window, "end");
    assert_float_eq(
        read_observations(cx, window).last_root_state().value,
        Some(10.0),
    );
}
