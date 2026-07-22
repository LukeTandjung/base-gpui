use crate::menu::MenuOpenChangeReason;
use crate::menubar::{
    MenubarMove, MenubarOrientation, MenubarProps, MenubarRuntime, MenubarTriggerSlot,
};

fn runtime_with(count: usize) -> MenubarRuntime {
    let mut runtime = MenubarRuntime::new();
    runtime.sync_triggers((0..count).map(|_| MenubarTriggerSlot::new(false)).collect());
    runtime.reconcile();
    runtime
}

#[test]
fn initial_tab_stop_is_the_first_trigger() {
    let runtime = runtime_with(3);
    assert!(runtime.is_tab_stop(0));
    assert!(!runtime.is_tab_stop(1));
}

#[test]
fn disabled_first_trigger_stays_keyboard_reachable() {
    let mut runtime = MenubarRuntime::new();
    runtime.sync_triggers(vec![
        MenubarTriggerSlot::new(true),
        MenubarTriggerSlot::new(false),
    ]);
    runtime.reconcile();
    // Composite roving includes disabled triggers: the row stays reachable
    // through its first (disabled) trigger.
    assert!(runtime.is_tab_stop(0));
    assert!(runtime.trigger_disabled(0));
}

#[test]
fn move_highlight_wraps_with_loop_and_clamps_without() {
    let mut runtime = runtime_with(3);
    runtime.move_highlight(MenubarMove::Last, true);
    assert!(runtime.is_tab_stop(2));

    runtime.move_highlight(MenubarMove::Next, true);
    assert!(runtime.is_tab_stop(0));

    runtime.move_highlight(MenubarMove::Previous, true);
    assert!(runtime.is_tab_stop(2));

    runtime.move_highlight(MenubarMove::Next, false);
    assert!(runtime.is_tab_stop(2));

    runtime.move_highlight(MenubarMove::First, false);
    runtime.move_highlight(MenubarMove::Previous, false);
    assert!(runtime.is_tab_stop(0));
}

#[test]
fn home_and_end_jump_to_the_row_ends() {
    let mut runtime = runtime_with(4);
    runtime.move_highlight(MenubarMove::Last, false);
    assert!(runtime.is_tab_stop(3));
    runtime.move_highlight(MenubarMove::First, false);
    assert!(runtime.is_tab_stop(0));
}

#[test]
fn has_submenu_open_sets_on_open_and_follows_the_open_menu() {
    let mut runtime = runtime_with(3);
    assert!(!runtime.has_submenu_open());

    runtime.note_child_open_change(1, true, MenuOpenChangeReason::TriggerPress);
    assert!(runtime.has_submenu_open());
    assert_eq!(runtime.open_menu_index(), Some(1));
    // The roving highlight follows the open menu.
    assert!(runtime.is_tab_stop(1));
}

#[test]
fn has_submenu_open_is_retained_across_handoff_closes() {
    let mut runtime = runtime_with(3);
    runtime.note_child_open_change(0, true, MenuOpenChangeReason::TriggerPress);

    runtime.note_child_open_change(0, false, MenuOpenChangeReason::SiblingOpen);
    assert!(runtime.has_submenu_open());
    assert_eq!(runtime.open_menu_index(), None);

    runtime.note_child_open_change(1, true, MenuOpenChangeReason::TriggerHover);
    runtime.note_child_open_change(1, false, MenuOpenChangeReason::ListNavigation);
    assert!(runtime.has_submenu_open());
}

#[test]
fn has_submenu_open_clears_on_dismiss_closes() {
    for reason in [
        MenuOpenChangeReason::OutsidePress,
        MenuOpenChangeReason::EscapeKey,
        MenuOpenChangeReason::ItemPress,
        MenuOpenChangeReason::TriggerPress,
    ] {
        let mut runtime = runtime_with(2);
        runtime.note_child_open_change(0, true, MenuOpenChangeReason::TriggerPress);
        runtime.note_child_open_change(0, false, reason);
        assert!(!runtime.has_submenu_open(), "reason {reason:?}");
        assert_eq!(runtime.open_menu_index(), None);
    }
}

#[test]
fn stale_close_from_a_non_open_menu_keeps_the_open_menu_identity() {
    let mut runtime = runtime_with(3);
    runtime.note_child_open_change(0, true, MenuOpenChangeReason::TriggerPress);
    runtime.note_child_open_change(2, true, MenuOpenChangeReason::TriggerHover);
    // Menu 0's late close (its handoff close) must not clear menu 2.
    runtime.note_child_open_change(0, false, MenuOpenChangeReason::SiblingOpen);
    assert_eq!(runtime.open_menu_index(), Some(2));
    assert!(runtime.has_submenu_open());
}

#[test]
fn sync_triggers_preserves_registered_commands_and_reconciles_range() {
    let mut runtime = runtime_with(3);
    runtime.note_child_open_change(2, true, MenuOpenChangeReason::TriggerPress);

    // Shrinking the row drops the out-of-range open menu and highlight.
    runtime.sync_triggers(vec![MenubarTriggerSlot::new(false)]);
    runtime.reconcile();
    assert_eq!(runtime.open_menu_index(), None);
    assert!(!runtime.has_submenu_open());
    assert!(runtime.is_tab_stop(0));
}

#[test]
fn set_highlight_ignores_out_of_range_indices() {
    let mut runtime = runtime_with(2);
    runtime.set_highlight(5);
    assert!(runtime.is_tab_stop(0));
    runtime.set_highlight(1);
    assert!(runtime.is_tab_stop(1));
}

#[test]
fn hand_off_bookkeeping_exposes_the_open_siblings_close_command() {
    use crate::menu::MenuMenubarOpenFn;

    let mut runtime = runtime_with(2);
    let noop: MenuMenubarOpenFn = std::rc::Rc::new(|_, _, _, _| {});
    runtime.register_trigger(0, false, None, noop.clone(), noop.clone());
    runtime.register_trigger(1, false, None, noop.clone(), noop);

    // Nothing open: no sibling to close, but the target open command exists.
    assert!(runtime.open_sibling_close_command(1).is_none());
    assert!(runtime.open_command(1).is_some());

    runtime.note_child_open_change(0, true, MenuOpenChangeReason::TriggerPress);
    // Handing off to trigger 1 must close menu 0, never itself.
    assert!(runtime.open_sibling_close_command(1).is_some());
    assert!(runtime.open_sibling_close_command(0).is_none());

    // Once settled, at most one child menu is recorded open.
    runtime.note_child_open_change(0, false, MenuOpenChangeReason::ListNavigation);
    runtime.note_child_open_change(1, true, MenuOpenChangeReason::ListNavigation);
    assert_eq!(runtime.open_menu_index(), Some(1));
    assert!(runtime.has_submenu_open());
}

#[test]
fn relay_moves_from_the_relaying_trigger_with_wrap_and_clamp() {
    let mut runtime = runtime_with(3);

    // The relay re-anchors at the relaying menu's trigger before moving.
    runtime.set_highlight(1);
    runtime.move_highlight(MenubarMove::Next, true);
    assert!(runtime.is_tab_stop(2));

    runtime.set_highlight(2);
    runtime.move_highlight(MenubarMove::Next, true);
    assert!(runtime.is_tab_stop(0));

    runtime.set_highlight(2);
    runtime.move_highlight(MenubarMove::Next, false);
    assert!(runtime.is_tab_stop(2));
}

#[test]
fn allow_mouse_up_trigger_window_is_shared_state() {
    let mut runtime = runtime_with(2);
    assert!(!runtime.allow_mouse_up_trigger());
    runtime.set_allow_mouse_up_trigger(true);
    assert!(runtime.allow_mouse_up_trigger());
}

#[test]
fn root_state_reports_the_base_ui_facts() {
    let mut runtime = runtime_with(2);
    let props = MenubarProps::new(MenubarOrientation::Vertical, true, false, true);
    runtime.note_child_open_change(0, true, MenuOpenChangeReason::TriggerPress);

    let state = runtime.root_state(&props);
    assert_eq!(state.orientation, MenubarOrientation::Vertical);
    assert!(!state.modal);
    assert!(state.has_submenu_open);
    assert!(state.disabled);
}
