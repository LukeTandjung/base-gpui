use gpui::{Modifiers, MouseButton, SharedString, TestAppContext, VisualTestContext};

use super::support::{
    debug_bounds, focus_input, open_input, read_clipboard, read_observations, simulate_keys,
    write_clipboard, InputTestConfig,
};

#[gpui::test]
fn user_text_insertion_updates_uncontrolled_value(cx: &mut TestAppContext) {
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
fn backspace_removes_previous_grapheme(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("a💝"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "backspace");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("a")
    );
}

#[gpui::test]
fn delete_removes_next_grapheme(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("a💝"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "home delete");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("💝")
    );
}

#[gpui::test]
fn replacing_selection_inserts_new_text(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("ab"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "shift-left c");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("ac")
    );
}

#[gpui::test]
fn left_and_right_cursor_movement_respects_grapheme_boundaries(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("a💝b"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "left left x right y");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("ax💝yb")
    );
}

#[gpui::test]
fn shift_right_extends_selection(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("ab"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "home shift-right x");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("xb")
    );
}

#[gpui::test]
fn home_and_end_move_to_line_boundaries(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("ab"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "home x end y");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("xaby")
    );
}

#[gpui::test]
fn select_all_replaces_whole_value(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-a x");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-a x");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("x")
    );
}

#[gpui::test]
fn copy_copies_selected_text(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "shift-left");
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-c");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-c");

    assert_eq!(read_clipboard(cx), Some("c".to_string()));
}

#[gpui::test]
fn cut_removes_selected_text(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "shift-left");
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-x");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-x");

    assert_eq!(read_clipboard(cx), Some("c".to_string()));
    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("ab")
    );
}

#[gpui::test]
fn paste_inserts_clipboard_text_and_normalizes_line_breaks(cx: &mut TestAppContext) {
    let window = open_input(cx, InputTestConfig::default());
    write_clipboard(cx, "a\nb");

    focus_input(cx, window);
    #[cfg(target_os = "macos")]
    simulate_keys(cx, window, "cmd-v");
    #[cfg(not(target_os = "macos"))]
    simulate_keys(cx, window, "ctrl-v");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("a b")
    );
}

#[gpui::test]
fn mouse_click_moves_cursor(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("ab"),
            ..Default::default()
        },
    );
    let bounds = debug_bounds(cx, window, "input-root").expect("input should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(
        gpui::point(bounds.left() + gpui::px(1.0), bounds.center().y),
        Modifiers::default(),
    );
    visual.run_until_parked();
    simulate_keys(cx, window, "c");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("cab")
    );
}

#[gpui::test]
fn mouse_drag_selects_text(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );
    let bounds = debug_bounds(cx, window, "input-root").expect("input should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    let start = gpui::point(bounds.right() - gpui::px(1.0), bounds.center().y);
    let end = gpui::point(bounds.left() + gpui::px(1.0), bounds.center().y);

    visual.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
    visual.simulate_mouse_move(end, Some(MouseButton::Left), Modifiers::default());
    visual.simulate_mouse_up(end, MouseButton::Left, Modifiers::default());
    visual.run_until_parked();
    simulate_keys(cx, window, "x");

    assert_eq!(
        read_observations(cx, window).last_state().value,
        SharedString::from("x")
    );
}

#[gpui::test]
fn enter_calls_enter_handler_without_inserting_newline(cx: &mut TestAppContext) {
    let window = open_input(
        cx,
        InputTestConfig {
            default_value: SharedString::from("abc"),
            ..Default::default()
        },
    );

    focus_input(cx, window);
    simulate_keys(cx, window, "enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_state().value, SharedString::from("abc"));
    assert_eq!(observations.enter_values, vec![SharedString::from("abc")]);
}
