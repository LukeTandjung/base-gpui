use gpui::TestAppContext;

use crate::otp_field::OTPFieldChangeReason;

use super::support::{
    focus_otp_field, open_otp_field, read_observations, shared, simulate_keys, write_clipboard,
    OTPFieldTestConfig,
};

fn paste(cx: &mut TestAppContext, window: gpui::WindowHandle<super::support::OTPFieldTestView>) {
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-v");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-v");
}

#[gpui::test]
fn paste_distributes_from_active_slot_and_clamps(cx: &mut TestAppContext) {
    let window = open_otp_field(cx, OTPFieldTestConfig::default());
    focus_otp_field(cx, window);
    write_clipboard(cx, "12345");

    paste(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("1234"));
    assert_eq!(
        observations.change_reasons,
        vec![OTPFieldChangeReason::InputPaste]
    );
    assert_eq!(observations.invalid_values, vec![shared("12345")]);
    assert_eq!(observations.completed_values, vec![shared("1234")]);
    assert_eq!(
        observations.complete_reasons,
        vec![OTPFieldChangeReason::InputPaste]
    );
    assert_eq!(observations.active_index(), Some(3));
}

#[gpui::test]
fn paste_reports_rejected_characters_with_paste_reason(cx: &mut TestAppContext) {
    let window = open_otp_field(cx, OTPFieldTestConfig::default());
    focus_otp_field(cx, window);
    write_clipboard(cx, "1x2");

    paste(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("12"));
    assert_eq!(observations.invalid_values, vec![shared("1x2")]);
    assert_eq!(
        observations.invalid_reasons,
        vec![OTPFieldChangeReason::InputPaste]
    );
}

#[gpui::test]
fn all_rejected_paste_is_a_no_op_besides_invalid(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "1",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);
    write_clipboard(cx, "abc");

    paste(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("1"));
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.invalid_values, vec![shared("abc")]);
}

#[gpui::test]
fn complete_paste_over_identical_complete_value_refires_complete(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "1234",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);
    simulate_keys(cx, window, "home");
    write_clipboard(cx, "1234");

    paste(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.completed_values, vec![shared("1234")]);
}

#[gpui::test]
fn auto_submit_submits_the_surrounding_form_on_completion(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            auto_submit: true,
            field: true,
            form: true,
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);
    write_clipboard(cx, "1234");

    paste(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.completed_values, vec![shared("1234")]);
    assert_eq!(observations.submitted.len(), 1);
}

#[gpui::test]
fn auto_submit_outside_a_form_is_a_no_op(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            auto_submit: true,
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);
    write_clipboard(cx, "1234");

    paste(cx, window);

    let observations = read_observations(cx, window);
    assert_eq!(observations.completed_values, vec![shared("1234")]);
    assert!(observations.submitted.is_empty());
}
