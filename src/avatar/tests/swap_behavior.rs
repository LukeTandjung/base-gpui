use gpui::TestAppContext;

use super::support::{
    debug_bounds, open_avatar, pending_source, png_source, update_config, AvatarTestConfig,
};

#[gpui::test]
fn fallback_is_visible_while_image_is_loading(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "avatar-fallback").is_some());
    assert!(debug_bounds(cx, window, "avatar-image").is_none());
}

#[gpui::test]
fn image_swaps_in_on_loaded_and_fallback_unmounts(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.source = Some(png_source());
    });

    assert!(debug_bounds(cx, window, "avatar-image").is_some());
    assert!(debug_bounds(cx, window, "avatar-fallback").is_none());
}

#[gpui::test]
fn only_one_of_image_or_fallback_is_mounted_when_switching(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            ..Default::default()
        },
    );

    let image_mounted = debug_bounds(cx, window, "avatar-image").is_some();
    let fallback_mounted = debug_bounds(cx, window, "avatar-fallback").is_some();
    assert!(fallback_mounted && !image_mounted);

    update_config(cx, window, |config| {
        config.source = Some(png_source());
    });

    let image_mounted = debug_bounds(cx, window, "avatar-image").is_some();
    let fallback_mounted = debug_bounds(cx, window, "avatar-fallback").is_some();
    assert!(image_mounted && !fallback_mounted);
}

#[gpui::test]
fn fallback_only_root_shows_fallback(cx: &mut TestAppContext) {
    let window = open_avatar(cx, AvatarTestConfig::default());

    assert!(debug_bounds(cx, window, "avatar-fallback").is_some());
    assert!(debug_bounds(cx, window, "avatar-image").is_none());
}
