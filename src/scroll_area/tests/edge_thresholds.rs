//! Non-zero edge thresholds: flags stay false until the scrolled/remaining
//! distance exceeds the threshold; negative inputs clamp to zero; a uniform
//! value applies to all four edges.

use gpui::{point, px};

use crate::scroll_area::{ScrollAreaEdgeThreshold, ScrollAreaRuntime};

#[test]
fn edge_flag_stays_false_until_distance_exceeds_threshold() {
    let mut runtime = ScrollAreaRuntime::new();
    let threshold = ScrollAreaEdgeThreshold::uniform(px(20.0));

    // Scrolled 20 from start: not past the 20px start threshold yet.
    runtime.refresh_overflow(
        point(px(0.0), px(-20.0)),
        point(px(0.0), px(100.0)),
        &threshold,
    );
    assert!(!runtime.root_state().overflow_y_start);

    // Scrolled 21 from start: past it.
    runtime.refresh_overflow(
        point(px(0.0), px(-21.0)),
        point(px(0.0), px(100.0)),
        &threshold,
    );
    assert!(runtime.root_state().overflow_y_start);

    // 20 remaining to the end: not past the end threshold.
    runtime.refresh_overflow(
        point(px(0.0), px(-80.0)),
        point(px(0.0), px(100.0)),
        &threshold,
    );
    assert!(!runtime.root_state().overflow_y_end);
}

#[test]
fn negative_threshold_inputs_clamp_to_zero() {
    let threshold = ScrollAreaEdgeThreshold::new(px(-5.0), px(-1.0), px(-10.0), px(-0.5));
    assert_eq!(threshold.x_start, px(0.0));
    assert_eq!(threshold.x_end, px(0.0));
    assert_eq!(threshold.y_start, px(0.0));
    assert_eq!(threshold.y_end, px(0.0));

    let uniform = ScrollAreaEdgeThreshold::uniform(px(-7.0));
    assert_eq!(uniform, ScrollAreaEdgeThreshold::default());
}

#[test]
fn uniform_value_applies_to_all_four_edges() {
    let threshold: ScrollAreaEdgeThreshold = px(8.0).into();
    assert_eq!(threshold.x_start, px(8.0));
    assert_eq!(threshold.x_end, px(8.0));
    assert_eq!(threshold.y_start, px(8.0));
    assert_eq!(threshold.y_end, px(8.0));
}
