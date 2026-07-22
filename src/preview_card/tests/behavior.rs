use std::time::Duration;

use gpui::{point, px, TestAppContext};

use crate::preview_card::tests::support::{
    advance_clock, blur, click_selector, debug_bounds, focus_next, hover_trigger, move_mouse_to,
    move_over_selector, open_preview_card, read_observations, PreviewCardTestConfig,
};
use crate::preview_card::{create_preview_card_handle, PreviewCardOpenChangeReason};

#[gpui::test]
fn starts_closed_by_default(cx: &mut TestAppContext) {
    let window = open_preview_card(cx, PreviewCardTestConfig::default());
    let observations = read_observations(cx, window);

    assert!(observations.popup_state().is_none());
    assert!(observations.open_changes.is_empty());
}

#[gpui::test]
fn hover_opens_and_unhover_closes(cx: &mut TestAppContext) {
    let window = open_preview_card(cx, PreviewCardTestConfig::default());
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
    assert_eq!(
        observations.open_changes.first().copied(),
        Some((true, PreviewCardOpenChangeReason::TriggerHover))
    );
    assert_eq!(
        observations.trigger_state().map(|state| state.open),
        Some(true)
    );

    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    advance_clock(cx, Duration::from_millis(50));
    let observations = read_observations(cx, window);
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));
}

#[gpui::test]
fn hover_open_waits_for_open_delay(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            delay: Some(Duration::from_millis(600)),
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));

    advance_clock(cx, Duration::from_millis(650));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
}

#[gpui::test]
fn unhover_close_waits_for_close_delay_and_rehover_cancels(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            close_delay: Some(Duration::from_millis(300)),
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );

    // Leave the trigger; before the close delay elapses the card is still open.
    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    advance_clock(cx, Duration::from_millis(100));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );

    // Re-hovering cancels the pending close.
    hover_trigger(cx, window);
    advance_clock(cx, Duration::from_millis(400));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );

    // Leaving again closes after the delay.
    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    advance_clock(cx, Duration::from_millis(400));
    let observations = read_observations(cx, window);
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));
}

#[gpui::test]
fn focus_opens_and_blur_closes(cx: &mut TestAppContext) {
    let window = open_preview_card(cx, PreviewCardTestConfig::default());
    focus_next(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
    assert!(observations
        .open_changes
        .iter()
        .any(|(open, reason)| *open && *reason == PreviewCardOpenChangeReason::TriggerFocus));

    blur(cx, window);
    let observations = read_observations(cx, window);
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));
}

#[gpui::test]
fn escape_closes_and_rehover_reopens(cx: &mut TestAppContext) {
    let window = open_preview_card(cx, PreviewCardTestConfig::default());
    // Focus the trigger so key dispatch reaches the PreviewCard key context.
    focus_next(cx, window);
    hover_trigger(cx, window);
    simulate_escape(cx, window);
    let observations = read_observations(cx, window);
    assert!(observations
        .open_changes
        .iter()
        .any(|(open, reason)| !*open && *reason == PreviewCardOpenChangeReason::EscapeKey));
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));

    // Hovering again reopens (move away first so hover state changes).
    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
}

fn simulate_escape(
    cx: &mut TestAppContext,
    window: gpui::WindowHandle<crate::preview_card::tests::support::PreviewCardTestView>,
) {
    crate::preview_card::tests::support::simulate_keys(cx, window, "escape");
}

#[gpui::test]
fn outside_press_closes(cx: &mut TestAppContext) {
    let window = open_preview_card(cx, PreviewCardTestConfig::default());
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );

    click_selector(cx, window, "preview-card-outside-target");
    let observations = read_observations(cx, window);
    assert!(observations
        .open_changes
        .iter()
        .any(|(open, reason)| !*open && *reason == PreviewCardOpenChangeReason::OutsidePress));
}

#[gpui::test]
fn hovering_popup_keeps_card_open(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            close_delay: Some(Duration::from_millis(100)),
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );

    // Travel from trigger into the popup.
    move_over_selector(cx, window, "preview-card-popup");
    advance_clock(cx, Duration::from_millis(400));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );

    // Leaving the popup closes after the close delay.
    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    advance_clock(cx, Duration::from_millis(400));
    let observations = read_observations(cx, window);
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));
}

#[gpui::test]
fn multiple_triggers_switch_active_payload(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            second_trigger: true,
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert_eq!(observations.rendered_payload(), Some(Some("primary")));

    move_over_selector(cx, window, "preview-card-trigger-secondary");
    advance_clock(cx, Duration::from_millis(50));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
    assert_eq!(observations.rendered_payload(), Some(Some("secondary")));
}

#[gpui::test]
fn controlled_root_reports_without_mutating(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            controlled_open: Some(false),
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert!(observations.open_changes.iter().any(|(open, _)| *open));
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));
}

#[gpui::test]
fn canceled_open_stays_closed(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            cancel_open: true,
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert!(!observations.open_changes.is_empty());
    assert!(observations
        .popup_state()
        .map(|state| !state.open)
        .unwrap_or(true));
    assert!(observations.open_change_completes.is_empty());
}

#[gpui::test]
fn canceled_close_stays_open(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            cancel_close: true,
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    advance_clock(cx, Duration::from_millis(100));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
}

#[gpui::test]
fn prevent_unmount_on_close_keeps_popup_mounted(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            prevent_unmount_on_close: true,
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    move_mouse_to(cx, window, point(px(600.0), px(340.0)));
    advance_clock(cx, Duration::from_millis(100));
    let observations = read_observations(cx, window);
    let popup = observations
        .popup_state()
        .expect("popup should stay mounted");
    assert!(!popup.open);
    assert!(popup.mounted);
}

#[gpui::test]
fn handle_open_close_and_is_open(cx: &mut TestAppContext) {
    let handle = create_preview_card_handle::<&'static str>();
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            handle: Some(handle.clone()),
            ..PreviewCardTestConfig::default()
        },
    );
    read_observations(cx, window);

    let opened = window
        .update(cx, |_view, window, cx| {
            handle.open("preview-card-trigger", window, cx)
        })
        .expect("window should be open");
    assert!(opened);
    cx.run_until_parked();
    assert!(cx.update(|cx| handle.is_open(cx)));
    let observations = read_observations(cx, window);
    assert_eq!(
        observations.popup_state().map(|state| state.open),
        Some(true)
    );
    assert!(observations
        .open_changes
        .iter()
        .any(|(open, reason)| *open && *reason == PreviewCardOpenChangeReason::ImperativeAction));

    let missing = window
        .update(cx, |_view, window, cx| handle.open("missing", window, cx))
        .expect("window should be open");
    assert!(!missing);

    let closed = window
        .update(cx, |_view, window, cx| handle.close(window, cx))
        .expect("window should be open");
    assert!(closed);
    cx.run_until_parked();
    assert!(!cx.update(|cx| handle.is_open(cx)));
}

#[gpui::test]
fn unbound_handle_is_recoverable(cx: &mut TestAppContext) {
    let handle = create_preview_card_handle::<&'static str>();
    let window = open_preview_card(cx, PreviewCardTestConfig::default());
    read_observations(cx, window);
    assert!(!cx.update(|cx| handle.is_open(cx)));
}

#[gpui::test]
fn keep_mounted_portal_stays_hidden_when_closed(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            keep_mounted: true,
            ..PreviewCardTestConfig::default()
        },
    );
    let observations = read_observations(cx, window);
    let portal = observations
        .portal_states
        .last()
        .copied()
        .expect("portal should render when keep-mounted");
    assert!(portal.mounted);
    assert!(!portal.open);
    let popup = observations.popup_state().expect("popup should be mounted");
    assert!(!popup.open);
    assert!(popup.mounted);
}

#[gpui::test]
fn backdrop_does_not_capture_outside_clicks(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            backdrop: true,
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);
    assert!(!observations.backdrop_states.is_empty());
    assert!(debug_bounds(cx, window, "preview-card-backdrop").is_some());

    // A click on content covered by the backdrop reaches that content and
    // dismissal is attributed to the outside-press path.
    click_selector(cx, window, "preview-card-outside-target");
    let observations = read_observations(cx, window);
    assert_eq!(observations.outside_clicks, 1);
    assert!(observations
        .open_changes
        .iter()
        .any(|(open, reason)| !*open && *reason == PreviewCardOpenChangeReason::OutsidePress));
}

#[gpui::test]
fn style_states_expose_part_facts(cx: &mut TestAppContext) {
    let window = open_preview_card(
        cx,
        PreviewCardTestConfig {
            backdrop: true,
            ..PreviewCardTestConfig::default()
        },
    );
    hover_trigger(cx, window);
    let observations = read_observations(cx, window);

    let trigger = observations.trigger_state().expect("trigger state");
    assert!(trigger.open);
    assert!(trigger.active_trigger);
    assert!(trigger.payload_present);

    let positioner = observations
        .positioner_states
        .last()
        .cloned()
        .expect("positioner state");
    assert!(positioner.open);
    assert!(positioner.anchor_available);

    let arrow = observations
        .arrow_states
        .last()
        .copied()
        .expect("arrow state");
    assert!(arrow.open);

    let backdrop = observations
        .backdrop_states
        .last()
        .copied()
        .expect("backdrop state");
    assert!(backdrop.open);

    let viewport = observations
        .viewport_states
        .last()
        .cloned()
        .expect("viewport state");
    assert!(viewport.current_trigger_id.is_some());
}
