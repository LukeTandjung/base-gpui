use gpui::TestAppContext;

use crate::{separator::SeparatorOrientation, toolbar::ToolbarOrientation};

use super::support::{open_toolbar, read_observations, ToolbarTestConfig};

#[gpui::test]
fn separator_defaults_to_perpendicular_and_honors_an_override(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());
    assert_eq!(
        read_observations(cx, window)
            .last_separator_state()
            .orientation,
        SeparatorOrientation::Vertical
    );

    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            orientation: ToolbarOrientation::Vertical,
            ..Default::default()
        },
    );
    assert_eq!(
        read_observations(cx, window)
            .last_separator_state()
            .orientation,
        SeparatorOrientation::Horizontal
    );

    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            separator_override: Some(SeparatorOrientation::Horizontal),
            ..Default::default()
        },
    );
    assert_eq!(
        read_observations(cx, window)
            .last_separator_state()
            .orientation,
        SeparatorOrientation::Horizontal
    );
}
