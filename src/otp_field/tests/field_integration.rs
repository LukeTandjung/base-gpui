use gpui::{Modifiers, TestAppContext, VisualTestContext};

use crate::field::FieldValidationMode;

use super::support::{
    blur_otp_field, debug_bounds, focus_otp_field, open_otp_field, read_observations, shared,
    simulate_keys, simulate_text, OTPFieldTestConfig,
};

#[gpui::test]
fn otp_field_consumes_field_root_and_item_disabled_state(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            field: true,
            field_root_disabled: true,
            ..Default::default()
        },
    );
    assert!(read_observations(cx, window).last_root_state().disabled);

    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            field: true,
            field_item_disabled: true,
            ..Default::default()
        },
    );
    assert!(read_observations(cx, window).last_root_state().disabled);
}

#[gpui::test]
fn field_label_click_focuses_the_otp_group(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            field: true,
            ..Default::default()
        },
    );
    let label_bounds = debug_bounds(cx, window, "otp-label").expect("label should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(label_bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();

    assert!(read_observations(cx, window).last_root_state().focused);
}

#[gpui::test]
fn field_tracks_filled_dirty_focused_and_touched(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            field: true,
            ..Default::default()
        },
    );

    focus_otp_field(cx, window);
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

    blur_otp_field(cx, window);
    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(field_state.touched);

    focus_otp_field(cx, window);
    simulate_keys(cx, window, "backspace");
    let observations = read_observations(cx, window);
    let field_state = observations
        .field_validity_states
        .last()
        .expect("validity should be observed")
        .root;
    assert!(!field_state.filled);
    assert_eq!(observations.last_root_state().value, shared(""));
}

#[gpui::test]
fn required_field_validation_reports_missing_value_on_blur(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            field: true,
            required: true,
            field_validation_mode: FieldValidationMode::OnBlur,
            ..Default::default()
        },
    );

    focus_otp_field(cx, window);
    blur_otp_field(cx, window);

    let observations = read_observations(cx, window);
    let validity = observations
        .field_validity_states
        .last()
        .expect("validity should be observed");
    assert!(validity.validity.state.value_missing);
}

#[gpui::test]
fn on_change_field_validation_tracks_cleared_value(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            field: true,
            default_value: "1",
            required: true,
            field_validation_mode: FieldValidationMode::OnChange,
            ..Default::default()
        },
    );

    focus_otp_field(cx, window);
    simulate_keys(cx, window, "backspace");

    let observations = read_observations(cx, window);
    let validity = observations
        .field_validity_states
        .last()
        .expect("validity should be observed");
    assert!(validity.validity.state.value_missing);
}
