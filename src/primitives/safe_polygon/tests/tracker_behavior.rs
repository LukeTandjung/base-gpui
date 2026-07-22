use crate::primitives::safe_polygon::{
    SafePolygon, SafePolygonConfig, SafePolygonSide, SafePolygonVerdict,
};

use super::support::{ms, pt, rect};

fn tracker() -> SafePolygon {
    SafePolygon::new(SafePolygonConfig::default())
}

fn square_trigger() -> gpui::Bounds<gpui::Pixels> {
    rect(100.0, 100.0, 100.0, 100.0)
}

#[test]
fn unarmed_evaluate_is_outside_and_stable() {
    let mut tracker = tracker();
    assert!(!tracker.is_armed());
    assert_eq!(
        tracker.evaluate(pt(150.0, 150.0), ms(0)),
        SafePolygonVerdict::Outside
    );
    assert_eq!(
        tracker.evaluate(pt(150.0, 150.0), ms(10)),
        SafePolygonVerdict::Outside
    );
    assert!(!tracker.is_armed());
}

#[test]
fn opposite_side_exit_is_outside_for_all_sides() {
    let trigger = square_trigger();
    let cases = [
        // (side, popup, exit beyond the trigger's far edge, probe in the gap)
        (
            SafePolygonSide::Right,
            rect(220.0, 100.0, 100.0, 100.0),
            pt(100.0, 150.0),
            pt(210.0, 150.0),
        ),
        (
            SafePolygonSide::Left,
            rect(-20.0, 100.0, 100.0, 100.0),
            pt(200.0, 150.0),
            pt(90.0, 150.0),
        ),
        (
            SafePolygonSide::Top,
            rect(100.0, -20.0, 100.0, 100.0),
            pt(150.0, 200.0),
            pt(150.0, 90.0),
        ),
        (
            SafePolygonSide::Bottom,
            rect(100.0, 220.0, 100.0, 100.0),
            pt(150.0, 100.0),
            pt(150.0, 210.0),
        ),
    ];
    for (side, popup, exit, probe) in cases {
        let mut tracker = tracker();
        tracker.arm(exit, trigger, popup, side);
        assert_eq!(
            tracker.evaluate(probe, ms(5)),
            SafePolygonVerdict::Outside,
            "side {side:?}"
        );
    }
}

#[test]
fn diagonal_traversal_stays_inside_for_all_sides() {
    let trigger = square_trigger();
    // Exit at the near-edge center, heading to a near corner of the popup;
    // sample interior points of the straight path at high speed.
    let cases = [
        (
            SafePolygonSide::Right,
            rect(220.0, 100.0, 100.0, 100.0),
            pt(200.0, 150.0),
            pt(220.0, 100.0),
        ),
        (
            SafePolygonSide::Left,
            rect(-20.0, 100.0, 100.0, 100.0),
            pt(100.0, 150.0),
            pt(80.0, 100.0),
        ),
        (
            SafePolygonSide::Top,
            rect(100.0, -20.0, 100.0, 100.0),
            pt(150.0, 100.0),
            pt(100.0, 80.0),
        ),
        (
            SafePolygonSide::Bottom,
            rect(100.0, 220.0, 100.0, 100.0),
            pt(150.0, 200.0),
            pt(200.0, 220.0),
        ),
    ];
    for (side, popup, exit, target) in cases {
        let mut tracker = tracker();
        tracker.arm(exit, trigger, popup, side);
        for (index, t) in [0.25_f32, 0.5, 0.75].into_iter().enumerate() {
            let sample = pt(
                f32::from(exit.x) + (f32::from(target.x) - f32::from(exit.x)) * t,
                f32::from(exit.y) + (f32::from(target.y) - f32::from(exit.y)) * t,
            );
            assert_eq!(
                tracker.evaluate(sample, ms(5 * (index as u64 + 1))),
                SafePolygonVerdict::Inside,
                "side {side:?} sample {sample:?}"
            );
        }
    }
}

#[test]
fn diagonal_popup_placement_covers_direct_path() {
    // Popup overlaps the trigger on neither axis; the side parameter picks
    // the governing axis and the region still covers the direct path.
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(250.0, 200.0, 100.0, 100.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 140.0), trigger, popup, SafePolygonSide::Right);

    for (index, sample) in [pt(212.5, 167.5), pt(225.0, 195.0), pt(237.5, 222.5)]
        .into_iter()
        .enumerate()
    {
        assert_eq!(
            tracker.evaluate(sample, ms(5 * (index as u64 + 1))),
            SafePolygonVerdict::Inside,
            "sample {sample:?}"
        );
    }
}

#[test]
fn perpendicular_exit_away_from_popup_is_outside() {
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(220.0, 100.0, 100.0, 300.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 120.0), trigger, popup, SafePolygonSide::Right);

    // Above the trigger, and far outside quad/trough/trigger/popup.
    assert_eq!(
        tracker.evaluate(pt(150.0, 60.0), ms(5)),
        SafePolygonVerdict::Outside
    );
    assert_eq!(
        tracker.evaluate(pt(20.0, 500.0), ms(10)),
        SafePolygonVerdict::Outside
    );
}

#[test]
fn slow_cursor_inside_polygon_is_outside_fast_is_inside() {
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(220.0, 100.0, 100.0, 300.0);

    // Points inside the quadrilateral but below the trough band (trigger is
    // only 40px tall, so the trough stops at y=141).
    let mut slow = tracker();
    slow.arm(pt(200.0, 120.0), trigger, popup, SafePolygonSide::Right);
    assert_eq!(
        slow.evaluate(pt(210.0, 190.0), ms(10)),
        SafePolygonVerdict::Inside,
        "first sample never fails the velocity check"
    );
    assert_eq!(
        slow.evaluate(pt(210.3, 190.3), ms(110)),
        SafePolygonVerdict::Outside,
        "sub-threshold speed loses the grace"
    );

    let mut fast = tracker();
    fast.arm(pt(200.0, 120.0), trigger, popup, SafePolygonSide::Right);
    assert_eq!(
        fast.evaluate(pt(210.0, 190.0), ms(10)),
        SafePolygonVerdict::Inside
    );
    assert_eq!(
        fast.evaluate(pt(215.0, 200.0), ms(20)),
        SafePolygonVerdict::Inside,
        "same trajectory at high speed keeps the grace"
    );
}

#[test]
fn trough_position_with_slow_cursor_is_inside() {
    let trigger = square_trigger();
    let popup = rect(220.0, 100.0, 100.0, 100.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 150.0), trigger, popup, SafePolygonSide::Right);

    assert_eq!(
        tracker.evaluate(pt(210.0, 150.0), ms(10)),
        SafePolygonVerdict::Inside
    );
    assert_eq!(
        tracker.evaluate(pt(210.2, 150.0), ms(500)),
        SafePolygonVerdict::Inside,
        "trough is safe without any velocity check"
    );
}

#[test]
fn landing_on_popup_disarms() {
    let trigger = square_trigger();
    let popup = rect(220.0, 100.0, 100.0, 100.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 150.0), trigger, popup, SafePolygonSide::Right);

    assert_eq!(
        tracker.evaluate(pt(250.0, 150.0), ms(5)),
        SafePolygonVerdict::LandedPopup
    );
    assert!(!tracker.is_armed());
    assert_eq!(
        tracker.evaluate(pt(250.0, 150.0), ms(10)),
        SafePolygonVerdict::Outside,
        "unarmed behavior after landing"
    );
}

#[test]
fn landing_on_trigger_disarms() {
    let trigger = square_trigger();
    let popup = rect(220.0, 100.0, 100.0, 100.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 150.0), trigger, popup, SafePolygonSide::Right);

    assert_eq!(
        tracker.evaluate(pt(150.0, 150.0), ms(5)),
        SafePolygonVerdict::LandedTrigger
    );
    assert!(!tracker.is_armed());
    assert_eq!(
        tracker.evaluate(pt(150.0, 150.0), ms(10)),
        SafePolygonVerdict::Outside
    );
}

#[test]
fn rearm_replaces_region_and_resets_velocity_history() {
    let trigger = rect(100.0, 100.0, 100.0, 40.0);
    let popup = rect(220.0, 100.0, 100.0, 300.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 120.0), trigger, popup, SafePolygonSide::Right);
    assert_eq!(
        tracker.evaluate(pt(210.0, 190.0), ms(10)),
        SafePolygonVerdict::Inside
    );
    assert_eq!(
        tracker.evaluate(pt(210.3, 190.3), ms(110)),
        SafePolygonVerdict::Outside,
        "slow sample with retained history"
    );

    // Re-arm: the same slow-looking sample is now a fresh first sample.
    tracker.arm(pt(200.0, 120.0), trigger, popup, SafePolygonSide::Right);
    assert_eq!(
        tracker.evaluate(pt(210.3, 190.3), ms(210)),
        SafePolygonVerdict::Inside,
        "velocity history reset by re-arm"
    );

    // Re-arm with a different popup replaces the region: a point inside the
    // old popup no longer lands.
    let square = square_trigger();
    let old_popup = rect(220.0, 100.0, 100.0, 100.0);
    let new_popup = rect(400.0, 100.0, 100.0, 100.0);
    let mut tracker = SafePolygon::new(SafePolygonConfig::default());
    tracker.arm(pt(200.0, 150.0), square, old_popup, SafePolygonSide::Right);
    tracker.arm(pt(200.0, 150.0), square, new_popup, SafePolygonSide::Right);
    let verdict = tracker.evaluate(pt(250.0, 150.0), ms(5));
    assert_ne!(verdict, SafePolygonVerdict::LandedPopup);
    assert_eq!(verdict, SafePolygonVerdict::Inside, "in the new trough");
}

#[test]
fn disarm_clears_state() {
    let trigger = square_trigger();
    let popup = rect(220.0, 100.0, 100.0, 100.0);
    let mut tracker = tracker();
    tracker.arm(pt(200.0, 150.0), trigger, popup, SafePolygonSide::Right);
    assert!(tracker.is_armed());
    tracker.disarm();
    assert!(!tracker.is_armed());
    assert_eq!(
        tracker.evaluate(pt(210.0, 150.0), ms(5)),
        SafePolygonVerdict::Outside
    );
}

#[test]
fn degenerate_inputs_do_not_panic() {
    let trigger = square_trigger();

    // Zero-size popup.
    let mut tracker = tracker();
    tracker.arm(
        pt(200.0, 120.0),
        trigger,
        rect(220.0, 120.0, 0.0, 0.0),
        SafePolygonSide::Right,
    );
    let _ = tracker.evaluate(pt(210.0, 120.0), ms(5));
    let _ = tracker.evaluate(pt(500.0, 500.0), ms(10));

    // Exit point already inside the popup.
    let popup = rect(220.0, 100.0, 100.0, 100.0);
    let mut tracker = SafePolygon::new(SafePolygonConfig::default());
    tracker.arm(pt(250.0, 150.0), trigger, popup, SafePolygonSide::Right);
    assert_eq!(
        tracker.evaluate(pt(250.0, 150.0), ms(5)),
        SafePolygonVerdict::LandedPopup
    );

    // Empty bounds everywhere.
    let mut tracker = SafePolygon::new(SafePolygonConfig::default());
    tracker.arm(
        pt(0.0, 0.0),
        rect(0.0, 0.0, 0.0, 0.0),
        rect(0.0, 0.0, 0.0, 0.0),
        SafePolygonSide::Bottom,
    );
    let _ = tracker.evaluate(pt(1.0, 1.0), ms(5));
}
