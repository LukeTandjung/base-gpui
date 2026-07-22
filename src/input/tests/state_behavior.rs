use gpui::{SharedString, TestAppContext};

use super::support::{
    focus_input, open_input, read_observations, simulate_keys, update_config, InputTestConfig,
};

#[gpui::test]
fn public_input_default_value_is_empty(cx: &mut TestAppContext) {
    let window = open_input(cx, InputTestConfig::default());

    let state = read_observations(cx, window).last_state();
    assert!(state.value.is_empty());
    assert!(state.empty);
    assert!(!state.filled);
}

#[gpui::test]
fn public_input_default_value_initializes_displayed_text(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("hello"),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_state();
    assert_eq!(state.value, SharedString::from("hello"));
    assert!(state.filled);
}

#[gpui::test]
fn public_controlled_input_value_is_displayed_and_updates(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            value: Some(SharedString::from("one")),
            ..Default::default()
        },
    );

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("one")
    );

    update_config(cx, window, |config| {
        config.value = Some(SharedString::from("two"));
    });

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("two")
    );
}

#[gpui::test]
fn public_uncontrolled_input_user_edits_update_text(cx: &mut TestAppContext) {
    let window = open_input(cx, InputTestConfig::default());

    focus_input(cx, window);
    simulate_keys(cx, window, "a b");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_state().value, SharedString::from("ab"));
    assert_eq!(
        observations.value_changes.last(),
        Some(&SharedString::from("ab"))
    );
}

#[gpui::test]
fn public_controlled_input_user_edits_call_change_without_mutating(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            value: Some(SharedString::from("a")),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "b");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_state().value, SharedString::from("a"));
    assert_eq!(observations.value_changes, vec![SharedString::from("ab")]);
}

#[gpui::test]
fn public_input_preserves_keyboard_editing_behavior(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "shift-left x home y");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("yabx")
    );
}

#[gpui::test]
fn public_uncontrolled_rerender_preserves_user_text_selection_and_focus(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "shift-left");
    update_config(cx, window, |config| {
        config.placeholder = SharedString::from("updated placeholder");
    });
    simulate_keys(cx, window, "x");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_state().value, SharedString::from("abx"));
    assert!(observations.last_state().focused);
}

#[gpui::test]
fn public_disabled_input_ignores_editing(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("a"),
            disabled: true,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "b");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_state().value, SharedString::from("a"));
    assert!(!observations.last_state().focused);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn public_read_only_input_ignores_editing_but_can_focus(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("a"),
            read_only: true,
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "b");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_state().value, SharedString::from("a"));
    assert!(observations.last_state().focused);
    assert!(observations.value_changes.is_empty());
}
