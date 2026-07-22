use gpui::TestAppContext;

use super::support::{
    open_avatar, pending_source, png_source, read_observations, update_config, AvatarTestConfig,
};
use crate::avatar::AvatarImageLoadingStatus;

#[gpui::test]
fn each_part_observes_the_shared_loading_status(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(pending_source()),
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().image_loading_status,
        AvatarImageLoadingStatus::Loading
    );
    assert_eq!(
        observations
            .fallback_states
            .last()
            .expect("fallback state should be observed")
            .image_loading_status,
        AvatarImageLoadingStatus::Loading
    );

    update_config(cx, window, |config| {
        config.source = Some(png_source());
    });

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().image_loading_status,
        AvatarImageLoadingStatus::Loaded
    );
    assert_eq!(
        observations
            .image_states
            .last()
            .expect("image state should be observed")
            .image_loading_status,
        AvatarImageLoadingStatus::Loaded
    );
}

#[gpui::test]
fn fallback_only_root_reports_idle_status(cx: &mut TestAppContext) {
    let window = open_avatar(cx, AvatarTestConfig::default());

    let observations = read_observations(cx, window);
    assert_eq!(
        observations.last_root_state().image_loading_status,
        AvatarImageLoadingStatus::Idle
    );
    assert!(observations.statuses.is_empty());
}
