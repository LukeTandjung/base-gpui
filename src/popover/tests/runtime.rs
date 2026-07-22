use std::time::Duration;

use gpui::{point, px, size, Bounds, ElementId};

use crate::popover::{
    PopoverAlign, PopoverOpenChangeDetails, PopoverOpenChangeReason, PopoverOpenChangeSource,
    PopoverProps, PopoverRuntime, PopoverSide, PopoverTriggerMetadata,
};

fn trigger<P: Clone + 'static>(
    id: &'static str,
    payload: Option<P>,
    order: usize,
) -> PopoverTriggerMetadata<P> {
    let id = ElementId::from(id);
    PopoverTriggerMetadata::new_without_focus(
        id.clone(),
        id,
        false,
        false,
        Duration::ZERO,
        Duration::ZERO,
        payload,
        order,
        false,
    )
}

#[test]
fn uncontrolled_default_closed_state_is_reflected() {
    let runtime = PopoverRuntime::<()>::new(false, None, false);
    let props = PopoverProps::new(false, None, None);

    let state = runtime.root_state(&props);

    assert!(!state.open);
    assert!(!state.mounted);
}

#[test]
fn default_open_state_is_reflected() {
    let runtime = PopoverRuntime::<()>::new(true, None, false);
    let props = PopoverProps::new(false, None, None);

    let state = runtime.root_state(&props);

    assert!(state.open);
    assert!(state.mounted);
}

#[test]
fn controlled_open_reconciliation_overwrites_internal_open_state() {
    let mut runtime = PopoverRuntime::<()>::new(false, None, false);
    runtime.commit_open(
        true,
        None,
        PopoverOpenChangeSource::Imperative,
        false,
        true,
        true,
    );

    runtime.sync_open_from_context(false);

    assert!(!runtime.open_value());
}

#[test]
fn canceled_open_and_close_details_are_observable() {
    let mut details = PopoverOpenChangeDetails::<String>::new(
        PopoverOpenChangeReason::TriggerPress,
        PopoverOpenChangeSource::Pointer,
        Some(ElementId::from("trigger")),
        Some("payload".to_string()),
        true,
    );

    details.cancel();

    assert!(details.is_canceled());
    assert_eq!(details.reason(), PopoverOpenChangeReason::TriggerPress);
    assert_eq!(details.source(), PopoverOpenChangeSource::Pointer);
    assert_eq!(
        details.trigger_id().map(ToString::to_string),
        Some("trigger".to_string())
    );
    assert_eq!(details.payload().map(String::as_str), Some("payload"));
}

#[test]
fn prevent_unmount_on_close_keeps_portal_mounted_until_normal_close() {
    let mut runtime = PopoverRuntime::<()>::new(true, None, false);
    runtime.commit_open(
        false,
        None,
        PopoverOpenChangeSource::Pointer,
        true,
        true,
        true,
    );

    assert!(!runtime.open_value());
    assert!(runtime.portal_state(false).mounted);

    runtime.commit_open(
        false,
        None,
        PopoverOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert!(!runtime.portal_state(false).mounted);
}

#[test]
fn active_trigger_switch_updates_payload() {
    let mut runtime = PopoverRuntime::new(false, None, false);
    runtime.sync_triggers(vec![
        trigger("first", Some("alpha".to_string()), 0),
        trigger("second", Some("beta".to_string()), 1),
    ]);

    let outcome = runtime.request_open_change(false, true, Some(ElementId::from("first")));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        PopoverOpenChangeSource::Pointer,
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
        PopoverOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert_eq!(runtime.active_payload().as_deref(), Some("beta"));
    assert_eq!(
        runtime.viewport_state().activation_direction,
        crate::popover::PopoverActivationDirection::Forward
    );
}

#[test]
fn duplicate_trigger_id_uses_last_registered_trigger() {
    let mut runtime = PopoverRuntime::new(false, None, false);
    runtime.sync_triggers(vec![
        trigger("duplicate", Some("first".to_string()), 0),
        trigger("duplicate", Some("second".to_string()), 1),
    ]);

    let outcome = runtime.request_open_change(false, true, Some(ElementId::from("duplicate")));
    let (open, trigger_id, _, _) = outcome.into_parts();
    runtime.commit_open(
        open,
        trigger_id,
        PopoverOpenChangeSource::Pointer,
        false,
        true,
        true,
    );

    assert_eq!(runtime.active_payload().as_deref(), Some("second"));
}

#[test]
fn missing_trigger_open_fails_deterministically() {
    let runtime = PopoverRuntime::<()>::new(false, None, false);

    let outcome = runtime.request_open_change(false, true, Some(ElementId::from("missing")));

    assert!(!outcome.changed());
    assert!(outcome.trigger_id().is_none());
    assert!(outcome.payload().is_none());
}

#[test]
fn disabled_trigger_does_not_open() {
    let mut runtime = PopoverRuntime::<()>::new(false, None, false);
    let id = ElementId::from("disabled");
    runtime.sync_triggers(vec![PopoverTriggerMetadata::new_without_focus(
        id.clone(),
        id.clone(),
        true,
        false,
        Duration::ZERO,
        Duration::ZERO,
        None,
        0,
        false,
    )]);

    let outcome = runtime.request_open_change(false, true, Some(id));

    assert!(!outcome.changed());
}

#[test]
fn modal_flag_is_reflected_in_root_state() {
    let runtime = PopoverRuntime::<()>::new(false, None, true);
    let props = PopoverProps::new(true, None, None);

    assert!(runtime.root_state(&props).modal);
}

#[test]
fn arrow_state_tracks_effective_side_and_clamps_to_popup_edges() {
    let id = ElementId::from("trigger");
    let mut runtime = PopoverRuntime::new(true, Some(id.clone()), false);
    runtime.sync_triggers(vec![trigger("trigger", None::<()>, 0)]);
    runtime.set_trigger_bounds(
        &id,
        Bounds::new(point(px(-20.0), px(20.0)), size(px(20.0), px(20.0))),
    );
    runtime.set_popup_bounds(Bounds::new(
        point(px(0.0), px(40.0)),
        size(px(100.0), px(80.0)),
    ));
    runtime.set_arrow_bounds(Bounds::new(
        point(px(0.0), px(0.0)),
        size(px(20.0), px(10.0)),
    ));

    let bottom_state = runtime.arrow_state(PopoverSide::Bottom, PopoverAlign::Center);
    assert_eq!(bottom_state.side, PopoverSide::Bottom);
    assert_eq!(bottom_state.offset_x, Some(px(4.0)));
    assert_eq!(bottom_state.offset_y, Some(px(0.0)));
    assert!(bottom_state.uncentered);

    runtime.set_effective_placement(PopoverSide::Top, PopoverAlign::Center);
    let top_state = runtime.arrow_state(PopoverSide::Bottom, PopoverAlign::Center);
    assert_eq!(top_state.side, PopoverSide::Top);
    assert_eq!(top_state.offset_y, Some(px(70.0)));
}

#[test]
fn popup_and_positioner_state_expose_side_align_and_mounting() {
    let runtime = PopoverRuntime::<()>::new(true, None, false);

    assert_eq!(
        runtime
            .popup_state(PopoverSide::Top, PopoverAlign::End, false)
            .side,
        PopoverSide::Top
    );
    assert!(
        runtime
            .positioner_state(PopoverSide::Bottom, PopoverAlign::Center, false)
            .mounted
    );
}
