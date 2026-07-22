use gpui::SharedString;

use crate::combobox::tests::support::{multiple_runtime, single_runtime};

#[test]
fn default_input_value_derives_from_selected_label_in_single_mode() {
    let mut runtime = single_runtime(Some("banana"), "", false);
    runtime.derive_initial_input_from_selection();

    assert_eq!(runtime.input_value(), SharedString::from("Banana"));
}

#[test]
fn derivation_does_not_overwrite_an_explicit_input_value() {
    let mut runtime = single_runtime(Some("banana"), "typed", false);
    runtime.derive_initial_input_from_selection();

    assert_eq!(runtime.input_value(), SharedString::from("typed"));
}

#[test]
fn input_cleanup_after_close_syncs_single_to_selected_label() {
    let mut runtime = single_runtime(Some("banana"), "Ban", true);
    runtime.commit_open(false);

    assert_eq!(
        runtime.input_cleanup_after_close(),
        Some(SharedString::from("Banana"))
    );
}

#[test]
fn input_cleanup_after_close_clears_single_when_nothing_selected() {
    let mut runtime = single_runtime(None, "Ban", true);
    runtime.commit_open(false);

    assert_eq!(
        runtime.input_cleanup_after_close(),
        Some(SharedString::default())
    );
}

#[test]
fn input_cleanup_after_close_clears_multiple_filter() {
    let mut runtime = multiple_runtime(vec!["apple"], "Ban", true);
    runtime.commit_open(false);

    assert_eq!(
        runtime.input_cleanup_after_close(),
        Some(SharedString::default())
    );
}

#[test]
fn input_sync_for_selected_reports_label_for_programmatic_change() {
    let mut runtime = single_runtime(None, "", false);
    runtime.sync_selected_from_context(Some("apple"));

    assert_eq!(
        runtime.input_sync_for_selected(),
        Some(SharedString::from("Apple"))
    );
}

#[test]
fn inline_overlay_is_a_whole_value_display_replacement() {
    let mut runtime = single_runtime(None, "ban", true);
    runtime.set_inline_overlay(Some("Banana".into()));

    // Display value paints the overlay; the real input value is untouched.
    assert_eq!(runtime.display_value(), SharedString::from("Banana"));
    assert_eq!(runtime.input_value(), SharedString::from("ban"));

    // User typing clears the overlay.
    runtime.commit_input_value("bana".into());
    assert_eq!(runtime.inline_overlay(), None);
    assert_eq!(runtime.display_value(), SharedString::from("bana"));
}

#[test]
fn clear_visibility_follows_mode() {
    let runtime = single_runtime(Some("banana"), "", false);
    assert!(runtime.clear_visible());

    let runtime = single_runtime(None, "", false);
    assert!(!runtime.clear_visible());

    let runtime = multiple_runtime(vec!["apple"], "", false);
    assert!(runtime.clear_visible());

    let runtime = multiple_runtime(Vec::new(), "", false);
    assert!(!runtime.clear_visible());
}

#[test]
fn commit_clear_resets_input_selection_and_highlight() {
    let mut runtime = multiple_runtime(vec!["apple", "banana"], "Ban", true);
    runtime.commit_clear();

    assert!(runtime.selected_values().is_empty());
    assert_eq!(runtime.input_value(), SharedString::default());
    assert_eq!(runtime.highlighted_index(), None);
    assert_eq!(runtime.highlighted_chip_index(), None);
}
