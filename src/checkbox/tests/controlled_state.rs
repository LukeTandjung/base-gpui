use gpui::TestAppContext;

use super::support::{
    click_checkbox, open_checkbox, read_observations, update_config, CheckboxTestConfig,
};

#[gpui::test]
fn controlled_checked_state_reflects_external_state(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            controlled_checked: Some(true),
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_root_state().checked);

    update_config(cx, window, |config| {
        config.controlled_checked = Some(false);
    });

    assert!(!read_observations(cx, window).last_root_state().checked);
}

#[gpui::test]
fn controlled_click_calls_change_without_mutating_source_of_truth(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            controlled_checked: Some(false),
            ..Default::default()
        },
    );

    click_checkbox(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert_eq!(observations.value_changes, vec![true]);
}
