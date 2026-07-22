//! Hidden-axis and corner facts: `max_offset == 0` hides an axis, the
//! corner is hidden unless both axes overflow, and the corner size equals
//! the measured scrollbar thicknesses only while both axes overflow.

use gpui::{point, px, size, Size};

use crate::scroll_area::{ScrollAreaEdgeThreshold, ScrollAreaOrientation, ScrollAreaRuntime};

#[test]
fn zero_max_offset_hides_that_axis() {
    let mut runtime = ScrollAreaRuntime::new();
    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(0.0), px(100.0)),
        &ScrollAreaEdgeThreshold::default(),
    );

    assert!(runtime.axis_hidden(ScrollAreaOrientation::Horizontal));
    assert!(!runtime.axis_hidden(ScrollAreaOrientation::Vertical));
}

#[test]
fn corner_hidden_unless_both_axes_overflow() {
    let mut runtime = ScrollAreaRuntime::new();
    let threshold = ScrollAreaEdgeThreshold::default();

    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(0.0), px(100.0)),
        &threshold,
    );
    assert!(runtime.corner_hidden());

    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(100.0), px(100.0)),
        &threshold,
    );
    assert!(!runtime.corner_hidden());
}

#[test]
fn corner_size_equals_scrollbar_thicknesses_and_resets_without_overflow() {
    let mut runtime = ScrollAreaRuntime::new();
    let threshold = ScrollAreaEdgeThreshold::default();
    runtime.set_scrollbar_thickness(ScrollAreaOrientation::Vertical, px(10.0));
    runtime.set_scrollbar_thickness(ScrollAreaOrientation::Horizontal, px(14.0));

    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(100.0), px(100.0)),
        &threshold,
    );
    assert_eq!(runtime.corner_size(), size(px(10.0), px(14.0)));

    // Losing one axis's overflow resets the corner to zero.
    runtime.refresh_overflow(
        point(px(0.0), px(0.0)),
        point(px(0.0), px(100.0)),
        &threshold,
    );
    assert_eq!(runtime.corner_size(), Size::default());
}
