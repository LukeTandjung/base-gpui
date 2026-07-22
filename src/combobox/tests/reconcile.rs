use gpui::SharedString;

use crate::combobox::tests::support::{fruit_items, single_runtime};
use crate::combobox::{
    ComboboxChangeDetails, ComboboxChangeReason, ComboboxChangeSource, ComboboxItemMetadata,
    ComboboxRuntime, ComboboxSelectionMode,
};

#[test]
fn uncontrolled_single_initial_value_is_reflected() {
    let runtime = single_runtime(Some("banana"), "", false);
    assert_eq!(runtime.selected_value(), Some("banana"));
}

#[test]
fn uncontrolled_multiple_defaults_to_empty_and_dedupes() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Multiple,
        None::<&str>,
        vec!["apple", "apple", "banana"],
        SharedString::default(),
        false,
    );
    runtime.sync_children(fruit_items(), None, false);
    runtime.reconcile_multiple(runtime.selected_values(), true);

    assert_eq!(runtime.selected_values(), vec!["apple", "banana"]);
}

#[test]
fn uncontrolled_single_prunes_missing_value_on_reconcile() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        Some("kiwi"),
        Vec::new(),
        SharedString::default(),
        false,
    );
    runtime.sync_children(fruit_items(), None, false);
    runtime.reconcile_single(runtime.selected_value(), true);

    assert_eq!(runtime.selected_value(), None);
}

#[test]
fn controlled_single_is_not_pruned() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        Some("kiwi"),
        Vec::new(),
        SharedString::default(),
        false,
    );
    runtime.sync_children(fruit_items(), None, false);
    runtime.reconcile_single(Some("kiwi"), false);

    assert_eq!(runtime.selected_value(), Some("kiwi"));
}

#[test]
fn open_reconciliation_from_context_freezes_and_completes_close() {
    let mut runtime = single_runtime(None, "ap", true);
    runtime.commit_input_value("app".into());
    assert!(runtime.query_changed_after_open());

    runtime.sync_open_from_context(false);
    // Close-query freeze keeps the filter stable while closing.
    assert_eq!(runtime.query(), "app");
    runtime.complete_close();
    assert!(!runtime.query_changed_after_open());
}

#[test]
fn change_details_cancel_state_is_respected() {
    let mut details = ComboboxChangeDetails::new(
        ComboboxChangeReason::ItemPress,
        ComboboxChangeSource::Pointer,
        true,
    );
    assert!(!details.is_canceled());
    details.cancel();
    assert!(details.is_canceled());

    let mut non_cancelable = ComboboxChangeDetails::new(
        ComboboxChangeReason::None,
        ComboboxChangeSource::None,
        false,
    );
    non_cancelable.cancel();
    assert!(!non_cancelable.is_canceled());
}

#[test]
fn disabled_or_read_only_root_rejects_open_requests() {
    let runtime = single_runtime(None, "", false);
    assert!(!runtime
        .request_open_change(false, true, true, false)
        .changed());
    assert!(!runtime
        .request_open_change(false, true, false, true)
        .changed());
    assert!(runtime
        .request_open_change(false, true, false, false)
        .changed());
}

#[test]
fn dynamic_item_removal_reconciles_multiple_selection() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Multiple,
        None::<&str>,
        vec!["apple", "banana"],
        SharedString::default(),
        false,
    );
    runtime.sync_children(
        vec![ComboboxItemMetadata::new(
            "banana",
            Some("Banana".into()),
            false,
            0,
        )],
        None,
        false,
    );
    runtime.reconcile_multiple(runtime.selected_values(), true);

    assert_eq!(runtime.selected_values(), vec!["banana"]);
}
