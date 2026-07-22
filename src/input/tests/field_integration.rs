use gpui::{SharedString, TestAppContext};

use crate::field::FieldValidationMode;

use super::support::{
    blur_input, click_label, focus_input, open_input, read_observations, simulate_keys,
    InputTestConfig,
};

#[gpui::test]
fn public_input_consumes_field_root_disabled_state(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            field_root_disabled: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_state().disabled);
}

#[gpui::test]
fn public_input_inside_disabled_field_item_is_disabled(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            field_item_disabled: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_state().disabled);
}

#[gpui::test]
fn field_label_click_focuses_public_input(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            ..Default::default()
        },
    );

    click_label(cx, window);

    assert!(read_observations(cx, window).last_state().focused);
}

#[gpui::test]
fn field_tracks_public_input_filled_dirty_focused_and_touched(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    let state = read_observations(cx, window).last_state();
    assert!(state.filled);
    assert!(state.dirty);
    assert!(state.focused);
    assert!(!state.touched);

    blur_input(cx, window);
    let state = read_observations(cx, window).last_state();
    assert!(state.touched);
}

#[gpui::test]
fn required_public_input_reports_field_value_missing_when_empty(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            required: true,
            field_validation_mode: FieldValidationMode::OnBlur,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    blur_input(cx, window);

    let observations = read_observations(cx, window);
    let validity = observations
        .field_validity_states
        .last()
        .expect("validity should be observed");
    assert!(validity.validity.state.value_missing);
    assert_eq!(validity.validity.error, SharedString::from("Required"));
    assert!(observations.last_state().invalid);
}

#[gpui::test]
fn public_input_on_change_field_validation_updates_error_presence(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            required: true,
            field_validation_mode: FieldValidationMode::OnChange,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-a backspace");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-a backspace");

    let observations = read_observations(cx, window);
    let validity = observations
        .field_validity_states
        .last()
        .expect("validity should be observed");
    assert!(validity.validity.state.value_missing);
    assert!(observations.last_state().invalid);
}
