use std::time::Duration;

use gpui::{point, px, size, Bounds, ElementId};

use crate::tooltip::{
    TooltipAlign, TooltipDelayGroup, TooltipHoverTarget, TooltipInstant, TooltipOpenChangeDetails,
    TooltipOpenChangeReason, TooltipOpenChangeSource, TooltipProps, TooltipProviderConfig,
    TooltipRuntime, TooltipSide, TooltipTrackCursorAxis, TooltipTriggerMetadata,
};

fn trigger<P: Clone + 'static>(
    id: &'static str,
    payload: Option<P>,
    order: usize,
) -> TooltipTriggerMetadata<P> {
    let id = ElementId::from(id);
    TooltipTriggerMetadata::new_without_focus(
        id.clone(),
        id,
        false,
        Duration::ZERO,
        Duration::ZERO,
        true,
        payload,
        order,
        false,
    )
}

#[test]
fn uncontrolled_default_closed_state_is_reflected() {
    let runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    let props = TooltipProps::new(
        false,
        false,
        TooltipTrackCursorAxis::None,
        TooltipProviderConfig::default(),
        None,
        None,
    );

    let state = runtime.root_state(&props);

    assert!(!state.open);
    assert!(!state.mounted);
}

#[test]
fn default_open_state_is_reflected() {
    let runtime = TooltipRuntime::<()>::new(true, None, false, false, TooltipTrackCursorAxis::None);
    let props = TooltipProps::new(
        false,
        false,
        TooltipTrackCursorAxis::None,
        TooltipProviderConfig::default(),
        None,
        None,
    );

    let state = runtime.root_state(&props);

    assert!(state.open);
    assert!(state.mounted);
}

#[test]
fn disabled_default_open_renders_closed() {
    let runtime = TooltipRuntime::<()>::new(true, None, true, false, TooltipTrackCursorAxis::None);
    let props = TooltipProps::new(
        true,
        false,
        TooltipTrackCursorAxis::None,
        TooltipProviderConfig::default(),
        None,
        None,
    );

    assert!(!runtime.root_state(&props).open);
}

#[test]
fn disabled_root_requests_close_for_open_tooltip() {
    let mut runtime =
        TooltipRuntime::<()>::new(true, None, false, false, TooltipTrackCursorAxis::None);

    assert!(runtime.sync_root_options(true, false, TooltipTrackCursorAxis::None));
    let outcome = runtime.request_close(runtime.raw_open_value(), None);

    assert!(outcome.changed());
    assert!(!outcome.open());
}

#[test]
fn default_open_with_default_trigger_resolves_payload() {
    let id = ElementId::from("trigger");
    let mut runtime =
        TooltipRuntime::new(true, Some(id), false, false, TooltipTrackCursorAxis::None);
    runtime.sync_triggers(vec![trigger("trigger", Some("payload".to_string()), 0)]);

    assert_eq!(runtime.active_payload().as_deref(), Some("payload"));
}

#[test]
fn controlled_missing_trigger_does_not_invent_fallback_trigger() {
    let missing = ElementId::from("missing");
    let mut runtime = TooltipRuntime::new(
        true,
        Some(missing.clone()),
        false,
        false,
        TooltipTrackCursorAxis::None,
    );
    runtime.sync_triggers(vec![trigger("available", Some("payload".to_string()), 0)]);

    assert_eq!(runtime.active_trigger_id().as_ref(), Some(&missing));
    assert!(runtime.active_payload().is_none());
}

#[test]
fn controlled_open_reconciliation_overwrites_internal_open_state() {
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.commit_open(
        true,
        None,
        TooltipOpenChangeSource::Imperative,
        false,
        true,
        true,
    );

    runtime.sync_open_from_context(false);

    assert!(!runtime.open_value());
}

#[test]
fn canceled_open_and_close_details_are_observable() {
    let mut details = TooltipOpenChangeDetails::<String>::new(
        TooltipOpenChangeReason::TriggerHover,
        TooltipOpenChangeSource::Pointer,
        Some(ElementId::from("trigger")),
        Some("payload".to_string()),
        true,
    );

    details.cancel();

    assert!(details.is_canceled());
    assert_eq!(details.reason(), TooltipOpenChangeReason::TriggerHover);
    assert_eq!(details.source(), TooltipOpenChangeSource::Pointer);
    assert_eq!(
        details.trigger_id().map(ToString::to_string),
        Some("trigger".to_string())
    );
    assert_eq!(details.payload().map(String::as_str), Some("payload"));
}

#[test]
fn prevent_unmount_on_close_keeps_portal_mounted_until_normal_close() {
    let mut runtime =
        TooltipRuntime::<()>::new(true, None, false, false, TooltipTrackCursorAxis::None);
    runtime.commit_open(
        false,
        None,
        TooltipOpenChangeSource::Pointer,
        true,
        true,
        true,
    );

    assert!(!runtime.open_value());
    assert!(runtime.portal_state(false).mounted);

    runtime.commit_open(
        false,
        None,
        TooltipOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert!(!runtime.portal_state(false).mounted);
}

#[test]
fn delay_group_tracks_instant_handoff_generation() {
    let group = TooltipDelayGroup::new();

    assert_eq!(group.instant(), TooltipInstant::Delay);
    let first = group.mark_recently_visible();
    assert_eq!(group.instant(), TooltipInstant::Instant);
    let second = group.mark_recently_visible();

    assert!(!group.clear_recently_visible(first));
    assert_eq!(group.instant(), TooltipInstant::Instant);
    assert!(group.clear_recently_visible(second));
    assert_eq!(group.instant(), TooltipInstant::Delay);
}

#[test]
fn provider_config_preserves_delay_close_delay_and_timeout() {
    let config = TooltipProviderConfig::new(
        Duration::from_millis(10),
        Duration::from_millis(20),
        Duration::from_millis(30),
    );

    assert_eq!(config.delay(), Duration::from_millis(10));
    assert_eq!(config.close_delay(), Duration::from_millis(20));
    assert_eq!(config.timeout(), Duration::from_millis(30));
}

#[test]
fn runtime_syncs_provider_delay_group_facts() {
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    let props = TooltipProps::new(
        false,
        false,
        TooltipTrackCursorAxis::None,
        TooltipProviderConfig::default(),
        None,
        None,
    );

    assert!(runtime.sync_provider_delay_group(
        TooltipInstant::Instant,
        Some(ElementId::from("provider-root")),
    ));
    assert_eq!(
        runtime.provider_active_root_id().map(|id| id.to_string()),
        Some("provider-root".to_string())
    );
    assert_eq!(runtime.root_state(&props).instant, TooltipInstant::Instant);
}

#[test]
fn canceling_pending_hover_open_rejects_stale_timer_generation() {
    let id = ElementId::from("trigger");
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);

    let generation = runtime.schedule_hover(TooltipHoverTarget::Open, Some(id.clone()));
    runtime.cancel_hover();

    assert!(!runtime.take_scheduled_hover(generation, TooltipHoverTarget::Open, Some(&id),));
    assert!(runtime.pending_hover().is_none());
}

#[test]
fn pending_hover_open_survives_without_explicit_cancel() {
    let id = ElementId::from("trigger");
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);

    let generation = runtime.schedule_hover(TooltipHoverTarget::Open, Some(id.clone()));

    assert!(runtime.take_scheduled_hover(generation, TooltipHoverTarget::Open, Some(&id),));
    assert!(runtime.pending_hover().is_none());
}

#[test]
fn close_on_click_trigger_press_cancels_pending_hover_open() {
    let id = ElementId::from("trigger");
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    let generation = runtime.schedule_hover(TooltipHoverTarget::Open, Some(id.clone()));

    let press_change = runtime.sync_trigger_press(id.clone(), true, false);

    assert!(!press_change.close_active());
    assert!(!press_change.open_detached_focus());
    assert!(!runtime.take_scheduled_hover(generation, TooltipHoverTarget::Open, Some(&id),));
}

#[test]
fn close_on_click_false_preserves_pending_hover_open() {
    let id = ElementId::from("trigger");
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    let generation = runtime.schedule_hover(TooltipHoverTarget::Open, Some(id.clone()));

    let press_change = runtime.sync_trigger_press(id.clone(), false, false);

    assert!(!press_change.close_active());
    assert!(!press_change.open_detached_focus());
    assert!(runtime.take_scheduled_hover(generation, TooltipHoverTarget::Open, Some(&id),));
}

#[test]
fn close_on_click_false_preserves_open_active_trigger() {
    let id = ElementId::from("trigger");
    let mut runtime = TooltipRuntime::new(
        true,
        Some(id.clone()),
        false,
        false,
        TooltipTrackCursorAxis::None,
    );
    runtime.sync_triggers(vec![trigger("trigger", None::<()>, 0)]);

    let press_change = runtime.sync_trigger_press(id, false, false);

    assert!(!press_change.close_active());
    assert!(runtime.open_value());
}

#[test]
fn disabled_trigger_hover_deactivates_previous_open_trigger() {
    let active_id = ElementId::from("active");
    let disabled_id = ElementId::from("disabled");
    let mut runtime = TooltipRuntime::new(
        true,
        Some(active_id.clone()),
        false,
        false,
        TooltipTrackCursorAxis::None,
    );
    runtime.sync_triggers(vec![
        trigger("active", None::<()>, 0),
        TooltipTriggerMetadata::new_without_focus(
            disabled_id.clone(),
            disabled_id.clone(),
            true,
            Duration::ZERO,
            Duration::ZERO,
            true,
            None,
            1,
            false,
        ),
    ]);
    runtime.schedule_hover(TooltipHoverTarget::Close, Some(active_id));

    assert!(runtime.sync_disabled_trigger_hover(&disabled_id));
    assert!(runtime.pending_hover().is_none());
}

#[test]
fn detached_trigger_focus_requests_open_and_blur_requests_close() {
    let id = ElementId::from("detached");
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.register_detached_trigger(TooltipTriggerMetadata::new_without_focus(
        id.clone(),
        id.clone(),
        false,
        Duration::ZERO,
        Duration::ZERO,
        true,
        None,
        0,
        true,
    ));

    assert_eq!(
        runtime.sync_detached_trigger_focus(id.clone(), true),
        crate::tooltip::TooltipFocusChange::Open(id.clone())
    );
    runtime.commit_open(
        true,
        Some(id.clone()),
        TooltipOpenChangeSource::Focus,
        false,
        true,
        true,
    );
    assert_eq!(
        runtime.sync_detached_trigger_focus(id, false),
        crate::tooltip::TooltipFocusChange::Close
    );
}

#[test]
fn active_trigger_unmount_requests_close_once() {
    let active_id = ElementId::from("trigger");
    let mut runtime = TooltipRuntime::new(
        true,
        Some(active_id),
        false,
        false,
        TooltipTrackCursorAxis::None,
    );
    runtime.sync_triggers(vec![trigger("trigger", None::<()>, 0)]);

    runtime.sync_triggers(Vec::new());

    assert!(runtime.take_active_trigger_missing_close_request());
    assert!(!runtime.take_active_trigger_missing_close_request());
}

#[test]
fn runtime_domain_command_aliases_are_available() {
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.sync_children(vec![trigger("trigger", None::<()>, 0)]);

    let outcome = runtime.activate_trigger(false, ElementId::from("trigger"));
    assert!(outcome.changed());
    assert!(outcome.open());
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        TooltipOpenChangeSource::Imperative,
        false,
        true,
        true,
    );

    assert!(runtime.reconcile(true, false, TooltipTrackCursorAxis::None));
}

#[test]
fn active_trigger_switch_updates_payload() {
    let mut runtime = TooltipRuntime::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.sync_triggers(vec![
        trigger("first", Some("alpha".to_string()), 0),
        trigger("second", Some("beta".to_string()), 1),
    ]);

    let outcome = runtime.request_open_change(false, true, Some(ElementId::from("first")));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        TooltipOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    assert_eq!(runtime.active_payload().as_deref(), Some("alpha"));

    let outcome = runtime.request_open_change(true, true, Some(ElementId::from("second")));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        TooltipOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert_eq!(runtime.active_payload().as_deref(), Some("beta"));
    let viewport_state = runtime.viewport_state();
    assert_eq!(
        viewport_state.activation_direction,
        crate::tooltip::TooltipActivationDirection::Forward
    );
    assert_eq!(
        viewport_state
            .previous_trigger_id
            .as_ref()
            .map(ToString::to_string),
        Some("first".to_string())
    );
    assert_eq!(
        viewport_state
            .current_trigger_id
            .as_ref()
            .map(ToString::to_string),
        Some("second".to_string())
    );
}

#[test]
fn rapid_trigger_switching_settles_on_latest_payload_and_direction() {
    let mut runtime = TooltipRuntime::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.sync_triggers(vec![
        trigger("first", Some("alpha".to_string()), 0),
        trigger("second", Some("beta".to_string()), 1),
        trigger("third", Some("gamma".to_string()), 2),
    ]);

    for id in ["first", "second", "third"] {
        let outcome = runtime.request_open_change(true, true, Some(ElementId::from(id)));
        let (open, trigger_id, _, _) = outcome.into_parts();
        runtime.commit_open(
            open,
            trigger_id,
            TooltipOpenChangeSource::Pointer,
            false,
            true,
            true,
        );
    }

    assert_eq!(runtime.active_payload().as_deref(), Some("gamma"));
    assert_eq!(
        runtime.viewport_state().activation_direction,
        crate::tooltip::TooltipActivationDirection::Forward
    );
}

#[test]
fn activation_direction_uses_trigger_bounds_when_available() {
    let first_id = ElementId::from("first");
    let second_id = ElementId::from("second");
    let mut runtime = TooltipRuntime::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.sync_triggers(vec![
        trigger("first", None::<()>, 0),
        trigger("second", None::<()>, 1),
    ]);
    runtime.set_trigger_bounds(
        &first_id,
        Bounds::new(point(px(10.0), px(60.0)), size(px(20.0), px(20.0))),
    );
    runtime.set_trigger_bounds(
        &second_id,
        Bounds::new(point(px(50.0), px(20.0)), size(px(20.0), px(20.0))),
    );

    let outcome = runtime.request_open_change(false, true, Some(first_id));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        TooltipOpenChangeSource::Pointer,
        false,
        true,
        true,
    );
    let outcome = runtime.request_open_change(true, true, Some(second_id));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        TooltipOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert_eq!(
        runtime.viewport_state().activation_direction,
        crate::tooltip::TooltipActivationDirection::RightUp
    );
}

#[test]
fn duplicate_trigger_id_uses_last_registered_trigger() {
    let mut runtime = TooltipRuntime::new(false, None, false, false, TooltipTrackCursorAxis::None);
    runtime.sync_triggers(vec![
        trigger("duplicate", Some("first".to_string()), 0),
        trigger("duplicate", Some("second".to_string()), 1),
    ]);

    let outcome = runtime.request_open_change(false, true, Some(ElementId::from("duplicate")));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        TooltipOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert_eq!(runtime.active_payload().as_deref(), Some("second"));
}

#[test]
fn missing_trigger_open_fails_deterministically() {
    let runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);

    let outcome = runtime.request_open_change(false, true, Some(ElementId::from("missing")));

    assert!(!outcome.changed());
    assert!(outcome.trigger_id().is_none());
    assert!(outcome.payload().is_none());
}

#[test]
fn disabled_trigger_does_not_open() {
    let mut runtime =
        TooltipRuntime::<()>::new(false, None, false, false, TooltipTrackCursorAxis::None);
    let id = ElementId::from("disabled");
    runtime.sync_triggers(vec![TooltipTriggerMetadata::new_without_focus(
        id.clone(),
        id.clone(),
        true,
        Duration::ZERO,
        Duration::ZERO,
        true,
        None,
        0,
        false,
    )]);

    let outcome = runtime.request_open_change(false, true, Some(id));

    assert!(!outcome.changed());
}

#[test]
fn cursor_tracking_can_replace_anchor_axes() {
    let id = ElementId::from("trigger");
    let mut runtime = TooltipRuntime::new(
        true,
        Some(id.clone()),
        false,
        false,
        TooltipTrackCursorAxis::Both,
    );
    runtime.sync_triggers(vec![trigger("trigger", None::<()>, 0)]);
    runtime.set_trigger_bounds(
        &id,
        Bounds::new(point(px(20.0), px(30.0)), size(px(100.0), px(40.0))),
    );
    runtime.set_cursor_position(point(px(200.0), px(210.0)));

    let state = runtime.positioner_state(TooltipSide::Top, TooltipAlign::Center, false);

    assert_eq!(
        state.anchor_bounds.unwrap().origin,
        point(px(200.0), px(210.0))
    );
    assert_eq!(state.anchor_bounds.unwrap().size, size(px(0.0), px(0.0)));
}
