use gpui::TestAppContext;

use super::support::{
    click_item, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn clicking_an_item_makes_it_the_roving_tab_stop(cx: &mut TestAppContext) {
    let window = open_toolbar(cx, ToolbarTestConfig::default());

    click_item(cx, window, 1);
    let observations = read_observations(cx, window);
    assert_eq!(observations.focused_item(), Some(1));
    assert!(observations.last_button_state(1).tab_stop);

    simulate_keys(cx, window, "right");
    assert_eq!(read_observations(cx, window).focused_item(), Some(2));
}
