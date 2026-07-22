//! Scrolling flags clear exactly `SCROLL_TIMEOUT` past the last activity,
//! and the deadline extends on continued scrolling.

use std::time::{Duration, Instant};

use gpui::{point, px};

use crate::scroll_area::{ScrollAreaOrientation, ScrollAreaRuntime, SCROLL_TIMEOUT};

#[test]
fn flag_clears_exactly_at_scroll_timeout() {
    let mut runtime = ScrollAreaRuntime::new();
    let start = Instant::now();
    runtime.observe_scroll(point(px(0.0), px(-10.0)), start);

    assert!(!runtime.expire_scrolling(start + SCROLL_TIMEOUT - Duration::from_millis(1)));
    assert!(runtime.scrolling(ScrollAreaOrientation::Vertical));

    assert!(runtime.expire_scrolling(start + SCROLL_TIMEOUT));
    assert!(!runtime.scrolling(ScrollAreaOrientation::Vertical));
}

#[test]
fn continued_scrolling_extends_the_deadline() {
    let mut runtime = ScrollAreaRuntime::new();
    let start = Instant::now();
    runtime.observe_scroll(point(px(0.0), px(-10.0)), start);

    let later = start + Duration::from_millis(300);
    runtime.observe_scroll(point(px(0.0), px(-20.0)), later);

    // The original deadline has passed, but the extended one has not.
    assert!(!runtime.expire_scrolling(start + SCROLL_TIMEOUT));
    assert!(runtime.scrolling(ScrollAreaOrientation::Vertical));

    assert!(runtime.expire_scrolling(later + SCROLL_TIMEOUT));
    assert!(!runtime.scrolling(ScrollAreaOrientation::Vertical));
}

#[test]
fn remaining_scroll_activity_tracks_the_latest_deadline() {
    let mut runtime = ScrollAreaRuntime::new();
    let start = Instant::now();

    assert_eq!(runtime.remaining_scroll_activity(start), None);

    runtime.observe_scroll(point(px(-5.0), px(0.0)), start);
    let later = start + Duration::from_millis(200);
    runtime.observe_scroll(point(px(-5.0), px(-5.0)), later);

    assert_eq!(
        runtime.remaining_scroll_activity(later),
        Some(SCROLL_TIMEOUT)
    );
}
