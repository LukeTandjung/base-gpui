use gpui::TestAppContext;

use crate::toolbar::ToolbarOrientation;

use super::support::{open_toolbar, read_observations, ToolbarTestConfig};

#[gpui::test]
fn style_states_expose_disabled_orientation_and_focusable_facts(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            second_disabled: true,
            group_disabled: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);

    let root = observations.last_root_state();
    assert!(!root.disabled);
    assert_eq!(root.orientation, ToolbarOrientation::Horizontal);

    let first = observations.last_button_state(0);
    assert!(!first.disabled);
    assert!(first.focusable);
    assert!(first.tab_stop);

    let second = observations.last_button_state(1);
    assert!(second.disabled);
    assert!(second.focusable);

    let grouped = observations.last_button_state(2);
    assert!(grouped.disabled);

    let link = observations.last_link_state();
    assert_eq!(link.orientation, ToolbarOrientation::Horizontal);
    assert!(!link.tab_stop);

    let input = observations.last_input_state();
    assert!(!input.disabled);
    assert!(input.focusable);

    let group = observations.last_group_state();
    assert!(group.disabled);
}
