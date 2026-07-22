use gpui::TestAppContext;

use super::support::{
    debug_bounds, failing_source, open_avatar, read_observations, AvatarTestConfig,
};
use crate::avatar::AvatarImageLoadingStatus;

#[gpui::test]
fn fallback_is_visible_after_image_load_error(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(failing_source()),
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "avatar-fallback").is_some());
    assert!(debug_bounds(cx, window, "avatar-image").is_none());
    assert_eq!(
        read_observations(cx, window)
            .last_root_state()
            .image_loading_status,
        AvatarImageLoadingStatus::Error
    );
}
