use gpui::TestAppContext;

use super::support::{click_item, open_toolbar, read_observations, ToolbarTestConfig, LINK_INDEX};

#[gpui::test]
fn toolbar_disabled_cascades_to_buttons_and_inputs_but_not_links(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            toolbar_disabled: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().disabled);
    assert!(observations.last_button_state(0).disabled);
    assert!(observations.last_button_state(2).disabled);
    assert!(observations.last_input_state().disabled);

    click_item(cx, window, 0);
    assert!(read_observations(cx, window).clicks.is_empty());

    click_item(cx, window, LINK_INDEX);
    assert_eq!(read_observations(cx, window).clicks, vec![LINK_INDEX]);
}
