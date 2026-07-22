use gpui::TestAppContext;

use crate::otp_field::OTPFieldChangeReason;

use super::support::{
    click_selector, focus_otp_field, open_otp_field, read_observations, shared, simulate_text,
    update_config, OTPFieldTestConfig,
};

#[gpui::test]
fn default_value_renders_across_slots(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("12"));
    assert_eq!(observations.slot_state(0).value, shared("1"));
    assert_eq!(observations.slot_state(1).value, shared("2"));
    assert!(!observations.slot_state(2).filled);
}

#[gpui::test]
fn controlled_value_updates_slots_across_renders(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            controlled_value: Some("12"),
            ..Default::default()
        },
    );
    assert_eq!(
        read_observations(cx, window).last_root_state().value,
        shared("12")
    );

    update_config(cx, window, |config| {
        config.controlled_value = Some("34 5x6");
    });
    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("3456"));
    // Unfocused styling-active slot is min(value length, length - 1).
    assert_eq!(observations.active_index(), Some(3));
}

#[gpui::test]
fn typing_distributes_forward_and_advances_active_slot(cx: &mut TestAppContext) {
    let window = open_otp_field(cx, OTPFieldTestConfig::default());

    focus_otp_field(cx, window);
    simulate_text(cx, window, "12");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("12"));
    assert_eq!(observations.active_index(), Some(2));
    assert_eq!(observations.value_changes, vec![shared("1"), shared("12")]);
    assert_eq!(
        observations.change_reasons,
        vec![
            OTPFieldChangeReason::InputChange,
            OTPFieldChangeReason::InputChange
        ]
    );
}

#[gpui::test]
fn same_character_overtype_advances_without_value_change(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            ..Default::default()
        },
    );

    click_selector(cx, window, "otp-slot-0");
    simulate_text(cx, window, "1");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("12"));
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.active_index(), Some(1));
}

#[gpui::test]
fn fully_rejected_typing_fires_invalid_and_keeps_state(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "1",
            ..Default::default()
        },
    );

    focus_otp_field(cx, window);
    simulate_text(cx, window, "x");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("1"));
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.invalid_values, vec![shared("x")]);
    assert_eq!(
        observations.invalid_reasons,
        vec![OTPFieldChangeReason::InputChange]
    );
}

#[gpui::test]
fn completion_fires_when_typing_fills_the_last_slot(cx: &mut TestAppContext) {
    let window = open_otp_field(cx, OTPFieldTestConfig::default());

    focus_otp_field(cx, window);
    simulate_text(cx, window, "1234");

    let observations = read_observations(cx, window);
    assert_eq!(observations.completed_values, vec![shared("1234")]);
    assert_eq!(
        observations.complete_reasons,
        vec![OTPFieldChangeReason::InputChange]
    );
    assert!(observations.last_root_state().complete);
}

#[gpui::test]
fn mask_renders_mask_character_but_keeps_real_value(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            mask: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.slot_state(0).masked);
    assert_eq!(observations.slot_state(0).value, shared("1"));
    assert_eq!(observations.last_root_state().value, shared("12"));
}

#[gpui::test]
fn disabled_field_ignores_typing_and_pointer_activation(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "1",
            disabled: true,
            ..Default::default()
        },
    );

    click_selector(cx, window, "otp-slot-0");
    simulate_text(cx, window, "2");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("1"));
    assert!(observations.value_changes.is_empty());
    assert!(!observations.last_root_state().focused);
}

#[gpui::test]
fn read_only_field_allows_focus_but_ignores_edits(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            read_only: true,
            ..Default::default()
        },
    );

    focus_otp_field(cx, window);
    simulate_text(cx, window, "3");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().focused);
    assert_eq!(observations.last_root_state().value, shared("12"));
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn pointer_activation_clamps_to_end_of_value(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "1",
            ..Default::default()
        },
    );

    click_selector(cx, window, "otp-slot-3");

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().focused);
    assert_eq!(observations.active_index(), Some(1));
}
