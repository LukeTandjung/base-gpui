use gpui::TestAppContext;

use super::support::{open_checkbox, read_observations, CheckboxTestConfig};

#[gpui::test]
fn style_with_state_receives_correct_root_state(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            default_checked: true,
            indeterminate: true,
            disabled: true,
            read_only: true,
            required: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.checked);
    assert!(!state.unchecked);
    assert!(state.indeterminate);
    assert!(state.disabled);
    assert!(state.read_only);
    assert!(state.required);
}

#[gpui::test]
fn style_with_state_receives_correct_indicator_state(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            default_checked: true,
            include_indicator: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .last_indicator_state()
        .expect("indicator state should be observed");
    assert!(state.present);
    assert!(state.root.checked);
    assert!(!state.root.indeterminate);
}
