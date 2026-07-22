use super::support::{
    focus_input, open_form, read_observations, simulate_keys, submit_form, update_config,
    FormTestConfig,
};
use crate::form::FormErrors;

fn errors_for(name: &'static str, messages: impl IntoIterator<Item = &'static str>) -> FormErrors {
    let mut errors = FormErrors::new();
    errors.insert(name.into(), messages.into_iter().map(Into::into).collect());
    errors
}

fn errors_for_email(messages: impl IntoIterator<Item = &'static str>) -> FormErrors {
    errors_for("email", messages)
}

#[gpui::test]
fn external_errors_mark_matching_field_invalid_and_populate_error_state(
    cx: &mut gpui::TestAppContext,
) {
    let window = open_form(
        cx,
        FormTestConfig {
            value: "luke@example.com".into(),
            external_errors: errors_for_email(["Server rejected email", "Try another email"]),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);

    assert!(observations.last_field_state().invalid);
    assert_eq!(
        observations.last_error_state().error,
        "Server rejected email"
    );
    assert_eq!(observations.last_error_state().errors.len(), 2);
    assert_eq!(
        observations.last_validity_state().validity.errors[1],
        "Try another email"
    );
}

#[gpui::test]
fn external_errors_do_not_mark_unmatched_fields_invalid(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            value: "luke@example.com".into(),
            external_errors: errors_for("username", ["Username is taken"]),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);

    assert!(!observations.last_field_state().invalid);
}

#[gpui::test]
fn updating_external_errors_updates_error_presence(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            value: "luke@example.com".into(),
            external_errors: errors_for_email(["Server rejected email"]),
            ..Default::default()
        },
    );
    assert!(read_observations(cx, window).last_field_state().invalid);

    update_config(cx, window, |config| {
        config.external_errors = FormErrors::new();
    });
    let observations = read_observations(cx, window);

    assert!(!observations.last_field_state().invalid);
}

#[gpui::test]
fn editing_field_clears_only_that_fields_external_error(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            value: "luke@example.com".into(),
            external_errors: errors_for_email(["Server rejected email"]),
            ..Default::default()
        },
    );
    assert!(read_observations(cx, window).last_field_state().invalid);

    focus_input(cx, window);
    simulate_keys(cx, window, "a");
    let observations = read_observations(cx, window);

    assert!(!observations.last_field_state().invalid);
    assert!(!observations
        .error_states
        .last()
        .map(|state| state.present)
        .unwrap_or(false));
}

#[gpui::test]
fn external_errors_block_submit(cx: &mut gpui::TestAppContext) {
    let window = open_form(
        cx,
        FormTestConfig {
            value: "luke@example.com".into(),
            external_errors: errors_for_email(["Server rejected email"]),
            ..Default::default()
        },
    );

    submit_form(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.submissions.is_empty());
    assert!(observations.last_input_state().focused);
}
