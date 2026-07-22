use std::time::Duration;

use gpui::{point, px, size, Bounds, Pixels};

use crate::navigation_menu::{
    NavigationMenuActivationDirection, NavigationMenuBoundsKind, NavigationMenuHoverTarget,
    NavigationMenuInstant, NavigationMenuItemMetadata, NavigationMenuListEntry, NavigationMenuMove,
    NavigationMenuOrientation, NavigationMenuRuntime, NavigationMenuValueChangeReason,
};
use crate::primitives::safe_polygon::SafePolygonVerdict;

fn bounds(x: f32, y: f32, w: f32, h: f32) -> Bounds<Pixels> {
    Bounds::new(point(px(x), px(y)), size(px(w), px(h)))
}

fn runtime_with_items(
    value: Option<&'static str>,
    orientation: NavigationMenuOrientation,
) -> NavigationMenuRuntime<&'static str> {
    let mut runtime = NavigationMenuRuntime::new(value, orientation);
    runtime.sync_children(
        vec![
            NavigationMenuItemMetadata::new("one", false, None, 0),
            NavigationMenuItemMetadata::new("two", false, None, 1),
            NavigationMenuItemMetadata::new("disabled", true, None, 2),
        ],
        vec![
            NavigationMenuListEntry::new(None, Some("one"), false),
            NavigationMenuListEntry::new(None, Some("two"), false),
            NavigationMenuListEntry::new(None, Some("disabled"), true),
        ],
    );
    runtime.set_bounds(
        NavigationMenuBoundsKind::Trigger("one"),
        bounds(0.0, 0.0, 50.0, 20.0),
    );
    runtime.set_bounds(
        NavigationMenuBoundsKind::Trigger("two"),
        bounds(60.0, 0.0, 50.0, 20.0),
    );
    runtime
}

#[test]
fn uncontrolled_default_closed_and_initial_open() {
    let closed: NavigationMenuRuntime<&'static str> =
        NavigationMenuRuntime::new(None, NavigationMenuOrientation::Horizontal);
    assert!(!closed.open_value());
    assert_eq!(closed.instant(), NavigationMenuInstant::None);

    let open: NavigationMenuRuntime<&'static str> =
        NavigationMenuRuntime::new(Some("one"), NavigationMenuOrientation::Horizontal);
    assert!(open.open_value());
    assert_eq!(open.instant(), NavigationMenuInstant::Initial);
}

#[test]
fn controlled_reconciliation_takes_precedence() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);
    runtime.reconcile(Some(Some("two")));
    assert!(runtime.is_active_value(&"two"));

    // No controlled value: uncontrolled state untouched.
    runtime.reconcile(None);
    assert!(runtime.is_active_value(&"two"));

    runtime.reconcile(Some(None));
    assert!(!runtime.open_value());
}

#[test]
fn request_value_deduplicates_and_respects_disabled() {
    let runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);

    assert!(!runtime.request_value(Some("one"), Some("one")).changed());
    assert!(runtime.request_value(Some("one"), Some("two")).changed());
    assert!(runtime.request_value(Some("one"), None).changed());
    assert!(!runtime
        .request_value(Some("one"), Some("disabled"))
        .changed());
}

#[test]
fn activation_direction_horizontal_and_reset_on_close() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);

    // First open: direction None.
    runtime.commit_value(Some("one"), true);
    assert_eq!(
        runtime.viewport_state().activation_direction,
        NavigationMenuActivationDirection::None
    );

    // Switch right.
    runtime.commit_value(Some("two"), true);
    assert_eq!(
        runtime.viewport_state().activation_direction,
        NavigationMenuActivationDirection::Right
    );

    // Switch left.
    runtime.commit_value(Some("one"), true);
    assert_eq!(
        runtime.viewport_state().activation_direction,
        NavigationMenuActivationDirection::Left
    );

    // Close resets.
    runtime.commit_value(None, true);
    assert_eq!(
        runtime.viewport_state().activation_direction,
        NavigationMenuActivationDirection::None
    );
}

#[test]
fn activation_direction_vertical() {
    let mut runtime: NavigationMenuRuntime<&'static str> =
        NavigationMenuRuntime::new(None, NavigationMenuOrientation::Vertical);
    runtime.sync_children(
        vec![
            NavigationMenuItemMetadata::new("one", false, None, 0),
            NavigationMenuItemMetadata::new("two", false, None, 1),
        ],
        vec![
            NavigationMenuListEntry::new(None, Some("one"), false),
            NavigationMenuListEntry::new(None, Some("two"), false),
        ],
    );
    runtime.set_bounds(
        NavigationMenuBoundsKind::Trigger("one"),
        bounds(0.0, 0.0, 50.0, 20.0),
    );
    runtime.set_bounds(
        NavigationMenuBoundsKind::Trigger("two"),
        bounds(0.0, 30.0, 50.0, 20.0),
    );

    runtime.commit_value(Some("one"), true);
    runtime.commit_value(Some("two"), true);
    assert_eq!(
        runtime.viewport_state().activation_direction,
        NavigationMenuActivationDirection::Down
    );
    runtime.commit_value(Some("one"), true);
    assert_eq!(
        runtime.viewport_state().activation_direction,
        NavigationMenuActivationDirection::Up
    );
}

#[test]
fn retargeting_switches_anchor_and_records_morph_facts() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);
    runtime.commit_value(Some("one"), true);
    assert_eq!(runtime.anchor_bounds(), Some(bounds(0.0, 0.0, 50.0, 20.0)));

    runtime.set_bounds(
        NavigationMenuBoundsKind::Popup,
        bounds(0.0, 20.0, 200.0, 100.0),
    );
    runtime.commit_value(Some("two"), true);
    assert_eq!(runtime.anchor_bounds(), Some(bounds(60.0, 0.0, 50.0, 20.0)));
    assert_eq!(
        runtime.viewport_state().previous_popup_size,
        Some(size(px(200.0), px(100.0)))
    );
}

#[test]
fn anchor_falls_back_to_last_known_bounds_when_trigger_unmounts() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);
    runtime.commit_value(Some("two"), true);
    assert_eq!(runtime.anchor_bounds(), Some(bounds(60.0, 0.0, 50.0, 20.0)));

    // The active item unmounts: only "one" remains.
    runtime.sync_children(
        vec![NavigationMenuItemMetadata::new("one", false, None, 0)],
        vec![NavigationMenuListEntry::new(None, Some("one"), false)],
    );
    assert_eq!(runtime.anchor_bounds(), Some(bounds(60.0, 0.0, 50.0, 20.0)));
}

#[test]
fn hover_timer_generations_cancel_safely() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);

    let target = NavigationMenuHoverTarget::Open("one");
    let generation = runtime.schedule_hover(target.clone());
    // A stale generation never fires.
    assert!(!runtime.take_scheduled_hover(generation.wrapping_sub(1), &target));
    assert!(runtime.take_scheduled_hover(generation, &target));
    // Taken once only.
    assert!(!runtime.take_scheduled_hover(generation, &target));

    let generation = runtime.schedule_hover(NavigationMenuHoverTarget::Close);
    runtime.cancel_hover();
    assert!(!runtime.take_scheduled_hover(generation, &NavigationMenuHoverTarget::Close));
}

#[test]
fn patient_click_threshold_blocks_early_toggle_close() {
    let mut runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);
    runtime.note_hover_open(Duration::from_millis(1000));

    assert!(runtime.patient_click_blocks_close(Duration::from_millis(1200)));
    assert!(!runtime.patient_click_blocks_close(Duration::from_millis(1600)));
}

#[test]
fn no_patient_click_state_without_hover_open() {
    let runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);
    assert!(!runtime.patient_click_blocks_close(Duration::from_millis(100)));
}

#[test]
fn safe_polygon_arm_evaluate_and_land() {
    let mut runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);
    runtime.commit_value(Some("one"), true);
    runtime.set_bounds(
        NavigationMenuBoundsKind::Popup,
        bounds(0.0, 30.0, 200.0, 100.0),
    );

    assert!(runtime.arm_safe_polygon(point(px(25.0), px(19.0))));
    assert!(runtime.safe_polygon_armed());

    // Trough between trigger and popup is inside.
    assert_eq!(
        runtime.evaluate_safe_polygon(point(px(25.0), px(25.0)), Duration::from_millis(1)),
        Some(SafePolygonVerdict::Inside)
    );
    // Landing on the popup disarms.
    assert_eq!(
        runtime.evaluate_safe_polygon(point(px(25.0), px(50.0)), Duration::from_millis(2)),
        Some(SafePolygonVerdict::LandedPopup)
    );
    assert!(!runtime.safe_polygon_armed());
    assert_eq!(
        runtime.evaluate_safe_polygon(point(px(25.0), px(50.0)), Duration::from_millis(3)),
        None
    );
}

#[test]
fn outside_press_hit_testing() {
    let mut runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);
    runtime.set_bounds(
        NavigationMenuBoundsKind::Popup,
        bounds(0.0, 30.0, 200.0, 100.0),
    );

    // Inside popup: no dismiss.
    assert!(!runtime.outside_press_should_dismiss(point(px(50.0), px(60.0))));
    // On a sibling trigger of the tree: no dismiss.
    assert!(!runtime.outside_press_should_dismiss(point(px(70.0), px(10.0))));
    // Genuinely outside: dismiss.
    assert!(runtime.outside_press_should_dismiss(point(px(300.0), px(300.0))));
}

#[test]
fn instant_classification_for_initial_open_and_resize() {
    let mut runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);
    assert_eq!(runtime.instant(), NavigationMenuInstant::Initial);

    // First measurement does not classify as resize.
    runtime.set_available_size(size(px(800.0), px(600.0)));
    assert_eq!(runtime.instant(), NavigationMenuInstant::Initial);

    // A committed change clears the instant fact.
    runtime.record_change(NavigationMenuValueChangeReason::TriggerHover);
    assert_eq!(runtime.instant(), NavigationMenuInstant::None);

    // A viewport size change while open classifies as resize.
    runtime.set_available_size(size(px(700.0), px(500.0)));
    assert_eq!(runtime.instant(), NavigationMenuInstant::Resize);
}

#[test]
fn focus_return_blocked_for_hover_outside_press_and_focus_out() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);

    runtime.record_change(NavigationMenuValueChangeReason::TriggerHover);
    assert!(runtime.focus_return_blocked());
    runtime.record_change(NavigationMenuValueChangeReason::OutsidePress);
    assert!(runtime.focus_return_blocked());
    runtime.record_change(NavigationMenuValueChangeReason::FocusOut);
    assert!(runtime.focus_return_blocked());

    runtime.record_change(NavigationMenuValueChangeReason::EscapeKey);
    assert!(!runtime.focus_return_blocked());
    runtime.record_change(NavigationMenuValueChangeReason::TriggerPress);
    assert!(!runtime.focus_return_blocked());
}

#[test]
fn roving_highlight_clamps_at_ends() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);

    assert_eq!(runtime.highlighted_index(), Some(0));
    runtime.move_highlight(NavigationMenuMove::Previous);
    assert_eq!(runtime.highlighted_index(), Some(0));
    runtime.move_highlight(NavigationMenuMove::Next);
    assert_eq!(runtime.highlighted_index(), Some(1));
    runtime.move_highlight(NavigationMenuMove::Last);
    assert_eq!(runtime.highlighted_index(), Some(2));
    runtime.move_highlight(NavigationMenuMove::Next);
    assert_eq!(runtime.highlighted_index(), Some(2));
    runtime.move_highlight(NavigationMenuMove::First);
    assert_eq!(runtime.highlighted_index(), Some(0));
}

#[test]
fn highlighted_trigger_value_skips_disabled_entries() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);

    runtime.set_highlight(0);
    assert_eq!(runtime.highlighted_trigger_value(), Some("one"));
    runtime.set_highlight(2);
    assert_eq!(runtime.highlighted_trigger_value(), None);
}

#[test]
fn content_state_keep_mounted_reports_closed_style() {
    let runtime = runtime_with_items(Some("one"), NavigationMenuOrientation::Horizontal);

    let active = runtime.content_state(&"one", false);
    assert!(active.open && active.mounted);

    let inactive_unmounted = runtime.content_state(&"two", false);
    assert!(!inactive_unmounted.open && !inactive_unmounted.mounted);

    let inactive_kept = runtime.content_state(&"two", true);
    assert!(!inactive_kept.open && inactive_kept.mounted);
}

#[test]
fn sync_children_preserves_measured_trigger_bounds() {
    let mut runtime = runtime_with_items(None, NavigationMenuOrientation::Horizontal);
    runtime.sync_children(
        vec![
            NavigationMenuItemMetadata::new("one", false, None, 0),
            NavigationMenuItemMetadata::new("two", false, None, 1),
        ],
        vec![
            NavigationMenuListEntry::new(None, Some("one"), false),
            NavigationMenuListEntry::new(None, Some("two"), false),
        ],
    );
    runtime.commit_value(Some("one"), true);
    assert_eq!(runtime.anchor_bounds(), Some(bounds(0.0, 0.0, 50.0, 20.0)));
}
