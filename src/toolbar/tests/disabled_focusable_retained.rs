use gpui::TestAppContext;

use super::support::{
    focus_toolbar, open_toolbar, read_observations, simulate_keys, ToolbarTestConfig,
};

#[gpui::test]
fn disabled_but_focusable_items_stay_in_the_roving_order(cx: &mut TestAppContext) {
    let window = open_toolbar(
        cx,
        ToolbarTestConfig {
            second_disabled: true,
            ..Default::default()
        },
    );

    focus_toolbar(cx, window);
    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.focused_item(), Some(1));

    let state = observations.last_button_state(1);
    assert!(state.disabled);
    assert!(state.focusable);
}
