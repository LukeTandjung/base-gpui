//! Style-state structs report the full field sets from runtime facts:
//! shared root shape, scrollbar hover/orientation extension, thumb facts,
//! and corner facts.

use std::time::Instant;

use gpui::{point, px, size};

use crate::scroll_area::{ScrollAreaEdgeThreshold, ScrollAreaOrientation, ScrollAreaRuntime};

#[test]
fn root_state_reports_all_shared_facts() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(-50.0), px(0.0)),
        point(px(100.0), px(0.0)),
        &ScrollAreaEdgeThreshold::default(),
    );
    runtime.observe_scroll(point(px(-50.0), px(0.0)), Instant::now());

    let state = runtime.root_state();
    assert!(state.scrolling);
    assert!(state.has_overflow_x);
    assert!(!state.has_overflow_y);
    assert!(state.overflow_x_start);
    assert!(state.overflow_x_end);
    assert!(!state.overflow_y_start);
    assert!(!state.overflow_y_end);
    assert!(state.corner_hidden);
    assert_eq!(runtime.viewport_state(), state);
}

#[test]
fn hover_toggles_hovering_in_scrollbar_state() {
    let mut runtime = ScrollAreaRuntime::new();

    assert!(runtime.set_hovering(true));
    assert!(
        runtime
            .scrollbar_state(ScrollAreaOrientation::Vertical)
            .hovering
    );
    assert!(
        !runtime.set_hovering(true),
        "no change reported when already hovering"
    );

    assert!(runtime.set_hovering(false));
    assert!(
        !runtime
            .scrollbar_state(ScrollAreaOrientation::Vertical)
            .hovering
    );
}

#[test]
fn scrollbar_and_thumb_state_track_their_own_orientation() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(100.0), px(100.0)),
        &ScrollAreaEdgeThreshold::default(),
    );
    runtime.observe_scroll(point(px(0.0), px(-10.0)), Instant::now());

    let vertical = runtime.scrollbar_state(ScrollAreaOrientation::Vertical);
    let horizontal = runtime.scrollbar_state(ScrollAreaOrientation::Horizontal);
    assert!(vertical.scrolling);
    assert!(!horizontal.scrolling);
    assert_eq!(vertical.orientation, ScrollAreaOrientation::Vertical);
    assert!(vertical.has_overflow());
    assert!(horizontal.has_overflow());

    let thumb = runtime.thumb_state(ScrollAreaOrientation::Vertical);
    assert!(thumb.scrolling);
    assert_eq!(thumb.orientation, ScrollAreaOrientation::Vertical);
    assert!(
        !runtime
            .thumb_state(ScrollAreaOrientation::Horizontal)
            .scrolling
    );
}

#[test]
fn corner_state_reports_size_and_hidden_facts() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.set_scrollbar_thickness(ScrollAreaOrientation::Vertical, px(12.0));
    runtime.set_scrollbar_thickness(ScrollAreaOrientation::Horizontal, px(12.0));
    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(100.0), px(100.0)),
        &ScrollAreaEdgeThreshold::default(),
    );

    let corner = runtime.corner_state();
    assert!(!corner.hidden);
    assert_eq!(corner.size, size(px(12.0), px(12.0)));
}
