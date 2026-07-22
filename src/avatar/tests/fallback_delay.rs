use std::time::Duration;

use gpui::TestAppContext;

use super::support::{
    advance_clock, debug_bounds, open_avatar, pending_source, png_source, AvatarTestConfig,
};

#[gpui::test]
fn delayed_fallback_is_absent_before_delay_and_present_after(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            fallback_delay: Some(Duration::from_millis(600)),
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "avatar-fallback").is_none());

    advance_clock(cx, Duration::from_millis(700));

    assert!(debug_bounds(cx, window, "avatar-fallback").is_some());
}

#[gpui::test]
fn delayed_fallback_never_appears_when_image_loads_first(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(png_source()),
            fallback_delay: Some(Duration::from_millis(600)),
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "avatar-image").is_some());
    assert!(debug_bounds(cx, window, "avatar-fallback").is_none());

    advance_clock(cx, Duration::from_millis(700));

    assert!(debug_bounds(cx, window, "avatar-image").is_some());
    assert!(debug_bounds(cx, window, "avatar-fallback").is_none());
}
