//! The Viewport is a tab stop only while at least one axis is scrollable.

use gpui::{point, px};

use crate::scroll_area::{ScrollAreaEdgeThreshold, ScrollAreaRuntime};

#[test]
fn viewport_focusable_only_while_scrollable() {
    let mut runtime = ScrollAreaRuntime::new();
    let threshold = ScrollAreaEdgeThreshold::default();
    let offset = point(px(0.0), px(0.0));

    assert!(
        !runtime.viewport_focusable(),
        "unmeasured area is not a tab stop"
    );

    runtime.refresh_overflow(offset, point(px(0.0), px(0.0)), &threshold);
    assert!(!runtime.viewport_focusable());

    runtime.refresh_overflow(offset, point(px(0.0), px(100.0)), &threshold);
    assert!(runtime.viewport_focusable());

    runtime.refresh_overflow(offset, point(px(0.0), px(0.0)), &threshold);
    assert!(!runtime.viewport_focusable());
}
