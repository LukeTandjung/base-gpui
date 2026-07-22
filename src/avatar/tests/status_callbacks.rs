use gpui::TestAppContext;

use super::support::{
    clear_statuses, debug_bounds, failing_source, open_avatar, pending_source, png_source,
    read_observations, update_config, AvatarTestConfig,
};
use crate::avatar::AvatarImageLoadingStatus;

#[gpui::test]
fn callback_fires_for_loading_loaded_and_error_but_not_idle(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            ..Default::default()
        },
    );

    assert_eq!(
        read_observations(cx, window).statuses,
        vec![AvatarImageLoadingStatus::Loading]
    );

    update_config(cx, window, |config| {
        config.source = Some(png_source());
    });

    let statuses = read_observations(cx, window).statuses;
    assert_eq!(
        *statuses.last().expect("statuses should be recorded"),
        AvatarImageLoadingStatus::Loaded
    );
    assert!(!statuses.contains(&AvatarImageLoadingStatus::Idle));

    update_config(cx, window, |config| {
        config.source = Some(failing_source());
    });

    let statuses = read_observations(cx, window).statuses;
    assert_eq!(
        *statuses.last().expect("statuses should be recorded"),
        AvatarImageLoadingStatus::Error
    );
    assert!(!statuses.contains(&AvatarImageLoadingStatus::Idle));
}

#[gpui::test]
fn callback_does_not_repeat_for_unchanged_status(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            ..Default::default()
        },
    );

    // Force additional renders without a status change.
    update_config(cx, window, |_config| {});
    update_config(cx, window, |_config| {});

    assert_eq!(
        read_observations(cx, window).statuses,
        vec![AvatarImageLoadingStatus::Loading]
    );
}

#[gpui::test]
fn cached_image_resolves_to_loaded_without_fallback_flash(cx: &mut TestAppContext) {
    let source = png_source();
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(source.clone()),
            ..Default::default()
        },
    );

    // Remount the avatar with the now-cached image under a fresh root.
    clear_statuses(cx, window);
    update_config(cx, window, |config| {
        config.root_id = "avatar-test-remounted";
        config.source = Some(source);
    });

    assert_eq!(
        read_observations(cx, window).statuses,
        vec![AvatarImageLoadingStatus::Loaded]
    );
    assert!(debug_bounds(cx, window, "avatar-image").is_some());
    assert!(debug_bounds(cx, window, "avatar-fallback").is_none());
}
