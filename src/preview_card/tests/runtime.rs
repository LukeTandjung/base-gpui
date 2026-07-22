use std::time::Duration;

use gpui::{point, px, size, Bounds, ElementId};

use crate::preview_card::{
    PreviewCardActivationDirection, PreviewCardAlign, PreviewCardHoverTarget, PreviewCardInstant,
    PreviewCardOpenChangeReason, PreviewCardOpenChangeSource, PreviewCardRuntime, PreviewCardSide,
    PreviewCardTriggerMetadata,
};
use crate::primitives::safe_polygon::SafePolygonVerdict;

fn trigger_metadata(
    id: &'static str,
    delay: Duration,
    close_delay: Duration,
    payload: Option<&'static str>,
    order: usize,
) -> PreviewCardTriggerMetadata<&'static str> {
    PreviewCardTriggerMetadata::new_without_focus(
        ElementId::from(id),
        ElementId::from(id),
        delay,
        close_delay,
        payload,
        order,
        false,
    )
}

fn runtime_with_triggers(open: bool) -> PreviewCardRuntime<&'static str> {
    let mut runtime = PreviewCardRuntime::new(open, None);
    runtime.sync_children(vec![
        trigger_metadata(
            "one",
            Duration::from_millis(600),
            Duration::from_millis(300),
            Some("payload-one"),
            0,
        ),
        trigger_metadata(
            "two",
            Duration::from_millis(50),
            Duration::from_millis(25),
            Some("payload-two"),
            1,
        ),
    ]);
    runtime
}

#[test]
fn default_closed_runtime_reports_closed_and_unmounted() {
    let runtime: PreviewCardRuntime<()> = PreviewCardRuntime::new(false, None);
    assert!(!runtime.open_value());
    assert!(!runtime.mounted_value(false));
    assert!(runtime.mounted_value(true));
}

#[test]
fn default_open_resolves_first_trigger_and_payload() {
    let runtime = runtime_with_triggers(true);
    assert!(runtime.open_value());
    assert_eq!(runtime.active_trigger_id(), Some(ElementId::from("one")));
    assert_eq!(runtime.active_payload(), Some("payload-one"));
}

#[test]
fn default_open_with_default_trigger_id_resolves_that_payload() {
    let mut runtime = PreviewCardRuntime::new(true, Some(ElementId::from("two")));
    runtime.sync_children(vec![
        trigger_metadata("one", Duration::ZERO, Duration::ZERO, Some("a"), 0),
        trigger_metadata("two", Duration::ZERO, Duration::ZERO, Some("b"), 1),
    ]);
    assert_eq!(runtime.active_payload(), Some("b"));
}

#[test]
fn open_with_no_active_trigger_resolves_no_payload() {
    let runtime: PreviewCardRuntime<&'static str> = PreviewCardRuntime::new(true, None);
    assert!(runtime.active_payload().is_none());
}

#[test]
fn request_open_for_unknown_explicit_trigger_does_not_change() {
    let runtime = runtime_with_triggers(false);
    let outcome = runtime.request_open(false, Some(ElementId::from("missing")));
    assert!(!outcome.changed());
    assert!(!outcome.open());
}

#[test]
fn hover_open_and_unhover_close_outcomes() {
    let mut runtime = runtime_with_triggers(false);
    let outcome = runtime.hover_trigger(false, ElementId::from("one"));
    assert!(outcome.changed());
    assert!(outcome.open());
    runtime.commit_open(
        true,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert!(runtime.open_value());

    let close = runtime.unhover_trigger(true);
    assert!(close.changed());
    assert!(!close.open());
}

#[test]
fn trigger_switch_while_open_changes_without_close() {
    let mut runtime = runtime_with_triggers(true);
    let outcome = runtime.hover_trigger(true, ElementId::from("two"));
    assert!(outcome.changed());
    assert!(outcome.open());
    runtime.commit_open(
        true,
        Some(ElementId::from("two")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert_eq!(runtime.active_payload(), Some("payload-two"));
}

#[test]
fn per_trigger_delay_overrides_are_stored_in_metadata() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        true,
        Some(ElementId::from("two")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert_eq!(runtime.active_close_delay(), Duration::from_millis(25));
}

#[test]
fn schedule_hover_generation_checks_reject_stale_tasks() {
    let mut runtime = runtime_with_triggers(false);
    let stale = runtime.schedule_hover(PreviewCardHoverTarget::Open, Some(ElementId::from("one")));
    let current =
        runtime.schedule_hover(PreviewCardHoverTarget::Open, Some(ElementId::from("one")));
    assert!(!runtime.take_scheduled_hover(
        stale,
        PreviewCardHoverTarget::Open,
        Some(&ElementId::from("one"))
    ));
    assert!(runtime.take_scheduled_hover(
        current,
        PreviewCardHoverTarget::Open,
        Some(&ElementId::from("one"))
    ));
}

#[test]
fn cancel_hover_clears_pending_close() {
    let mut runtime = runtime_with_triggers(true);
    let generation = runtime.schedule_hover(PreviewCardHoverTarget::Close, None);
    runtime.cancel_hover();
    assert!(!runtime.take_scheduled_hover(generation, PreviewCardHoverTarget::Close, None));
}

#[test]
fn instant_transitions_focus_dismiss_and_hover_clear() {
    let mut runtime = runtime_with_triggers(false);
    runtime.record_open_change(
        PreviewCardOpenChangeReason::TriggerFocus,
        PreviewCardOpenChangeSource::Focus,
    );
    assert_eq!(runtime.instant(), PreviewCardInstant::Focus);

    runtime.record_open_change(
        PreviewCardOpenChangeReason::TriggerPress,
        PreviewCardOpenChangeSource::Pointer,
    );
    assert_eq!(runtime.instant(), PreviewCardInstant::Dismiss);

    runtime.record_open_change(
        PreviewCardOpenChangeReason::EscapeKey,
        PreviewCardOpenChangeSource::Keyboard,
    );
    assert_eq!(runtime.instant(), PreviewCardInstant::Dismiss);

    runtime.record_open_change(
        PreviewCardOpenChangeReason::TriggerHover,
        PreviewCardOpenChangeSource::Pointer,
    );
    assert_eq!(runtime.instant(), PreviewCardInstant::None);
}

#[test]
fn press_on_open_active_trigger_requests_close_and_suppresses_focus_open() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        true,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert!(runtime.sync_trigger_press(ElementId::from("one")));
    assert!(runtime.is_trigger_press_suppressed(&ElementId::from("one")));
}

#[test]
fn press_on_closed_trigger_does_not_close() {
    let mut runtime = runtime_with_triggers(false);
    assert!(!runtime.sync_trigger_press(ElementId::from("one")));
}

#[test]
fn active_trigger_unmount_requests_close() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        true,
        Some(ElementId::from("two")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    runtime.sync_children(vec![trigger_metadata(
        "one",
        Duration::ZERO,
        Duration::ZERO,
        Some("a"),
        0,
    )]);
    assert!(runtime.take_active_trigger_missing_close_request());
    assert!(!runtime.take_active_trigger_missing_close_request());
}

#[test]
fn prevent_unmount_on_close_keeps_mounted_until_normal_close() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        false,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        true,
        true,
        true,
    );
    assert!(!runtime.open_value());
    assert!(runtime.mounted_value(false));

    // A later normal close (mounted-only) is still a change and unmounts.
    let outcome = runtime.request_close(false, None);
    assert!(outcome.changed());
    runtime.commit_open(
        false,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert!(!runtime.mounted_value(false));
}

#[test]
fn clear_prevent_unmount_unmounts_immediately() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        false,
        None,
        PreviewCardOpenChangeSource::Imperative,
        true,
        true,
        false,
    );
    assert!(runtime.mounted_value(false));
    assert!(runtime.clear_prevent_unmount());
    assert!(!runtime.mounted_value(false));
}

#[test]
fn effective_placement_and_positioner_state_facts() {
    let mut runtime = runtime_with_triggers(true);
    runtime.set_trigger_bounds(
        &ElementId::from("one"),
        Bounds::new(point(px(100.0), px(100.0)), size(px(80.0), px(20.0))),
    );
    runtime.set_popup_bounds(Bounds::new(
        point(px(90.0), px(130.0)),
        size(px(120.0), px(60.0)),
    ));
    runtime.set_available_size(size(px(640.0), px(480.0)));
    assert!(runtime.set_effective_placement(PreviewCardSide::Top, PreviewCardAlign::Start));
    assert!(!runtime.set_effective_placement(PreviewCardSide::Top, PreviewCardAlign::Start));

    let state = runtime.positioner_state(PreviewCardSide::Bottom, PreviewCardAlign::Center, false);
    assert!(state.open);
    assert!(state.anchor_available);
    assert_eq!(state.anchor_width, Some(px(80.0)));
    assert_eq!(state.popup_height, Some(px(60.0)));
    assert_eq!(state.available_width, Some(px(640.0)));

    let popup = runtime.popup_state(PreviewCardSide::Bottom, PreviewCardAlign::Center, false);
    assert_eq!(popup.side, PreviewCardSide::Top);
    assert_eq!(popup.align, PreviewCardAlign::Start);
}

#[test]
fn arrow_state_offsets_follow_effective_side_and_padding() {
    let mut runtime = runtime_with_triggers(true);
    runtime.set_trigger_bounds(
        &ElementId::from("one"),
        Bounds::new(point(px(100.0), px(100.0)), size(px(80.0), px(20.0))),
    );
    runtime.set_popup_bounds(Bounds::new(
        point(px(90.0), px(130.0)),
        size(px(120.0), px(60.0)),
    ));
    runtime.set_arrow_bounds(Bounds::new(point(px(0.0), px(0.0)), size(px(8.0), px(8.0))));
    runtime.set_arrow_padding(px(5.0));
    runtime.set_effective_placement(PreviewCardSide::Bottom, PreviewCardAlign::Center);

    let state = runtime.arrow_state(PreviewCardSide::Bottom, PreviewCardAlign::Center);
    assert!(state.open);
    assert_eq!(state.side, PreviewCardSide::Bottom);
    assert_eq!(state.padding, px(5.0));
    assert_eq!(state.offset_y, Some(px(0.0)));
    // Trigger center x = 140, popup left = 90 -> arrow center 50, minus half width.
    assert_eq!(state.offset_x, Some(px(46.0)));
    assert!(!state.uncentered);
}

#[test]
fn arrow_center_clamps_to_arrow_padding() {
    let mut runtime = runtime_with_triggers(true);
    runtime.set_trigger_bounds(
        &ElementId::from("one"),
        Bounds::new(point(px(0.0), px(100.0)), size(px(4.0), px(20.0))),
    );
    runtime.set_popup_bounds(Bounds::new(
        point(px(0.0), px(130.0)),
        size(px(120.0), px(60.0)),
    ));
    runtime.set_arrow_bounds(Bounds::new(point(px(0.0), px(0.0)), size(px(8.0), px(8.0))));
    runtime.set_arrow_padding(px(5.0));
    runtime.set_effective_placement(PreviewCardSide::Bottom, PreviewCardAlign::Center);

    let state = runtime.arrow_state(PreviewCardSide::Bottom, PreviewCardAlign::Center);
    assert_eq!(state.offset_x, Some(px(5.0)));
    assert!(state.uncentered);
}

#[test]
fn viewport_activation_direction_follows_trigger_bounds() {
    let mut runtime = runtime_with_triggers(true);
    runtime.set_trigger_bounds(
        &ElementId::from("one"),
        Bounds::new(point(px(0.0), px(0.0)), size(px(40.0), px(20.0))),
    );
    runtime.set_trigger_bounds(
        &ElementId::from("two"),
        Bounds::new(point(px(200.0), px(0.0)), size(px(40.0), px(20.0))),
    );
    runtime.commit_open(
        true,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    runtime.commit_open(
        true,
        Some(ElementId::from("two")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    let state = runtime.viewport_state();
    assert_eq!(
        state.activation_direction,
        PreviewCardActivationDirection::Right
    );
    assert_eq!(state.previous_trigger_id, Some(ElementId::from("one")));
    assert_eq!(state.current_trigger_id, Some(ElementId::from("two")));
}

#[test]
fn rapid_trigger_switching_settles_on_latest() {
    let mut runtime = runtime_with_triggers(true);
    for id in ["one", "two", "one", "two"] {
        runtime.commit_open(
            true,
            Some(ElementId::from(id)),
            PreviewCardOpenChangeSource::Pointer,
            false,
            true,
            true,
        );
    }
    assert_eq!(runtime.active_payload(), Some("payload-two"));
    assert_eq!(
        runtime.viewport_state().current_trigger_id,
        Some(ElementId::from("two"))
    );
}

#[test]
fn safe_polygon_arm_evaluate_and_disarm() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        true,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    runtime.set_trigger_bounds(
        &ElementId::from("one"),
        Bounds::new(point(px(100.0), px(100.0)), size(px(80.0), px(20.0))),
    );
    runtime.set_popup_bounds(Bounds::new(
        point(px(90.0), px(140.0)),
        size(px(120.0), px(60.0)),
    ));
    runtime.set_effective_placement(PreviewCardSide::Bottom, PreviewCardAlign::Center);

    assert!(runtime.arm_safe_polygon(point(px(140.0), px(119.0))));
    assert!(runtime.safe_polygon_armed());

    // The trough between trigger and popup is unconditionally Inside.
    assert_eq!(
        runtime.evaluate_safe_polygon(point(px(140.0), px(130.0)), Duration::from_millis(1)),
        Some(SafePolygonVerdict::Inside)
    );

    // Landing on the popup disarms the tracker.
    assert_eq!(
        runtime.evaluate_safe_polygon(point(px(140.0), px(150.0)), Duration::from_millis(2)),
        Some(SafePolygonVerdict::LandedPopup)
    );
    assert!(!runtime.safe_polygon_armed());
    assert_eq!(
        runtime.evaluate_safe_polygon(point(px(140.0), px(150.0)), Duration::from_millis(3)),
        None
    );
}

#[test]
fn arm_safe_polygon_requires_bounds() {
    let mut runtime = runtime_with_triggers(true);
    assert!(!runtime.arm_safe_polygon(point(px(0.0), px(0.0))));
}

#[test]
fn closing_commit_disarms_safe_polygon() {
    let mut runtime = runtime_with_triggers(true);
    runtime.commit_open(
        true,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    runtime.set_trigger_bounds(
        &ElementId::from("one"),
        Bounds::new(point(px(100.0), px(100.0)), size(px(80.0), px(20.0))),
    );
    runtime.set_popup_bounds(Bounds::new(
        point(px(90.0), px(140.0)),
        size(px(120.0), px(60.0)),
    ));
    assert!(runtime.arm_safe_polygon(point(px(140.0), px(119.0))));
    runtime.commit_open(
        false,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert!(!runtime.safe_polygon_armed());
}

#[test]
fn focus_change_open_and_close() {
    let mut runtime = runtime_with_triggers(false);
    let change = runtime.sync_focused_trigger(Some(ElementId::from("one")));
    assert_eq!(
        change,
        crate::preview_card::PreviewCardFocusChange::Open(ElementId::from("one"))
    );

    runtime.commit_open(
        true,
        Some(ElementId::from("one")),
        PreviewCardOpenChangeSource::Focus,
        false,
        true,
        true,
    );
    let change = runtime.sync_focused_trigger(None);
    assert_eq!(change, crate::preview_card::PreviewCardFocusChange::Close);
}
