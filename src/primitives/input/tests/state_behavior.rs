use gpui::{SharedString, TestAppContext};

use super::support::{
    focus_input, open_input, read_observations, simulate_keys, update_config, InputTestConfig,
};

#[gpui::test]
fn default_input_value_is_empty(cx: &mut TestAppContext) {
    let window = open_input(cx, InputTestConfig::default());

    let state = read_observations(cx, window).last_state();
    assert!(state.value.is_empty());
    assert!(state.empty);
    assert!(!state.filled);
}

#[gpui::test]
fn default_value_initializes_displayed_text(cx: &mut TestAppContext) {
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
fn controlled_value_is_displayed_and_updates(cx: &mut TestAppContext) {
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
fn auto_focus_focuses_input_on_first_render(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            auto_focus: true,
            ..Default::default()
        },
    );

    assert!(read_observations(cx, window).last_state().focused);
}

#[gpui::test]
fn controlled_user_edit_calls_change_without_mutating_source_of_truth(cx: &mut TestAppContext) {
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
fn disabled_input_ignores_editing(cx: &mut TestAppContext) {
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
fn uncontrolled_rerender_preserves_user_text_selection_and_focus(cx: &mut TestAppContext) {
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
fn changed_default_value_does_not_clobber_focused_uncontrolled_edits(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("a"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "b");
    update_config(cx, window, |config| {
        config.default_value = SharedString::from("z");
    });

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("ab")
    );
}

#[gpui::test]
fn read_only_input_ignores_editing_but_can_focus(cx: &mut TestAppContext) {
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
