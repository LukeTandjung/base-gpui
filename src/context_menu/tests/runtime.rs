use std::time::Instant;

use gpui::{point, px, size};

use crate::menu::{MenuContextMenuMouseUp, MenuParentKind, MenuRuntime, CONTEXT_MENU_GRACE};

fn context_menu_runtime() -> MenuRuntime<()> {
    MenuRuntime::new(false, MenuParentKind::ContextMenu)
}

#[test]
fn open_at_cursor_records_anchor_and_initial_point() {
    let mut runtime = context_menu_runtime();
    let now = Instant::now();
    let cursor = point(px(120.0), px(80.0));

    runtime.open_context_menu_at(cursor, now);

    assert_eq!(runtime.context_menu_anchor(), Some(cursor));
    assert_eq!(runtime.context_menu_initial_point(), Some(cursor));
    assert!(runtime.context_menu_grace_active(now));
}

#[test]
fn reopening_at_a_new_point_re_anchors() {
    let mut runtime = context_menu_runtime();
    let now = Instant::now();
    runtime.open_context_menu_at(point(px(10.0), px(10.0)), now);
    let second = point(px(300.0), px(200.0));

    runtime.open_context_menu_at(second, now);

    assert_eq!(runtime.context_menu_anchor(), Some(second));
    assert_eq!(runtime.context_menu_initial_point(), Some(second));
}

#[test]
fn trigger_bounds_are_a_zero_size_rect_at_the_cursor() {
    let mut runtime = context_menu_runtime();
    let cursor = point(px(42.0), px(24.0));
    runtime.open_context_menu_at(cursor, Instant::now());

    let bounds = runtime.trigger_bounds().unwrap();
    assert_eq!(bounds.origin, cursor);
    assert_eq!(bounds.size, size(px(0.0), px(0.0)));
}

#[test]
fn mouse_up_before_grace_deadline_is_inert() {
    let mut runtime = context_menu_runtime();
    let now = Instant::now();
    runtime.open_context_menu_at(point(px(50.0), px(50.0)), now);

    let outcome = runtime.context_menu_mouse_up(point(px(500.0), px(500.0)), false, now);
    assert_eq!(outcome, MenuContextMenuMouseUp::InertGrace);
}

#[test]
fn mouse_up_outside_tree_after_grace_deadline_cancels_open() {
    let mut runtime = context_menu_runtime();
    let now = Instant::now();
    runtime.open_context_menu_at(point(px(50.0), px(50.0)), now);
    let later = now + CONTEXT_MENU_GRACE + CONTEXT_MENU_GRACE;

    let outcome = runtime.context_menu_mouse_up(point(px(500.0), px(500.0)), false, later);
    assert_eq!(outcome, MenuContextMenuMouseUp::CloseCancelOpen);
}

#[test]
fn mouse_up_inside_tree_never_cancels_open() {
    let mut runtime = context_menu_runtime();
    let now = Instant::now();
    runtime.open_context_menu_at(point(px(50.0), px(50.0)), now);
    let later = now + CONTEXT_MENU_GRACE + CONTEXT_MENU_GRACE;

    let outcome = runtime.context_menu_mouse_up(point(px(500.0), px(500.0)), true, later);
    assert_eq!(outcome, MenuContextMenuMouseUp::InsideTree);
}

#[test]
fn mouse_up_near_initial_point_is_suppressed_and_consumed() {
    let mut runtime = context_menu_runtime();
    let cursor = point(px(100.0), px(100.0));
    runtime.open_context_menu_at(cursor, Instant::now());

    // Within ±1px: suppressed and consumed.
    assert!(runtime.consume_context_menu_initial_point(point(px(100.5), px(99.5))));
    // Consumed after the first check: the same point no longer suppresses.
    assert!(!runtime.consume_context_menu_initial_point(point(px(100.5), px(99.5))));
}

#[test]
fn mouse_up_beyond_tolerance_enables_drag_release_activation() {
    let mut runtime = context_menu_runtime();
    let cursor = point(px(100.0), px(100.0));
    runtime.open_context_menu_at(cursor, Instant::now());

    assert!(!runtime.consume_context_menu_initial_point(point(px(110.0), px(100.0))));
    // The point is consumed either way.
    assert_eq!(runtime.context_menu_initial_point(), None);
}

#[test]
fn grace_mouse_up_near_initial_point_reports_suppression() {
    let mut runtime = context_menu_runtime();
    let cursor = point(px(100.0), px(100.0));
    let now = Instant::now();
    runtime.open_context_menu_at(cursor, now);

    let outcome = runtime.context_menu_mouse_up(point(px(100.0), px(101.0)), true, now);
    assert_eq!(outcome, MenuContextMenuMouseUp::SuppressedInitialPoint);
}

#[test]
fn closing_clears_grace_state_and_initial_point() {
    let mut runtime = context_menu_runtime();
    let now = Instant::now();
    runtime.open_context_menu_at(point(px(50.0), px(50.0)), now);

    runtime.commit_open(false, false, true);

    assert!(!runtime.context_menu_grace_active(now));
    assert_eq!(runtime.context_menu_initial_point(), None);

    // A subsequent open re-arms both deterministically.
    let reopened_at = now + CONTEXT_MENU_GRACE;
    runtime.open_context_menu_at(point(px(70.0), px(70.0)), reopened_at);
    assert!(runtime.context_menu_grace_active(reopened_at));
    assert_eq!(
        runtime.context_menu_initial_point(),
        Some(point(px(70.0), px(70.0)))
    );
}

#[test]
fn parent_kind_is_context_menu_for_the_whole_runtime_lifetime() {
    let runtime = context_menu_runtime();
    assert_eq!(runtime.parent_kind(), MenuParentKind::ContextMenu);
}
