use gpui::{SharedString, TestAppContext};

use crate::field::FieldValidationMode;

use super::support::{
    blur, focus_next, open_field, read_observations, update_config, validate_manually,
    FieldTestConfig, FieldTestValidation,
};

#[gpui::test]
fn on_change_validation_updates_error_and_validity_data(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            value: SharedString::from("filled"),
            validation_mode: FieldValidationMode::OnChange,
            validation: FieldTestValidation::ErrorWhenEmpty,
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.value = SharedString::default();
    });

    let observations = read_observations(cx, window);
    let error = observations.error_state().expect("error should render");
    assert_eq!(error.error.as_ref(), "Required");
    assert_eq!(
        observations.validity_state().validity.error.as_ref(),
        "Required"
    );
    assert_eq!(observations.root_state().valid, Some(false));
}

#[gpui::test]
fn on_blur_validation_runs_when_control_loses_focus(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            validation_mode: FieldValidationMode::OnBlur,
            validation: FieldTestValidation::ErrorWhenEmpty,
            ..Default::default()
        },
    );

    focus_next(cx, window);
    blur(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.root_state().touched);
    assert_eq!(observations.root_state().valid, Some(false));
    assert!(observations.error_state().is_some());
}

#[gpui::test]
fn manual_validation_validates_current_registered_control_value(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            value: SharedString::from("filled"),
            validation_mode: FieldValidationMode::OnSubmit,
            validation: FieldTestValidation::ErrorWhenEmpty,
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.value = SharedString::default();
    });
    assert!(read_observations(cx, window).error_state().is_none());

    validate_manually(cx, window);

    let observations = read_observations(cx, window);
    let error = observations.error_state().expect("error should render");
    assert_eq!(error.error.as_ref(), "Required");
    assert_eq!(observations.root_state().valid, Some(false));
}

#[gpui::test]
fn required_manual_validation_sets_value_missing(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            control_required: true,
            validation_mode: FieldValidationMode::OnSubmit,
            ..Default::default()
        },
    );

    validate_manually(cx, window);

    let validity = read_observations(cx, window).validity_state().validity;
    assert_eq!(validity.error.as_ref(), "Required");
    assert!(validity.state.value_missing);
    assert_eq!(validity.state.valid, Some(false));
}

#[gpui::test]
fn validation_multiple_errors_updates_first_error_and_full_errors(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            validation_mode: FieldValidationMode::OnChange,
            validation: FieldTestValidation::MultipleErrors,
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.value = SharedString::from("trigger");
    });

    let validity = read_observations(cx, window).validity_state().validity;
    assert_eq!(validity.error.as_ref(), "First");
    assert_eq!(validity.errors.len(), 2);
}
