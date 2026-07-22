use std::time::{Duration, Instant};

use gpui::{point, px, Axis};

use crate::primitives::scroll::{
    ScrollbarFadePhase, ScrollbarRuntime, ScrollbarVisibility, FADE_OUT_DELAY, FADE_OUT_DURATION,
};

fn after(start: Instant, secs: f32) -> Instant {
    start + Duration::from_secs_f32(secs)
}

#[test]
fn always_mode_is_always_visible() {
    let runtime = ScrollbarRuntime::new();
    assert_eq!(
        runtime.opacity(ScrollbarVisibility::Always, Instant::now()),
        1.0
    );
}

#[test]
fn hover_mode_shows_only_while_hovered() {
    let mut runtime = ScrollbarRuntime::new();
    let now = Instant::now();
    assert_eq!(runtime.opacity(ScrollbarVisibility::Hover, now), 0.0);

    runtime.set_track_hovered(Axis::Vertical, true, ScrollbarVisibility::Hover, now);
    assert_eq!(runtime.opacity(ScrollbarVisibility::Hover, now), 1.0);

    runtime.set_track_hovered(Axis::Vertical, false, ScrollbarVisibility::Hover, now);
    assert_eq!(runtime.opacity(ScrollbarVisibility::Hover, now), 0.0);
}

#[test]
fn scrolling_mode_shows_on_offset_change_then_fades_to_hidden() {
    let mut runtime = ScrollbarRuntime::new();
    let start = Instant::now();
    let mode = ScrollbarVisibility::Scrolling;

    // Hidden before any observed scroll.
    assert_eq!(runtime.opacity(mode, start), 0.0);
    assert_eq!(runtime.fade_phase(mode, start), ScrollbarFadePhase::Hidden);

    // Offset change marks scrolling and shows the bar.
    assert!(runtime.observe_offset(point(px(0.0), px(-10.0)), start));
    assert_eq!(runtime.opacity(mode, start), 1.0);
    assert!(matches!(
        runtime.fade_phase(mode, start),
        ScrollbarFadePhase::Solid { .. }
    ));

    // Same offset again is not activity.
    assert!(!runtime.observe_offset(point(px(0.0), px(-10.0)), start));

    // Fully visible through the idle delay, fading after it.
    assert_eq!(
        runtime.opacity(mode, after(start, FADE_OUT_DELAY - 0.1)),
        1.0
    );
    let mid_fade = runtime.opacity(mode, after(start, FADE_OUT_DELAY + FADE_OUT_DURATION / 2.0));
    assert!(mid_fade > 0.0 && mid_fade < 1.0);
    assert_eq!(
        runtime.fade_phase(mode, after(start, FADE_OUT_DELAY + FADE_OUT_DURATION / 2.0)),
        ScrollbarFadePhase::Fading
    );

    // Fade completes to hidden and non-interactable.
    let done = after(start, FADE_OUT_DELAY + FADE_OUT_DURATION + 0.1);
    assert_eq!(runtime.opacity(mode, done), 0.0);
    assert_eq!(runtime.fade_phase(mode, done), ScrollbarFadePhase::Hidden);
    assert!(!runtime.is_interactable(mode, done));
}

#[test]
fn hover_during_visible_window_resets_idle_clock() {
    let mut runtime = ScrollbarRuntime::new();
    let start = Instant::now();
    let mode = ScrollbarVisibility::Scrolling;
    runtime.observe_offset(point(px(0.0), px(-10.0)), start);

    // Hover just before the delay expires: activity resets.
    let hover_time = after(start, FADE_OUT_DELAY - 0.5);
    runtime.set_track_hovered(Axis::Vertical, true, mode, hover_time);
    assert_eq!(
        runtime.opacity(mode, after(hover_time, FADE_OUT_DELAY - 0.1)),
        1.0
    );

    // Hovering a fully faded-out bar does not revive it.
    let mut faded = ScrollbarRuntime::new();
    faded.observe_offset(point(px(0.0), px(-10.0)), start);
    let late = after(start, FADE_OUT_DELAY + FADE_OUT_DURATION + 1.0);
    faded.set_track_hovered(Axis::Vertical, true, mode, late);
    assert_eq!(faded.opacity(mode, late), 0.0);
}

#[test]
fn dragging_forces_visibility_regardless_of_timing() {
    let mut runtime = ScrollbarRuntime::new();
    let start = Instant::now();
    let mode = ScrollbarVisibility::Scrolling;
    runtime.observe_offset(point(px(0.0), px(-10.0)), start);
    runtime.begin_drag(Axis::Vertical, px(8.0), start);

    let long_after = after(start, FADE_OUT_DELAY + FADE_OUT_DURATION + 10.0);
    assert_eq!(runtime.opacity(mode, long_after), 1.0);

    assert!(runtime.end_drag(long_after));
    assert!(!runtime.end_drag(long_after));
}

#[test]
fn drag_updates_are_rate_limited() {
    let mut runtime = ScrollbarRuntime::new();
    let start = Instant::now();
    assert!(runtime.try_claim_drag_update(start));
    // Immediately again: rejected.
    assert!(!runtime.try_claim_drag_update(start + Duration::from_millis(1)));
    // After the minimum interval: accepted.
    assert!(runtime.try_claim_drag_update(start + Duration::from_millis(20)));
}

#[test]
fn style_state_reports_runtime_facts() {
    let mut runtime = ScrollbarRuntime::new();
    let now = Instant::now();
    let mode = ScrollbarVisibility::Scrolling;

    runtime.observe_offset(point(px(0.0), px(-10.0)), now);
    runtime.set_track_hovered(Axis::Vertical, true, mode, now);
    runtime.set_thumb_hovered(Axis::Vertical, true, mode, now);
    runtime.begin_drag(Axis::Vertical, px(4.0), now);

    let state = runtime.style_state(Axis::Vertical, mode, now, true, false, true);
    assert_eq!(state.axis, Axis::Vertical);
    assert!(state.hovering_track);
    assert!(state.hovering_thumb);
    assert!(state.scrolling);
    assert!(state.dragging);
    assert!(state.has_overflow);
    assert!(!state.at_start);
    assert!(state.at_end);
    assert_eq!(state.opacity, 1.0);

    // The horizontal axis shares scroll facts but not per-axis hover/drag.
    let horizontal = runtime.style_state(Axis::Horizontal, mode, now, false, true, false);
    assert!(!horizontal.hovering_track);
    assert!(!horizontal.hovering_thumb);
    assert!(!horizontal.dragging);
    assert!(!horizontal.has_overflow);
    assert!(horizontal.at_start);
    assert!(!horizontal.at_end);
}
