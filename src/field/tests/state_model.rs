use gpui::{SharedString, TestAppContext};

use super::support::{open_field, read_observations, update_config, FieldTestConfig};

#[gpui::test]
fn default_root_state_is_empty(cx: &mut TestAppContext) {
    let window = open_field(cx, FieldTestConfig::default());

    let state = read_observations(cx, window).root_state();
    assert!(!state.disabled);
    assert!(!state.touched);
    assert!(!state.dirty);
    assert_eq!(state.valid, None);
    assert!(!state.filled);
    assert!(!state.focused);
}

#[gpui::test]
fn controlled_dirty_touched_and_invalid_state_is_reflected(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            root_dirty: Some(true),
            root_touched: Some(true),
            root_invalid: Some(true),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).root_state();
    assert!(state.dirty);
    assert!(state.touched);
    assert_eq!(state.valid, Some(false));
    assert!(state.invalid);
}

#[gpui::test]
fn field_tracks_filled_and_dirty_from_registered_control(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            value: SharedString::from("initial"),
            ..Default::default()
        },
    );
    assert!(read_observations(cx, window).root_state().filled);
    assert!(!read_observations(cx, window).root_state().dirty);

    update_config(cx, window, |config| {
        config.value = SharedString::from("changed");
    });

    let state = read_observations(cx, window).root_state();
    assert!(state.filled);
    assert!(state.dirty);

    update_config(cx, window, |config| {
        config.value = SharedString::default();
    });

    let state = read_observations(cx, window).root_state();
    assert!(!state.filled);
    assert!(state.dirty);
}
