use gpui::{Modifiers, SharedString, TestAppContext, VisualTestContext};

use crate::field::FieldValidationMode;

use super::support::{
    blur_number_input, click_selector, debug_bounds, focus_number_input, open_number_field,
    read_observations, simulate_keys, simulate_text, NumberFieldTestConfig,
};

#[gpui::test]
fn number_field_consumes_field_root_disabled_state(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            field: true,
            field_root_disabled: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_root_state().disabled);
}

#[gpui::test]
fn number_field_inside_disabled_field_item_is_disabled(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            field: true,
            field_item_disabled: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_root_state().disabled);
}

#[gpui::test]
fn field_label_click_focuses_number_input(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            field: true,
            ..Default::default()
        },
    );
    let label_bounds = debug_bounds(cx, window, "number-label").expect("label should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(label_bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();

    assert!(read_observations(cx, window).last_root_state().focused);
}

#[gpui::test]
fn field_tracks_filled_dirty_focused_and_touched(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            field: true,
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
    simulate_text(cx, window, "5");

    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(field_state.filled);
    assert!(field_state.dirty);
    assert!(field_state.focused);

    blur_number_input(cx, window);
    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(field_state.touched);

    focus_number_input(cx, window);
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
fn required_field_validation_reports_missing_number_value(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            field: true,
            required: true,
            field_validation_mode: FieldValidationMode::OnBlur,
            ..Default::default()
        },
    );

    click_selector(cx, window, "number-input");
    blur_number_input(cx, window);

    let observations = read_observations(cx, window);
    let validity = observations
        .field_validity_states
        .last()
        .expect("validity should be observed");
    assert!(validity.validity.state.value_missing);
    assert_eq!(validity.validity.error, SharedString::from("Required"));
}

#[gpui::test]
fn on_change_field_validation_tracks_empty_number_value(cx: &mut TestAppContext) {
    let window = open_number_field(
        cx,
        NumberFieldTestConfig {
            field: true,
            default_value: Some(1.0),
            required: true,
            field_validation_mode: FieldValidationMode::OnChange,
            ..Default::default()
        },
    );

    focus_number_input(cx, window);
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
