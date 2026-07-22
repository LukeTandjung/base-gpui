use gpui::TestAppContext;

use super::support::{open_switch, read_observations, SwitchTestConfig};

#[gpui::test]
fn thumb_receives_checked_state(cx: &mut TestAppContext) {
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
}

#[gpui::test]
fn thumb_receives_unchecked_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            include_thumb: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .last_thumb_state()
        .expect("thumb state should be observed");
    assert!(state.root.unchecked);
}

#[gpui::test]
fn thumb_receives_disabled_read_only_and_required_state(cx: &mut TestAppContext) {
    let window = open_switch(
        cx,
        SwitchTestConfig {
            include_thumb: true,
            disabled: true,
            read_only: true,
            required: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window)
        .last_thumb_state()
        .expect("thumb state should be observed");
    assert!(state.root.disabled);
    assert!(state.root.read_only);
    assert!(state.root.required);
}

#[gpui::test]
fn switch_root_works_without_thumb(cx: &mut TestAppContext) {
    let window = open_switch(cx, SwitchTestConfig::default());

    let observations = read_observations(cx, window);
    assert!(!observations.last_root_state().checked);
    assert!(observations.last_thumb_state().is_none());
}
