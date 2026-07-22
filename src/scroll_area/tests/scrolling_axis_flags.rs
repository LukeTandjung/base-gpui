//! Observing an offset change on one axis marks only that axis as
//! scrolling.

use std::time::Instant;

use gpui::{point, px};

use crate::scroll_area::{ScrollAreaOrientation, ScrollAreaRuntime};

#[test]
fn vertical_offset_change_sets_scrolling_y_only() {
    let mut runtime = ScrollAreaRuntime::new();
    let now = Instant::now();

    let changed = runtime.observe_scroll(point(px(0.0), px(-10.0)), now);

    assert!(changed);
    assert!(runtime.scrolling(ScrollAreaOrientation::Vertical));
    assert!(!runtime.scrolling(ScrollAreaOrientation::Horizontal));
}

#[test]
fn horizontal_offset_change_sets_scrolling_x_only() {
    let mut runtime = ScrollAreaRuntime::new();
    let now = Instant::now();

    let changed = runtime.observe_scroll(point(px(-10.0), px(0.0)), now);

    assert!(changed);
    assert!(runtime.scrolling(ScrollAreaOrientation::Horizontal));
    assert!(!runtime.scrolling(ScrollAreaOrientation::Vertical));
}

#[test]
fn unchanged_offset_is_not_scroll_activity() {
    let mut runtime = ScrollAreaRuntime::new();
    let now = Instant::now();

    let changed = runtime.observe_scroll(point(px(0.0), px(0.0)), now);

    assert!(!changed);
    assert!(!runtime.scrolling(ScrollAreaOrientation::Vertical));
    assert!(!runtime.scrolling(ScrollAreaOrientation::Horizontal));
}
