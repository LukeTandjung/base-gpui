use gpui::{Modifiers, SharedString, TestAppContext, VisualTestContext};

use crate::field::FieldValidationMode;

use super::support::{
    blur_input, click_input, debug_bounds, focus_input, open_input, read_observations,
    simulate_keys, InputTestConfig,
};

#[gpui::test]
fn field_control_consumes_field_root_disabled_state(cx: &mut TestAppContext) {
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
fn field_control_inside_disabled_field_item_is_disabled(cx: &mut TestAppContext) {
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
fn field_label_click_focuses_input(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            ..Default::default()
        },
    );
    let label_bounds = debug_bounds(cx, window, "input-label").expect("label should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(label_bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();

    assert!(read_observations(cx, window).last_state().focused);
}

#[gpui::test]
fn field_tracks_filled_dirty_focused_and_touched_from_input(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(field_state.filled);
    assert!(field_state.dirty);
    assert!(field_state.focused);

    blur_input(cx, window);
    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(field_state.touched);

    focus_input(cx, window);
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-a backspace");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-a backspace");
    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(!field_state.filled);
}

#[gpui::test]
fn required_field_validation_reports_missing_value(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            field: true,
            required: true,
            field_validation_mode: FieldValidationMode::OnBlur,
            ..Default::default()
        },
    );

    click_input(cx, window);
    blur_input(cx, window);

    let observations = read_observations(cx, window);
    let validity = observations
        .field_validity_states
        .last()
        .expect("validity should be observed");
    assert!(validity.validity.state.value_missing);
    assert_eq!(validity.validity.error, SharedString::from("Required"));
}

#[gpui::test]
fn field_error_presence_updates_when_text_changes(cx: &mut TestAppContext) {
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
    assert!(debug_bounds(cx, window, "input-error").is_some());

    simulate_keys(cx, window, "b");
    assert!(debug_bounds(cx, window, "input-error").is_none());
}

#[gpui::test]
fn on_change_field_validation_runs_after_value_edits(cx: &mut TestAppContext) {
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
}
