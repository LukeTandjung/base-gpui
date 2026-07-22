//! Overflow-edge flags with the default (zero) threshold: at start only
//! `*_end` is set, at end only `*_start`, mid-scroll both, no-overflow
//! neither.

use gpui::{point, px};

use crate::scroll_area::{ScrollAreaEdgeThreshold, ScrollAreaRuntime};

fn threshold() -> ScrollAreaEdgeThreshold {
    ScrollAreaEdgeThreshold::default()
}

#[test]
fn at_start_only_end_flags_are_set() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(100.0), px(100.0)),
        &threshold(),
    );

    let state = runtime.root_state();
    assert!(!state.overflow_x_start);
    assert!(state.overflow_x_end);
    assert!(!state.overflow_y_start);
    assert!(state.overflow_y_end);
}

#[test]
fn at_end_only_start_flags_are_set() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(-100.0), px(-100.0)),
        point(px(100.0), px(100.0)),
        &threshold(),
    );

    let state = runtime.root_state();
    assert!(state.overflow_x_start);
    assert!(!state.overflow_x_end);
    assert!(state.overflow_y_start);
    assert!(!state.overflow_y_end);
}

#[test]
fn mid_scroll_sets_both_flags() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(-50.0), px(-50.0)),
        point(px(100.0), px(100.0)),
        &threshold(),
    );

    let state = runtime.root_state();
    assert!(state.overflow_x_start && state.overflow_x_end);
    assert!(state.overflow_y_start && state.overflow_y_end);
}

#[test]
fn no_overflow_sets_no_edge_flags() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(0.0), px(0.0)),
        &threshold(),
    );

    let state = runtime.root_state();
    assert!(!state.overflow_x_start && !state.overflow_x_end);
    assert!(!state.overflow_y_start && !state.overflow_y_end);
}
