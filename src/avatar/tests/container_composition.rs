use gpui::TestAppContext;

use super::support::{debug_bounds, open_avatar, png_source, AvatarTestConfig};

#[gpui::test]
fn avatar_renders_inside_containers_without_affecting_siblings(cx: &mut TestAppContext) {
    let window = open_avatar(
        cx,
        AvatarTestConfig {
            source: Some(png_source()),
            ..Default::default()
        },
    );

    assert!(debug_bounds(cx, window, "avatar-root").is_some());
    assert!(debug_bounds(cx, window, "avatar-sibling").is_some());
}
