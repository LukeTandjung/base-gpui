use std::time::Duration;

use gpui::TestAppContext;

use crate::popover::{
    create_popover_handle,
    tests::support::{
        advance_clock, blur, click_close, click_outside_target, click_trigger, debug_bounds,
        focus_next, move_over_selector, open_popover, read_observations, simulate_keys,
        PopoverTestConfig,
    },
    PopoverOpenChangeReason, PopoverSide,
};

#[gpui::test]
fn trigger_click_opens_and_second_click_closes(cx: &mut TestAppContext) {
    let window = open_popover(cx, PopoverTestConfig::default());

    assert!(!read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    assert!(read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, PopoverOpenChangeReason::TriggerPress),
            (false, PopoverOpenChangeReason::TriggerPress),
        ]
    );
}

#[gpui::test]
fn escape_closes_when_trigger_has_focus(cx: &mut TestAppContext) {
    let window = open_popover(cx, PopoverTestConfig::default());

    click_trigger(cx, window);
    simulate_keys(cx, window, "escape");

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes.last(),
        Some(&(false, PopoverOpenChangeReason::EscapeKey))
    );
}

#[gpui::test]
fn close_part_closes(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            default_open: true,
            ..PopoverTestConfig::default()
        },
    );

    click_close(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, PopoverOpenChangeReason::ClosePress)]
    );
    assert!(observations.trigger_state().unwrap().focused);
}

#[gpui::test]
fn keyboard_activation_on_close_part_closes(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            default_open: true,
            ..PopoverTestConfig::default()
        },
    );

    focus_next(cx, window);
    focus_next(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, PopoverOpenChangeReason::ClosePress)]
    );
}

#[gpui::test]
fn pointer_open_moves_focus_into_popup_for_keyboard_activation(cx: &mut TestAppContext) {
    let window = open_popover(cx, PopoverTestConfig::default());

    click_trigger(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, PopoverOpenChangeReason::TriggerPress),
            (false, PopoverOpenChangeReason::ClosePress),
        ]
    );
}

#[gpui::test]
fn focus_out_closes_non_modal_popover(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            default_open: true,
            ..PopoverTestConfig::default()
        },
    );

    focus_next(cx, window);
    focus_next(cx, window);
    assert!(read_observations(cx, window).root_state().unwrap().open);

    blur(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, PopoverOpenChangeReason::FocusOut)]
    );
}

#[gpui::test]
fn outside_click_dismisses(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            default_open: true,
            ..PopoverTestConfig::default()
        },
    );

    click_outside_target(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, PopoverOpenChangeReason::OutsidePress)]
    );
}

#[gpui::test]
fn controlled_root_reports_open_change_without_internal_mutation(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            controlled_open: Some(false),
            ..PopoverTestConfig::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, PopoverOpenChangeReason::TriggerPress)]
    );
}

#[gpui::test]
fn detached_handle_opens_and_closes(cx: &mut TestAppContext) {
    let handle = create_popover_handle::<()>();
    let window = open_popover(
        cx,
        PopoverTestConfig {
            handle: Some(handle.clone()),
            ..PopoverTestConfig::default()
        },
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("popover-trigger", window, cx));
        })
        .expect("popover test window should be open");
    cx.run_until_parked();
    assert!(read_observations(cx, window).root_state().unwrap().open);

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.close(window, cx));
        })
        .expect("popover test window should be open");
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, PopoverOpenChangeReason::ImperativeAction),
            (false, PopoverOpenChangeReason::ImperativeAction),
        ]
    );
}

#[gpui::test]
fn hover_open_and_close_respect_configured_delays(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            open_on_hover: true,
            delay: Duration::from_millis(50),
            close_delay: Duration::from_millis(75),
            ..PopoverTestConfig::default()
        },
    );

    move_over_selector(cx, window, "popover-trigger");
    advance_clock(cx, Duration::from_millis(49));
    assert!(!read_observations(cx, window).root_state().unwrap().open);

    advance_clock(cx, Duration::from_millis(1));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "popover-outside-target");
    advance_clock(cx, Duration::from_millis(74));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    advance_clock(cx, Duration::from_millis(1));
    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, PopoverOpenChangeReason::TriggerHover),
            (false, PopoverOpenChangeReason::TriggerHover),
        ]
    );
}

#[gpui::test]
fn hover_open_makes_user_backdrop_non_interactive(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            open_on_hover: true,
            ..PopoverTestConfig::default()
        },
    );

    move_over_selector(cx, window, "popover-trigger");
    let observations = read_observations(cx, window);
    assert!(observations.root_state().unwrap().open);
    assert!(!observations.backdrop_states.last().unwrap().interactive);

    assert!(debug_bounds(cx, window, "popover-backdrop").is_none());
    assert_eq!(
        observations.open_changes,
        vec![(true, PopoverOpenChangeReason::TriggerHover)]
    );
}

#[gpui::test]
fn rehover_cancels_delayed_hover_close(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            open_on_hover: true,
            close_delay: Duration::from_millis(100),
            ..PopoverTestConfig::default()
        },
    );

    move_over_selector(cx, window, "popover-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "popover-outside-target");
    advance_clock(cx, Duration::from_millis(50));
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "popover-trigger");
    advance_clock(cx, Duration::from_millis(100));
    assert!(read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn hovering_popup_keeps_hover_opened_popover_open(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            open_on_hover: true,
            close_delay: Duration::from_millis(75),
            ..PopoverTestConfig::default()
        },
    );

    move_over_selector(cx, window, "popover-trigger");
    assert!(read_observations(cx, window).root_state().unwrap().open);

    // Move straight onto the popup: leaving the trigger schedules a delayed
    // close, but landing on the popup must keep the popover open well past
    // the close delay.
    move_over_selector(cx, window, "popover-popup");
    advance_clock(cx, Duration::from_millis(500));
    assert!(
        read_observations(cx, window).root_state().unwrap().open,
        "hovering the popup should keep a hover-opened popover open"
    );

    // Leaving the popup closes it after the close delay.
    move_over_selector(cx, window, "popover-outside-target");
    advance_clock(cx, Duration::from_millis(500));
    assert!(
        !read_observations(cx, window).root_state().unwrap().open,
        "leaving the popup should close a hover-opened popover"
    );
}

#[gpui::test]
fn leaving_popup_does_not_close_click_opened_popover(cx: &mut TestAppContext) {
    let window = open_popover(cx, PopoverTestConfig::default());

    click_trigger(cx, window);
    assert!(read_observations(cx, window).root_state().unwrap().open);

    move_over_selector(cx, window, "popover-popup");
    move_over_selector(cx, window, "popover-outside-target");
    advance_clock(cx, Duration::from_millis(500));
    assert!(
        read_observations(cx, window).root_state().unwrap().open,
        "a click-opened popover must stay open when the pointer leaves the popup"
    );
}

#[gpui::test]
fn keep_mounted_portal_reports_closed_mounted_popup(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            keep_mounted: true,
            ..PopoverTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    let popup_state = observations.popup_state().unwrap();
    assert!(!popup_state.open);
    assert!(popup_state.mounted);
    assert!(debug_bounds(cx, window, "popover-popup").is_some());
}

#[gpui::test]
fn modal_backdrop_occludes_outside_click_and_closes(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            default_open: true,
            modal: true,
            ..PopoverTestConfig::default()
        },
    );

    click_outside_target(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(observations.outside_clicks, 0);
    assert_eq!(
        observations.open_changes,
        vec![(false, PopoverOpenChangeReason::OutsidePress)]
    );
}

#[gpui::test]
fn rendered_viewport_reports_activation_direction(cx: &mut TestAppContext) {
    let handle = create_popover_handle::<()>();
    let window = open_popover(
        cx,
        PopoverTestConfig {
            handle: Some(handle.clone()),
            second_trigger: true,
            ..PopoverTestConfig::default()
        },
    );

    click_trigger(cx, window);
    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("popover-trigger-secondary", window, cx));
        })
        .expect("popover test window should be open");
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    assert_eq!(
        observations
            .viewport_states
            .last()
            .unwrap()
            .activation_direction,
        crate::popover::PopoverActivationDirection::Forward
    );
}

#[gpui::test]
fn rendered_title_description_and_arrow_states_are_reported(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            default_open: true,
            positioner_side: PopoverSide::Top,
            ..PopoverTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(!observations.title_states.is_empty());
    assert!(!observations.description_states.is_empty());
    assert_eq!(
        observations
            .positioner_states
            .last()
            .unwrap()
            .transform_origin_y_percent,
        100.0
    );
    assert_eq!(
        observations.arrow_states.last().unwrap().side,
        PopoverSide::Top
    );
    assert!(observations.arrow_states.last().unwrap().open);
}

#[gpui::test]
fn disabled_trigger_does_not_open(cx: &mut TestAppContext) {
    let window = open_popover(
        cx,
        PopoverTestConfig {
            trigger_disabled: true,
            ..PopoverTestConfig::default()
        },
    );

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert!(observations.open_changes.is_empty());
    assert!(observations.trigger_state().unwrap().disabled);
}
