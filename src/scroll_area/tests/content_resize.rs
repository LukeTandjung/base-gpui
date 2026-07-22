//! Content growth/shrink (a `max_offset` change) updates overflow, edge
//! flags, and mount state through `refresh_overflow` without any scroll
//! event, and the refresh reports whether anything changed.

use gpui::{point, px};

use crate::scroll_area::{ScrollAreaEdgeThreshold, ScrollAreaOrientation, ScrollAreaRuntime};

#[test]
fn content_growth_updates_overflow_without_scrolling() {
    let mut runtime = ScrollAreaRuntime::new();
    let threshold = ScrollAreaEdgeThreshold::default();
    let offset = point(px(0.0), px(0.0));

    let changed = runtime.refresh_overflow(offset, point(px(0.0), px(0.0)), &threshold);
    assert!(changed, "first measurement counts as a change");
    assert!(runtime.axis_hidden(ScrollAreaOrientation::Vertical));

    // Content grows to overflow vertically: the axis mounts and its end
    // edge flag appears, with no scroll observed.
    let changed = runtime.refresh_overflow(offset, point(px(0.0), px(300.0)), &threshold);
    assert!(changed);
    assert!(!runtime.axis_hidden(ScrollAreaOrientation::Vertical));
    assert!(runtime.root_state().overflow_y_end);
    assert!(!runtime.scrolling(ScrollAreaOrientation::Vertical));

    // A refresh with identical facts reports no change, so parts only
    // notify when needed.
    let changed = runtime.refresh_overflow(offset, point(px(0.0), px(300.0)), &threshold);
    assert!(!changed);
}
