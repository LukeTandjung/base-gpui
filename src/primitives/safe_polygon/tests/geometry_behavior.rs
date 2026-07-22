use gpui::px;

use crate::primitives::safe_polygon::{
    point_in_quadrilateral, point_in_trough, safe_polygon_quadrilateral, SafePolygonSide,
};

use super::support::{pt, rect};

const BUFFER: gpui::Pixels = px(0.5);

#[test]
fn quadrilateral_containment_side_right() {
    // Trigger 100x40, taller popup to the right with a 20px gap.
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(220.0, 100.0, 100.0, 300.0);
    let quad = safe_polygon_quadrilateral(
        pt(200.0, 120.0),
        trigger,
        popup,
        SafePolygonSide::Right,
        BUFFER,
    );

    assert!(point_in_quadrilateral(pt(210.0, 190.0), quad));
    assert!(!point_in_quadrilateral(pt(150.0, 120.0), quad));
    assert!(!point_in_quadrilateral(pt(210.0, 50.0), quad));
}

#[test]
fn quadrilateral_containment_side_left() {
    let trigger = rect(220.0, 100.0, 100.0, 40.0);
    let popup = rect(100.0, 100.0, 100.0, 300.0);
    let quad = safe_polygon_quadrilateral(
        pt(220.0, 120.0),
        trigger,
        popup,
        SafePolygonSide::Left,
        BUFFER,
    );

    assert!(point_in_quadrilateral(pt(210.0, 190.0), quad));
    assert!(!point_in_quadrilateral(pt(270.0, 120.0), quad));
    assert!(!point_in_quadrilateral(pt(210.0, 50.0), quad));
}

#[test]
fn quadrilateral_containment_side_top() {
    // Wider popup above the trigger.
    let trigger = rect(100.0, 220.0, 100.0, 40.0);
    let popup = rect(0.0, 100.0, 300.0, 100.0);
    let quad = safe_polygon_quadrilateral(
        pt(150.0, 220.0),
        trigger,
        popup,
        SafePolygonSide::Top,
        BUFFER,
    );

    assert!(point_in_quadrilateral(pt(150.0, 210.0), quad));
    assert!(!point_in_quadrilateral(pt(10.0, 215.0), quad));
    assert!(!point_in_quadrilateral(pt(150.0, 240.0), quad));
}

#[test]
fn quadrilateral_containment_side_bottom() {
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(0.0, 160.0, 300.0, 100.0);
    let quad = safe_polygon_quadrilateral(
        pt(150.0, 140.0),
        trigger,
        popup,
        SafePolygonSide::Bottom,
        BUFFER,
    );

    assert!(point_in_quadrilateral(pt(150.0, 150.0), quad));
    assert!(!point_in_quadrilateral(pt(10.0, 150.0), quad));
    assert!(!point_in_quadrilateral(pt(150.0, 90.0), quad));
}

#[test]
fn narrower_popup_corner_selection_side_bottom() {
    // Popup narrower than the trigger; exit from the right half sweeps the
    // quadrilateral across the popup's near-edge diagonal.
    let trigger = rect(100.0, 100.0, 200.0, 40.0);
    let popup = rect(150.0, 160.0, 100.0, 100.0);
    let quad = safe_polygon_quadrilateral(
        pt(280.0, 140.0),
        trigger,
        popup,
        SafePolygonSide::Bottom,
        BUFFER,
    );

    assert!(point_in_quadrilateral(pt(250.0, 180.0), quad));
    assert!(!point_in_quadrilateral(pt(150.0, 220.0), quad));
}

#[test]
fn trough_side_right() {
    let trigger = rect(100.0, 100.0, 100.0, 100.0);
    let popup = rect(220.0, 100.0, 100.0, 100.0);
    let side = SafePolygonSide::Right;

    assert!(point_in_trough(pt(210.0, 150.0), trigger, popup, side));
    // 1px insets into each box.
    assert!(point_in_trough(pt(199.0, 150.0), trigger, popup, side));
    assert!(point_in_trough(pt(221.0, 150.0), trigger, popup, side));
    assert!(!point_in_trough(pt(198.0, 150.0), trigger, popup, side));
    assert!(!point_in_trough(pt(222.0, 150.0), trigger, popup, side));
    assert!(!point_in_trough(pt(210.0, 250.0), trigger, popup, side));
}

#[test]
fn trough_side_left() {
    let trigger = rect(100.0, 100.0, 100.0, 100.0);
    let popup = rect(-20.0, 100.0, 100.0, 100.0);
    let side = SafePolygonSide::Left;

    assert!(point_in_trough(pt(90.0, 150.0), trigger, popup, side));
    assert!(point_in_trough(pt(79.0, 150.0), trigger, popup, side));
    assert!(point_in_trough(pt(101.0, 150.0), trigger, popup, side));
    assert!(!point_in_trough(pt(78.0, 150.0), trigger, popup, side));
    assert!(!point_in_trough(pt(102.0, 150.0), trigger, popup, side));
}

#[test]
fn trough_side_top() {
    let trigger = rect(100.0, 100.0, 100.0, 100.0);
    let popup = rect(100.0, -20.0, 100.0, 100.0);
    let side = SafePolygonSide::Top;

    assert!(point_in_trough(pt(150.0, 90.0), trigger, popup, side));
    assert!(point_in_trough(pt(150.0, 79.0), trigger, popup, side));
    assert!(point_in_trough(pt(150.0, 101.0), trigger, popup, side));
    assert!(!point_in_trough(pt(150.0, 78.0), trigger, popup, side));
    assert!(!point_in_trough(pt(150.0, 102.0), trigger, popup, side));
}

#[test]
fn trough_side_bottom() {
    let trigger = rect(100.0, 100.0, 100.0, 100.0);
    let popup = rect(100.0, 220.0, 100.0, 100.0);
    let side = SafePolygonSide::Bottom;

    assert!(point_in_trough(pt(150.0, 210.0), trigger, popup, side));
    assert!(point_in_trough(pt(150.0, 199.0), trigger, popup, side));
    assert!(point_in_trough(pt(150.0, 221.0), trigger, popup, side));
    assert!(!point_in_trough(pt(150.0, 198.0), trigger, popup, side));
    assert!(!point_in_trough(pt(150.0, 222.0), trigger, popup, side));
}

#[test]
fn trough_clamps_to_narrower_box_on_cross_axis() {
    // Wider popup below: the trough's x-range is the (narrower) trigger's.
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(0.0, 160.0, 300.0, 100.0);
    let side = SafePolygonSide::Bottom;

    assert!(point_in_trough(pt(150.0, 150.0), trigger, popup, side));
    assert!(!point_in_trough(pt(50.0, 150.0), trigger, popup, side));
}
