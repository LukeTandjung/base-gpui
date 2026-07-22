use std::time::{Duration, Instant};

use gpui::{point, px, size, Bounds};

use crate::menu::{
    MenuChildHoverDirective, MenuInstantKind, MenuItemKind, MenuItemMetadata, MenuMove,
    MenuOpenChangeReason, MenuOpenChangeSource, MenuParentKind, MenuRuntime, MenuTypeaheadOutcome,
};

fn item(index: usize, label: &str) -> MenuItemMetadata {
    MenuItemMetadata::new(index, MenuItemKind::Item, Some(label.into()), false, true)
}

fn disabled_item(index: usize, label: &str) -> MenuItemMetadata {
    MenuItemMetadata::new(index, MenuItemKind::Item, Some(label.into()), true, true)
}

fn runtime_with_items(labels: &[&str]) -> MenuRuntime<()> {
    let mut runtime = MenuRuntime::new(true, MenuParentKind::None);
    runtime.sync_items(
        labels
            .iter()
            .enumerate()
            .map(|(index, label)| item(index, label))
            .collect(),
    );
    runtime
}

#[test]
fn uncontrolled_defaults_closed_and_default_open_true_starts_open() {
    let closed: MenuRuntime<()> = MenuRuntime::new(false, MenuParentKind::None);
    assert!(!closed.open_value());

    let open: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    assert!(open.open_value());
}

#[test]
fn request_open_change_dedupes_redundant_requests() {
    let runtime: MenuRuntime<()> = MenuRuntime::new(false, MenuParentKind::None);
    assert!(!runtime.request_open_change(false, false).changed());
    assert!(runtime.request_open_change(false, true).changed());

    let open: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    assert!(!open.request_open_change(true, true).changed());
    assert!(open.request_open_change(true, false).changed());
}

#[test]
fn controlled_sync_overrides_internal_open_state() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(false, MenuParentKind::None);
    runtime.sync_open_from_context(true);
    assert!(runtime.open_value());
    runtime.sync_open_from_context(false);
    assert!(!runtime.open_value());
}

#[test]
fn commit_open_close_resets_highlight_and_typeahead() {
    let mut runtime = runtime_with_items(&["Alpha", "Beta"]);
    runtime.move_highlight(MenuMove::First, true);
    runtime.apply_typeahead("a", Instant::now());
    runtime.commit_open(false, false, true);

    assert!(!runtime.open_value());
    assert!(runtime.highlighted_activation().is_none());
    assert!(!runtime.typeahead_active(Instant::now()));
}

#[test]
fn prevent_unmount_on_close_keeps_mounted_for_one_cycle() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.commit_open(false, true, true);
    assert!(runtime.mounted_value(false));

    // A mounted-only close is still a change and clears the flag.
    let outcome = runtime.request_open_change(false, false);
    assert!(outcome.changed());
    runtime.commit_open(false, false, true);
    assert!(!runtime.mounted_value(false));
}

#[test]
fn record_open_change_classifies_instant_kinds() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.record_open_change(
        MenuOpenChangeReason::TriggerPress,
        MenuOpenChangeSource::Keyboard,
    );
    assert_eq!(runtime.instant_kind(), MenuInstantKind::Click);

    runtime.record_open_change(
        MenuOpenChangeReason::EscapeKey,
        MenuOpenChangeSource::Keyboard,
    );
    assert_eq!(runtime.instant_kind(), MenuInstantKind::Dismiss);

    runtime.record_open_change(
        MenuOpenChangeReason::TriggerHover,
        MenuOpenChangeSource::Pointer,
    );
    assert_eq!(runtime.instant_kind(), MenuInstantKind::None);
    assert!(runtime.opened_by_hover());
}

#[test]
fn move_highlight_wraps_and_clamps() {
    let mut runtime = runtime_with_items(&["Alpha", "Beta", "Gamma"]);

    runtime.move_highlight(MenuMove::Next, true);
    assert!(runtime.item_is_tab_stop(Some(0)));
    runtime.move_highlight(MenuMove::Last, true);
    assert!(runtime.item_is_tab_stop(Some(2)));
    runtime.move_highlight(MenuMove::Next, true);
    assert!(runtime.item_is_tab_stop(Some(0)));

    runtime.move_highlight(MenuMove::Previous, false);
    assert!(runtime.item_is_tab_stop(Some(0)));
    runtime.move_highlight(MenuMove::Last, false);
    runtime.move_highlight(MenuMove::Next, false);
    assert!(runtime.item_is_tab_stop(Some(2)));
    runtime.move_highlight(MenuMove::First, false);
    assert!(runtime.item_is_tab_stop(Some(0)));
}

#[test]
fn disabled_items_are_highlightable_but_report_disabled_activation() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.sync_items(vec![item(0, "Alpha"), disabled_item(1, "Beta")]);

    runtime.move_highlight(MenuMove::Last, true);
    let (index, _kind, disabled, _close, _activation) =
        runtime.highlighted_activation().expect("highlighted");
    assert_eq!(index, 1);
    assert!(disabled);
}

#[test]
fn item_registration_keeps_dense_indices_with_labels_interleaved() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.sync_items(vec![item(0, "Alpha"), item(1, "Beta"), item(2, "Gamma")]);
    runtime.sync_group_labels(vec!["Group".into()]);

    runtime.move_highlight(MenuMove::Last, true);
    assert!(runtime.item_is_tab_stop(Some(2)));
    assert_eq!(runtime.group_labels().len(), 1);
}

#[test]
fn typeahead_matches_labels_and_skips_disabled() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.sync_items(vec![
        item(0, "Apple"),
        disabled_item(1, "Apricot"),
        item(2, "Banana"),
    ]);

    let now = Instant::now();
    assert_eq!(
        runtime.apply_typeahead("a", now),
        MenuTypeaheadOutcome::Moved
    );
    assert!(runtime.item_is_tab_stop(Some(0)));

    // Repeated character cycles among enabled matches, skipping the disabled
    // "Apricot" and returning to "Apple".
    assert_eq!(
        runtime.apply_typeahead("a", now),
        MenuTypeaheadOutcome::Moved
    );
    assert!(runtime.item_is_tab_stop(Some(0)));

    // After the reset window a new query starts fresh.
    let later = now + Duration::from_secs(1);
    assert_eq!(
        runtime.apply_typeahead("b", later),
        MenuTypeaheadOutcome::Moved
    );
    assert!(runtime.item_is_tab_stop(Some(2)));

    assert_eq!(
        runtime.apply_typeahead("z", later + Duration::from_secs(1)),
        MenuTypeaheadOutcome::NoMatch
    );
}

#[test]
fn typeahead_resets_after_timeout_and_reports_typing_session() {
    let mut runtime = runtime_with_items(&["Alpha", "Beta"]);
    let start = Instant::now();
    runtime.apply_typeahead("a", start);
    assert!(runtime.typeahead_active(start));
    assert!(!runtime.typeahead_active(start + Duration::from_secs(2)));

    // After the timeout the buffer restarts from the new character.
    runtime.apply_typeahead("b", start + Duration::from_secs(2));
    assert!(runtime.item_is_tab_stop(Some(1)));
}

#[test]
fn pointer_highlight_is_gated_on_real_pointer_movement() {
    let mut runtime = runtime_with_items(&["Alpha", "Beta"]);
    runtime.commit_open(true, false, true);

    assert!(!runtime.highlight_item_from_pointer(0));
    runtime.note_pointer_moved();
    assert!(runtime.highlight_item_from_pointer(0));
    assert!(runtime.item_is_tab_stop(Some(0)));
}

#[test]
fn checkbox_state_registers_default_once_and_commits() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.register_checkbox_default(0, true);
    runtime.register_checkbox_default(0, false);
    assert!(runtime.checkbox_checked(0));

    runtime.commit_checkbox(0, false);
    assert!(!runtime.checkbox_checked(0));
}

#[test]
fn radio_state_registers_default_once_and_commits_selection() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.register_radio_default(0, Some(2));
    runtime.register_radio_default(0, None);
    assert_eq!(runtime.radio_selected(0), Some(2));

    runtime.commit_radio(0, 1);
    assert_eq!(runtime.radio_selected(0), Some(1));

    // Selecting the already-selected value is deterministic.
    runtime.commit_radio(0, 1);
    assert_eq!(runtime.radio_selected(0), Some(1));
}

#[test]
fn submenu_open_close_bookkeeping_toggles_hover_enabled() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    assert!(runtime.hover_enabled());
    runtime.note_submenu_opened(3);
    assert_eq!(runtime.open_submenu_item(), Some(3));
    assert!(!runtime.hover_enabled());
    runtime.note_submenu_closed(3);
    assert_eq!(runtime.open_submenu_item(), None);
    assert!(runtime.hover_enabled());
}

#[test]
fn scheduled_child_close_is_generation_checked() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    let generation = runtime.schedule_child_close(1);
    assert_eq!(runtime.pending_child_close_item(), Some(1));

    // A canceled schedule invalidates the old generation.
    runtime.cancel_child_close();
    assert!(!runtime.take_scheduled_child_close(generation, 1));

    let generation = runtime.schedule_child_close(1);
    assert!(runtime.take_scheduled_child_close(generation, 1));
    assert!(!runtime.take_scheduled_child_close(generation, 1));
}

#[test]
fn reconcile_child_hover_schedules_and_cancels_delayed_close() {
    let mut runtime = runtime_with_items(&["Alpha", "Submenu"]);
    runtime.note_submenu_opened(1);

    // Highlighting a different parent item schedules the branch close.
    runtime.move_highlight(MenuMove::First, true);
    let directive = runtime.reconcile_child_hover();
    let MenuChildHoverDirective::ScheduleClose {
        item_index,
        close_delay,
        generation,
    } = directive
    else {
        panic!("expected schedule directive, got {directive:?}");
    };
    assert_eq!(item_index, 1);
    assert_eq!(close_delay, Duration::ZERO);

    // A second reconcile while pending is a no-op.
    assert_eq!(
        runtime.reconcile_child_hover(),
        MenuChildHoverDirective::None
    );

    // Re-highlighting the submenu trigger cancels the pending close.
    runtime.move_highlight(MenuMove::Last, true);
    assert_eq!(
        runtime.reconcile_child_hover(),
        MenuChildHoverDirective::CancelClose
    );
    assert!(!runtime.take_scheduled_child_close(generation, 1));
}

#[test]
fn outside_press_union_uses_popup_and_trigger_bounds() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.sync_trigger(Some(crate::menu::MenuTriggerMetadata::new(
        "trigger".into(),
        false,
        false,
        Duration::ZERO,
        Duration::ZERO,
        None,
        None,
    )));
    runtime.set_trigger_bounds(Bounds::new(
        point(px(0.0), px(0.0)),
        size(px(50.0), px(20.0)),
    ));
    runtime.set_popup_bounds(Bounds::new(
        point(px(0.0), px(20.0)),
        size(px(100.0), px(200.0)),
    ));

    assert!(runtime.own_tree_contains(point(px(10.0), px(10.0))));
    assert!(runtime.own_tree_contains(point(px(50.0), px(100.0))));
    assert!(!runtime.own_tree_contains(point(px(300.0), px(300.0))));

    // Closed menus only count the trigger.
    runtime.commit_open(false, false, true);
    assert!(!runtime.own_tree_contains(point(px(50.0), px(100.0))));
    assert!(runtime.own_tree_contains(point(px(10.0), px(10.0))));
}

#[test]
fn positioner_state_reports_nested_for_submenus() {
    let runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::Submenu);
    let state = runtime.positioner_state(Default::default(), Default::default(), false);
    assert!(state.nested);
    assert_eq!(runtime.parent_kind(), MenuParentKind::Submenu);
}

#[test]
fn submenu_trigger_state_tracks_open_branch_and_highlight() {
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    runtime.sync_items(vec![
        item(0, "Alpha"),
        MenuItemMetadata::new(
            1,
            MenuItemKind::SubmenuTrigger,
            Some("More".into()),
            false,
            false,
        ),
    ]);
    runtime.note_submenu_opened(1);
    runtime.move_highlight(MenuMove::Last, true);

    let state = runtime.submenu_trigger_state(Some(1), false);
    assert!(state.open);
    assert!(state.highlighted);

    let other = runtime.submenu_trigger_state(Some(0), false);
    assert!(!other.open);
}

#[test]
fn indicator_states_expose_keep_mounted_presence() {
    let runtime: MenuRuntime<()> = MenuRuntime::new(true, MenuParentKind::None);
    let unchecked_kept = runtime.checkbox_indicator_state(Some(0), false, false, true);
    assert!(unchecked_kept.present);
    assert!(!unchecked_kept.checked);

    let unchecked_unkept = runtime.checkbox_indicator_state(Some(0), false, false, false);
    assert!(!unchecked_unkept.present);

    let radio = runtime.radio_indicator_state(Some(0), false, true, false);
    assert!(radio.present);
    assert!(radio.checked);
}

#[test]
fn opening_press_marker_suppresses_exactly_one_click() {
    // Menubar seam: mouse-down opens the menu, so the mouse-up click of that
    // same press must not toggle it closed. The marker is consumed by the
    // first read and a press on an already-open trigger clears it.
    let mut runtime: MenuRuntime<()> = MenuRuntime::new(false, MenuParentKind::None);

    runtime.set_opened_by_current_press(true);
    assert!(runtime.take_opened_by_current_press());
    assert!(!runtime.take_opened_by_current_press());

    runtime.set_opened_by_current_press(true);
    runtime.set_opened_by_current_press(false);
    assert!(!runtime.take_opened_by_current_press());
}
