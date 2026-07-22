use gpui::TestAppContext;

use super::support::{click_trigger, open_collapsible, read_observations, CollapsibleTestConfig};

#[gpui::test]
fn style_with_state_receives_closed_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            keep_mounted_panel: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().closed);
    assert!(observations.last_trigger_state().closed);
    assert!(observations.last_panel_state().expect("panel state").closed);
}

#[gpui::test]
fn style_with_state_receives_open_state(cx: &mut TestAppContext) {
    let window = open_collapsible(cx, CollapsibleTestConfig::default());

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().open);
    assert!(observations.last_trigger_state().open);
    assert!(observations.last_panel_state().expect("panel state").open);
}

#[gpui::test]
fn style_with_state_receives_disabled_state(cx: &mut TestAppContext) {
    let window = open_collapsible(
        cx,
        CollapsibleTestConfig {
            disabled: true,
            keep_mounted_panel: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.last_root_state().disabled);
    assert!(observations.last_trigger_state().disabled);
}
