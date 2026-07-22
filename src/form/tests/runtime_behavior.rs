use gpui::SharedString;

use crate::form::{FormErrors, FormFieldRegistration, FormFieldSnapshot, FormRuntime, FormValue};

#[test]
fn collects_named_enabled_values_in_registration_order_with_last_duplicate_winning() {
    let mut runtime = FormRuntime::new();

    runtime.begin_registration_pass();
    assert!(runtime.register_field(FormFieldRegistration::new(
        FormFieldSnapshot::new("first")
            .name("account")
            .value(FormValue::Text(SharedString::from("first")))
            .valid(Some(true)),
    )));
    assert!(runtime.register_field(FormFieldRegistration::new(
        FormFieldSnapshot::new("second")
            .name("account")
            .value(FormValue::Text(SharedString::from("second")))
            .valid(Some(true)),
    )));
    assert!(!runtime.finish_registration_pass());

    let result = runtime.submission_result();

    assert!(result.valid);
    assert_eq!(
        result.values.get("account"),
        Some(&FormValue::Text(SharedString::from("second")))
    );
}

#[test]
fn clearing_one_external_error_keeps_sibling_errors() {
    let mut runtime = FormRuntime::new();
    let mut errors = FormErrors::new();
    errors.insert("email".into(), vec!["Email rejected".into()]);
    errors.insert("username".into(), vec!["Username rejected".into()]);

    assert!(runtime.sync_external_errors(&errors));
    assert!(runtime.clear_external_error(&"email".into()));

    assert!(runtime
        .external_errors_for(Some(&"email".into()))
        .is_empty());
    assert_eq!(
        runtime.external_errors_for(Some(&"username".into())),
        vec![SharedString::from("Username rejected")]
    );
}

#[test]
fn disabled_fields_are_omitted_from_values_and_invalid_checks() {
    let mut runtime = FormRuntime::new();

    runtime.begin_registration_pass();
    runtime.register_field(FormFieldRegistration::new(
        FormFieldSnapshot::new("disabled")
            .name("email")
            .value(FormValue::Text(SharedString::from("bad")))
            .disabled(true)
            .valid(Some(false)),
    ));
    runtime.finish_registration_pass();

    let result = runtime.submission_result();

    assert!(result.valid);
    assert!(result.values.is_empty());
}

#[test]
fn replacing_field_registration_updates_values_without_stale_snapshots() {
    let mut runtime = FormRuntime::new();

    runtime.begin_registration_pass();
    runtime.register_field(FormFieldRegistration::new(
        FormFieldSnapshot::new("field")
            .name("email")
            .value(FormValue::Text(SharedString::from("old@example.com"))),
    ));
    runtime.finish_registration_pass();

    runtime.begin_registration_pass();
    runtime.register_field(FormFieldRegistration::new(
        FormFieldSnapshot::new("field")
            .name("email")
            .value(FormValue::Text(SharedString::from("new@example.com"))),
    ));
    runtime.finish_registration_pass();

    assert_eq!(
        runtime.submission_result().values.get("email"),
        Some(&FormValue::Text(SharedString::from("new@example.com")))
    );
}

#[test]
fn prunes_unmounted_fields() {
    let mut runtime = FormRuntime::new();

    runtime.begin_registration_pass();
    runtime.register_field(FormFieldRegistration::new(
        FormFieldSnapshot::new("field")
            .name("email")
            .value(FormValue::Text(SharedString::from("value"))),
    ));
    runtime.finish_registration_pass();
    assert_eq!(runtime.registered_field_count(), 1);

    runtime.begin_registration_pass();
    assert!(runtime.finish_registration_pass());

    assert_eq!(runtime.registered_field_count(), 0);
}
