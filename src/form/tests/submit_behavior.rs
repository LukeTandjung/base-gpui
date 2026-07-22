use super::support::{
    blur, focus_input, open_form, read_observations, simulate_keys, submit_form, text_value,
    validate_form, FormTestConfig,
};
use crate::{field::FieldValidationMode, form::FormSubmitReason};

#[gpui::test]
fn submit_with_valid_input_calls_submit_with_values(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            value: "luke@example.com".into(),
            required: true,
            ..Default::default()
        },
    );

    submit_form(cx, window);
    let observations = read_observations(cx, window);

    assert_eq!(observations.submissions.len(), 1);
    assert_eq!(
        observations.submissions[0].details.reason,
        FormSubmitReason::Programmatic
    );
    assert_eq!(
        observations.submissions[0].values.get("email"),
        Some(&text_value("luke@example.com"))
    );
}

#[gpui::test]
fn submit_with_empty_required_input_blocks_submit_and_marks_field_invalid(
    cx: &mut gpui::TestAppContext,
) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            ..Default::default()
        },
    );

    submit_form(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.submissions.is_empty());
    assert!(observations.last_field_state().invalid);
    assert!(observations.last_error_state().present);
    assert!(observations.last_input_state().focused);
}

#[gpui::test]
fn disabled_field_is_skipped_by_submit_validation_and_values(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            field_disabled: true,
            ..Default::default()
        },
    );

    submit_form(cx, window);
    let observations = read_observations(cx, window);

    assert_eq!(observations.submissions.len(), 1);
    assert!(observations.submissions[0].values.is_empty());
    assert!(!observations.last_field_state().invalid);
}

#[gpui::test]
fn re_enabled_required_field_registers_again_and_blocks_submit(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            field_disabled: true,
            ..Default::default()
        },
    );

    submit_form(cx, window);
    assert_eq!(read_observations(cx, window).submissions.len(), 1);

    super::support::update_config(cx, window, |config| {
        config.field_disabled = false;
    });
    submit_form(cx, window);
    let observations = read_observations(cx, window);

    assert_eq!(observations.submissions.len(), 1);
    assert!(observations.last_field_state().invalid);
}

#[gpui::test]
fn form_validation_mode_is_inherited_by_field(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            form_validation_mode: FieldValidationMode::OnChange,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    let observations = read_observations(cx, window);

    assert_eq!(observations.last_field_state().valid, Some(true));
}

#[gpui::test]
fn on_submit_fields_revalidate_on_change_after_failed_submit(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            ..Default::default()
        },
    );

    submit_form(cx, window);
    assert!(read_observations(cx, window).last_field_state().invalid);

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    let observations = read_observations(cx, window);

    assert_eq!(observations.last_field_state().valid, Some(true));
}

#[gpui::test]
fn on_blur_fields_validate_on_blur_inside_form(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            field_validation_mode: Some(FieldValidationMode::OnBlur),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    blur(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.last_field_state().invalid);
}

#[gpui::test]
fn explicit_field_validation_mode_overrides_form_validation_mode(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            required: true,
            form_validation_mode: FieldValidationMode::OnChange,
            field_validation_mode: Some(FieldValidationMode::OnSubmit),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    let observations = read_observations(cx, window);

    assert_eq!(observations.last_field_state().valid, None);
    assert!(validate_form(cx, window));
}
