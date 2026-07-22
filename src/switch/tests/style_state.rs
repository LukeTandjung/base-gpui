use gpui::TestAppContext;

use super::support::{open_switch, read_observations, SwitchTestConfig};

#[gpui::test]
fn style_with_state_receives_correct_root_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            default_checked: true,
            disabled: true,
            read_only: true,
            required: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.checked);
    assert!(!state.unchecked);
    assert!(state.disabled);
    assert!(state.read_only);
    assert!(state.required);
}

#[gpui::test]
fn style_with_state_receives_correct_thumb_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            default_checked: true,
            include_thumb: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .last_thumb_state()
        .expect("thumb state should be observed");
    assert!(state.root.checked);
    assert!(!state.root.unchecked);
}
