use gpui::TestAppContext;

use crate::otp_field::OTPFieldChangeReason;

use super::support::{
    focus_otp_field, open_otp_field, read_observations, shared, simulate_keys, OTPFieldTestConfig,
};

#[gpui::test]
fn left_and_right_navigate_with_end_of_value_clamping(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    simulate_keys(cx, window, "left left left");
    assert_eq!(read_observations(cx, window).active_index(), Some(0));

    simulate_keys(cx, window, "right right right right");
    // Never beyond min(value length, length - 1).
    assert_eq!(read_observations(cx, window).active_index(), Some(2));
}

#[gpui::test]
fn home_end_and_arrow_up_down_move_to_boundaries(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "123",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    simulate_keys(cx, window, "home");
    assert_eq!(read_observations(cx, window).active_index(), Some(0));

    simulate_keys(cx, window, "end");
    assert_eq!(read_observations(cx, window).active_index(), Some(3));

    simulate_keys(cx, window, "up");
    assert_eq!(read_observations(cx, window).active_index(), Some(0));

    simulate_keys(cx, window, "down");
    assert_eq!(read_observations(cx, window).active_index(), Some(3));

    #[cfg(target_os = "macos")]
    {
        simulate_keys(cx, window, "cmd-left");
        assert_eq!(read_observations(cx, window).active_index(), Some(0));
        simulate_keys(cx, window, "cmd-right");
        assert_eq!(read_observations(cx, window).active_index(), Some(3));
    }
    #[cfg(not(target_os = "macos"))]
    {
        simulate_keys(cx, window, "ctrl-left");
        assert_eq!(read_observations(cx, window).active_index(), Some(0));
        simulate_keys(cx, window, "ctrl-right");
        assert_eq!(read_observations(cx, window).active_index(), Some(3));
    }
}

#[gpui::test]
fn backspace_removes_filled_slot_and_moves_back(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "123",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    simulate_keys(cx, window, "home right backspace");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("13"));
    assert_eq!(observations.active_index(), Some(0));
    assert_eq!(
        observations.change_reasons,
        vec![OTPFieldChangeReason::Keyboard]
    );
}

#[gpui::test]
fn backspace_on_empty_slot_removes_previous_character(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    simulate_keys(cx, window, "end backspace");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("1"));
    assert_eq!(observations.active_index(), Some(1));
}

#[gpui::test]
fn ctrl_backspace_clears_the_entire_value(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "123",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-backspace");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-backspace");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared(""));
    assert_eq!(observations.active_index(), Some(0));
    assert_eq!(
        observations.change_reasons,
        vec![OTPFieldChangeReason::Keyboard]
    );
}

#[gpui::test]
fn delete_removes_at_active_index_and_stays(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "123",
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    simulate_keys(cx, window, "home delete");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("23"));
    assert_eq!(observations.active_index(), Some(0));
}

#[gpui::test]
fn read_only_allows_navigation_but_ignores_editing_keys(cx: &mut TestAppContext) {
    let window = open_otp_field(
        cx,
        OTPFieldTestConfig {
            default_value: "12",
            read_only: true,
            ..Default::default()
        },
    );
    focus_otp_field(cx, window);

    simulate_keys(cx, window, "home");
    assert_eq!(read_observations(cx, window).active_index(), Some(0));

    simulate_keys(cx, window, "backspace delete");
    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().value, shared("12"));
    assert!(observations.value_changes.is_empty());
}
