use std::rc::Rc;

use gpui::SharedString;

use crate::combobox::tests::support::{multiple_runtime, single_runtime};
use crate::combobox::ComboboxAutoHighlight;

#[test]
fn default_filter_is_case_insensitive_contains_on_label() {
    let mut runtime = single_runtime(None, "", true);
    runtime.commit_input_value("bAn".into());

    assert_eq!(runtime.filtered_indices(), vec![1]);
}

#[test]
fn empty_query_shows_all_items() {
    let mut runtime = single_runtime(None, "", true);
    runtime.refilter();

    assert_eq!(runtime.filtered_indices(), vec![0, 1, 2, 3]);
}

#[test]
fn custom_filter_replaces_default() {
    let mut runtime = single_runtime(None, "", true);
    runtime.sync_filtering(
        Some(Rc::new(|value: &&str, _label, query: &str| {
            value.starts_with(query)
        })),
        false,
        None,
        None,
        ComboboxAutoHighlight::Off,
    );
    runtime.commit_input_value("b".into());

    assert_eq!(runtime.filtered_indices(), vec![1, 3]);
}

#[test]
fn disabled_filtering_shows_all_registered_items() {
    let mut runtime = single_runtime(None, "", true);
    runtime.sync_filtering(None, true, None, None, ComboboxAutoHighlight::Off);
    runtime.commit_input_value("zzz".into());

    assert_eq!(runtime.filtered_indices(), vec![0, 1, 2, 3]);
}

#[test]
fn limit_caps_the_filtered_list_deterministically() {
    let mut runtime = single_runtime(None, "", true);
    runtime.sync_filtering(None, false, None, Some(2), ComboboxAutoHighlight::Off);
    runtime.refilter();

    assert_eq!(runtime.filtered_indices(), vec![0, 1]);
}

#[test]
fn single_mode_browse_bypass_shows_all_until_query_changes() {
    // Open with the query exactly matching the selected label: browse mode.
    let mut runtime = single_runtime(Some("banana"), "Banana", false);
    runtime.commit_open(true);
    assert_eq!(runtime.filtered_indices(), vec![0, 1, 2, 3]);

    // Editing the query after open re-enables normal filtering.
    runtime.commit_input_value("Banan".into());
    assert!(runtime.query_changed_after_open());
    assert_eq!(runtime.filtered_indices(), vec![1]);
}

#[test]
fn close_freezes_query_and_complete_close_resets_it() {
    let mut runtime = single_runtime(None, "", true);
    runtime.commit_input_value("blue".into());
    assert_eq!(runtime.filtered_indices(), vec![3]);

    runtime.commit_open(false);
    runtime.sync_input_value_from_context(SharedString::default());
    // Frozen close-query keeps the list filtered while closing.
    assert_eq!(runtime.query(), "blue");
    assert_eq!(runtime.filtered_indices(), vec![3]);

    runtime.complete_close();
    assert_eq!(runtime.query(), "");
    assert_eq!(runtime.filtered_indices(), vec![0, 1, 2, 3]);
}

#[test]
fn empty_filtered_list_reports_empty_and_clears_highlight() {
    let mut runtime = single_runtime(None, "", true);
    runtime.sync_filtering(None, false, None, None, ComboboxAutoHighlight::Always);
    runtime.commit_input_value("a".into());
    assert!(runtime.highlighted_index().is_some());

    runtime.commit_input_value("zzz".into());
    assert!(runtime.list_empty());
    assert_eq!(runtime.highlighted_index(), None);
    assert!(runtime.empty_state().empty);
}

#[test]
fn multiple_commit_clears_active_filter() {
    let mut runtime = multiple_runtime(Vec::new(), "Ban", true);
    let cleared = runtime.commit_selected_values(vec!["banana"]);

    assert_eq!(cleared, Some(SharedString::default()));
    assert_eq!(runtime.input_value(), SharedString::default());
    assert_eq!(runtime.filtered_indices(), vec![0, 1, 2, 3]);
}
