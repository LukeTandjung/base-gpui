use gpui::TestAppContext;

use crate::field::FieldValidityKey;

use super::support::{open_field, read_observations, FieldTestConfig};

#[gpui::test]
fn field_error_is_absent_by_default(cx: &mut TestAppContext) {
    let window = open_field(cx, FieldTestConfig::default());

    assert!(read_observations(cx, window).error_state().is_none());
}

#[gpui::test]
fn field_error_renders_when_invalid_by_default(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            root_invalid: Some(true),
            ..Default::default()
        },
    );

    let error = read_observations(cx, window)
        .error_state()
        .expect("default error should render");
    assert!(error.present);
    assert!(error.root.invalid);
}

#[gpui::test]
fn field_error_specific_match_renders_only_for_matching_flag(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            root_invalid: Some(true),
            error_match: Some(FieldValidityKey::CustomError),
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).error_state().is_none());
}

#[gpui::test]
fn field_error_match_always_renders_regardless_of_validity(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            error_always: true,
            ..Default::default()
        },
    );

    assert!(
        read_observations(cx, window)
            .error_state()
            .expect("always error should render")
            .present
    );
}
