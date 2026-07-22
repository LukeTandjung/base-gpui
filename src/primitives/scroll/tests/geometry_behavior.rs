use gpui::{point, px, Axis};

use crate::primitives::scroll::{
    axis_geometry, corner_size, drag_scroll_position, horizontal_margin_end,
    scroll_offset_for_axis, track_click_scroll_position, MIN_THUMB_SIZE,
};

#[test]
fn thumb_length_is_proportional_to_viewport_content_ratio() {
    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), px(0.0))
        .expect("overflowing content should produce geometry");
    assert_eq!(geometry.thumb_len, px(100.0));
}

#[test]
fn thumb_length_clamps_at_min_thumb_size_for_very_long_content() {
    let geometry = axis_geometry(px(100000.0), px(200.0), px(0.0), px(200.0), px(0.0))
        .expect("overflowing content should produce geometry");
    assert_eq!(geometry.thumb_len, MIN_THUMB_SIZE);
}

#[test]
fn thumb_sits_at_track_start_when_scrolled_to_content_start() {
    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), px(0.0))
        .expect("overflowing content should produce geometry");
    assert_eq!(geometry.thumb_offset, px(0.0));
}

#[test]
fn thumb_sits_at_track_end_when_scrolled_to_content_end() {
    let geometry = axis_geometry(px(400.0), px(200.0), px(-200.0), px(200.0), px(0.0))
        .expect("overflowing content should produce geometry");
    assert_eq!(geometry.thumb_offset + geometry.thumb_len, px(200.0));
}

#[test]
fn clamped_thumb_still_reaches_both_track_ends() {
    let at_start = axis_geometry(px(100000.0), px(200.0), px(0.0), px(200.0), px(0.0))
        .expect("overflowing content should produce geometry");
    assert_eq!(at_start.thumb_offset, px(0.0));

    let at_end = axis_geometry(px(100000.0), px(200.0), px(-99800.0), px(200.0), px(0.0))
        .expect("overflowing content should produce geometry");
    assert_eq!(at_end.thumb_offset + at_end.thumb_len, px(200.0));
}

#[test]
fn no_overflow_yields_no_geometry() {
    assert!(axis_geometry(px(150.0), px(200.0), px(0.0), px(200.0), px(0.0)).is_none());
    assert!(axis_geometry(px(200.0), px(200.0), px(0.0), px(200.0), px(0.0)).is_none());
}

#[test]
fn drag_delta_maps_to_expected_scroll_offset() {
    // Content 400, viewport/track 200 → thumb 100, draggable range 100,
    // scroll range 200: pointer travel maps 2x into scroll offset.
    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), px(0.0)).unwrap();
    let position =
        drag_scroll_position(&geometry, px(60.0), px(10.0), px(0.0), px(400.0), px(200.0));
    assert_eq!(position, px(-100.0));
}

#[test]
fn drag_clamps_at_both_extremes() {
    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), px(0.0)).unwrap();
    let before_start = drag_scroll_position(
        &geometry,
        px(-500.0),
        px(0.0),
        px(0.0),
        px(400.0),
        px(200.0),
    );
    assert_eq!(before_start, px(0.0));
    let past_end = drag_scroll_position(
        &geometry,
        px(5000.0),
        px(0.0),
        px(0.0),
        px(400.0),
        px(200.0),
    );
    assert_eq!(past_end, px(-200.0));
}

#[test]
fn drag_preserves_cross_axis_offset() {
    let current = point(px(-40.0), px(-10.0));
    let vertical = scroll_offset_for_axis(Axis::Vertical, px(-80.0), current);
    assert_eq!(vertical, point(px(-40.0), px(-80.0)));
    let horizontal = scroll_offset_for_axis(Axis::Horizontal, px(-80.0), current);
    assert_eq!(horizontal, point(px(-80.0), px(-10.0)));
}

#[test]
fn track_click_centers_thumb_on_click_position() {
    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), px(0.0)).unwrap();
    // Click at track center (100): thumb (100 long) centers → thumb offset
    // 50 → scroll ratio 0.5 → offset -100.
    let position = track_click_scroll_position(&geometry, px(100.0), px(0.0), px(400.0), px(200.0));
    assert_eq!(position, px(-100.0));
}

#[test]
fn track_click_clamps_at_the_ends() {
    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), px(0.0)).unwrap();
    let start = track_click_scroll_position(&geometry, px(0.0), px(0.0), px(400.0), px(200.0));
    assert_eq!(start, px(0.0));
    let end = track_click_scroll_position(&geometry, px(200.0), px(0.0), px(400.0), px(200.0));
    assert_eq!(end, px(-200.0));
}

#[test]
fn both_axes_reserve_horizontal_end_margin_and_define_corner() {
    let thickness = px(12.0);
    assert_eq!(horizontal_margin_end(true, thickness), thickness);
    assert_eq!(horizontal_margin_end(false, thickness), px(0.0));

    let geometry = axis_geometry(px(400.0), px(200.0), px(0.0), px(200.0), thickness)
        .expect("overflowing content should produce geometry");
    assert_eq!(geometry.usable_track_len(), px(188.0));
    // The thumb never enters the reserved margin, even at scroll end.
    let at_end = axis_geometry(px(400.0), px(200.0), px(-200.0), px(200.0), thickness).unwrap();
    assert_eq!(at_end.thumb_offset + at_end.thumb_len, px(188.0));

    let corner = corner_size(thickness, px(10.0));
    assert_eq!(corner.width, thickness);
    assert_eq!(corner.height, px(10.0));
}
