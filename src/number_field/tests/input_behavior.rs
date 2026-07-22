use gpui::{SharedString, TestAppContext};

use crate::number_field::{NumberFieldChangeReason, NumberFieldCommitReason};

use super::support::{
    assert_float_eq, blur_number_input, focus_number_input, open_number_field, read_observations,
    shared, simulate_keys, simulate_text, update_config, NumberFieldTestConfig,
};

#[gpui::test]
fn default_and_controlled_values_render_in_input(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(12.5),
            ..Default::default()
        },
    );
    assert_eq!(
        read_observations(cx, window)
            .last_input_state()
            .root
            .input_value,
        shared("12.5")
    );

    update_config(cx, window, |config| {
        config.controlled_value = Some(Some(8.0));
    });
    assert_eq!(
        read_observations(cx, window)
            .last_input_state()
            .root
            .input_value,
        shared("8")
    );

    update_config(cx, window, |config| {
        config.controlled_value = Some(None);
    });
    assert_eq!(
        read_observations(cx, window)
            .last_input_state()
            .root
            .input_value,
        SharedString::default()
    );
}

#[gpui::test]
fn typing_parseable_text_updates_value_and_input_text(cx: &mut TestAppContext) {
    let window = open_number_field(cx, NumberFieldTestConfig::default());

    focus_number_input(cx, window);
    simulate_text(cx, window, "42");

    let observations = read_observations(cx, window);
    assert_float_eq(observations.last_root_state().value, Some(42.0));
    assert_eq!(
        observations.last_input_state().root.input_value,
        shared("42")
    );
    assert_eq!(observations.value_changes, vec![Some(4.0), Some(42.0)]);
}

#[gpui::test]
fn clearing_input_sets_numeric_value_to_none(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(4.0),
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-a backspace");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-a backspace");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, None);
    assert_eq!(
        observations.change_reasons.last(),
        Some(&NumberFieldChangeReason::InputClear)
    );
}

#[gpui::test]
fn invalid_intermediate_text_is_preserved_until_blur(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(4.0),
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-a");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-a");
    simulate_text(cx, window, "-");

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_input_state().root.input_value,
        shared("-")
    );
    assert_eq!(observations.last_root_state().value, Some(4.0));

    blur_number_input(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_input_state().root.input_value,
        shared("4")
    );
    assert_eq!(
        observations.commit_reasons.last(),
        Some(&NumberFieldCommitReason::InputBlur)
    );
}

#[gpui::test]
fn blur_after_typing_commits_value(cx: &mut TestAppContext) {
    let window = open_number_field(cx, NumberFieldTestConfig::default());

    focus_number_input(cx, window);
    simulate_text(cx, window, "5");
    blur_number_input(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.committed_values, vec![Some(5.0)]);
    assert_eq!(
        observations.commit_reasons,
        vec![NumberFieldCommitReason::InputBlur]
    );
}

#[gpui::test]
fn style_states_expose_common_flags(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            required: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert_eq!(state.value, Some(1.0));
    assert_eq!(state.input_value, shared("1"));
    assert!(state.required);
    assert!(state.filled);
    assert!(!state.dirty);
    assert!(!state.touched);
    assert!(!state.focused);
    assert!(!state.scrubbing);

    focus_number_input(cx, window);
    simulate_text(cx, window, "2");
    blur_number_input(cx, window);

    let state = read_observations(cx, window).last_root_state();
    assert_eq!(state.value, Some(12.0));
    assert!(state.dirty);
    assert!(state.touched);
    assert!(!state.focused);

    let disabled = open_number_field(
        cx,
        NumberFieldTestConfig {
            disabled: true,
            read_only: true,
            required: true,
            ..Default::default()
        },
    );
    let state = read_observations(cx, disabled).last_root_state();
    assert!(state.disabled);
    assert!(state.read_only);
    assert!(state.required);
}

#[gpui::test]
fn disabled_and_read_only_ignore_typing(cx: &mut TestAppContext) {
    let disabled = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            disabled: true,
            ..Default::default()
        },
    );
    focus_number_input(cx, disabled);
    simulate_text(cx, disabled, "2");
    assert_float_eq(
        read_observations(cx, disabled).last_root_state().value,
        Some(1.0),
    );

    let read_only = open_number_field(
        cx,
        NumberFieldTestConfig {
            default_value: Some(1.0),
            read_only: true,
            ..Default::default()
        },
    );
    focus_number_input(cx, read_only);
    simulate_text(cx, read_only, "2");
    assert_float_eq(
        read_observations(cx, read_only).last_root_state().value,
        Some(1.0),
    );
}
