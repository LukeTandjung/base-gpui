use gpui::TestAppContext;

use super::support::{open_toolbar, read_observations, ToolbarTestConfig};

#[gpui::test]
fn group_disabled_cascades_to_contained_buttons_but_not_links(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            group_disabled: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.last_group_state().disabled);
    assert!(observations.last_button_state(2).disabled);
    assert!(!observations.last_button_state(0).disabled);
    assert!(!observations.last_input_state().disabled);
}
