use gpui::SharedString;

use crate::autocomplete::tests::support::none_mode_runtime;
use crate::combobox::ComboboxMove;

#[test]
fn keyboard_highlight_sets_overlay_without_touching_typed_value() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.commit_input_value("b".into());
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();

    assert_eq!(runtime.display_value(), SharedString::from("Banana"));
    assert_eq!(runtime.input_value(), SharedString::from("b"));
    assert_eq!(runtime.query(), "b");
}

#[test]
fn unhighlight_restores_typed_text() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();
    assert_eq!(runtime.display_value(), SharedString::from("Banana"));

    runtime.highlight_item(None, false);
    runtime.sync_inline_overlay();
    assert_eq!(runtime.display_value(), SharedString::from("b"));
}

#[test]
fn closing_the_popup_clears_the_overlay() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();

    runtime.commit_open(false);
    assert_eq!(runtime.inline_overlay(), None);
    runtime.complete_close();
    assert_eq!(runtime.display_value(), SharedString::from("b"));
}

#[test]
fn typing_clears_the_overlay_and_commits_the_typed_value() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();

    runtime.commit_input_value("ba".into());
    assert_eq!(runtime.inline_overlay(), None);
    assert_eq!(runtime.display_value(), SharedString::from("ba"));
    assert_eq!(runtime.input_value(), SharedString::from("ba"));
}

#[test]
fn clear_commits_clear_the_overlay() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();

    runtime.commit_clear();
    assert_eq!(runtime.inline_overlay(), None);
    assert_eq!(runtime.display_value(), SharedString::default());
}

#[test]
fn overlay_does_not_affect_dirty_or_clear_visibility() {
    let mut runtime = none_mode_runtime("", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();

    // Facts key off the committed value, not the overlay display.
    assert!(!runtime.clear_visible());
}

#[test]
fn pointer_highlight_alone_does_not_change_the_displayed_text() {
    // The context only syncs the overlay for non-Pointer highlight reasons;
    // a pointer highlight without a sync leaves the display untouched.
    let mut runtime = none_mode_runtime("b", true);
    runtime.highlight_item(Some(1), false);
    assert_eq!(runtime.display_value(), SharedString::from("b"));
}

#[test]
fn controlled_input_value_change_clears_the_overlay() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();

    runtime.reconcile_input_value(Some("ch".into()));
    assert_eq!(runtime.inline_overlay(), None);
    assert_eq!(runtime.display_value(), SharedString::from("ch"));
}

#[test]
fn sync_without_highlight_clears_overlay() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.set_inline_overlay(Some("Banana".into()));
    runtime.sync_inline_overlay();
    assert_eq!(runtime.inline_overlay(), None);
}
