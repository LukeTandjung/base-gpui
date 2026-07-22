use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use crate::select::{
    SelectAlign, SelectItemMetadata, SelectMove, SelectOpenChangeDetails, SelectOpenChangeReason,
    SelectOpenChangeSource, SelectProps, SelectRuntime, SelectSelectionChange, SelectSelectionMode,
    SelectSide, SelectTypeaheadOutcome, SelectValueChangeDetails, SelectValueChangeReason,
    SelectValueChangeSource,
};

#[test]
fn uncontrolled_single_initial_value_is_reflected() {
    let mut runtime = SelectRuntime::new(
        SelectSelectionMode::Single,
        Some("banana"),
        Vec::new(),
        false,
    );
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("apple", Some("Apple".into()), false, 0),
            SelectItemMetadata::new("banana", Some("Banana".into()), false, 1),
        ],
        Vec::new(),
        None,
        false,
        None,
    );
    runtime.reconcile_single(runtime.selected_value(), true);

    assert_eq!(runtime.selected_value(), Some("banana"));
}

#[test]
fn root_state_derives_selected_index_from_registered_items() {
    let mut runtime = SelectRuntime::new(
        SelectSelectionMode::Single,
        Some("banana"),
        Vec::new(),
        false,
    );
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("apple", Some("Apple".into()), false, 2),
            SelectItemMetadata::new("banana", Some("Banana".into()), false, 5),
        ],
        Vec::new(),
        None,
        false,
        None,
    );
    let props = SelectProps::new(
        None,
        None,
        false,
        false,
        false,
        true,
        SelectSelectionMode::Single,
        true,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    assert_eq!(runtime.root_state(&props).selected_index, Some(5));
}

#[test]
fn touch_open_source_is_exposed_to_positioning_state() {
    let mut runtime =
        SelectRuntime::new(SelectSelectionMode::Single, None::<&str>, Vec::new(), false);
    runtime.commit_open(true, SelectOpenChangeSource::Touch);
    let props = SelectProps::new(
        None,
        None,
        false,
        false,
        false,
        true,
        SelectSelectionMode::Single,
        true,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let root_state = runtime.root_state(&props);
    assert_eq!(root_state.open_source, SelectOpenChangeSource::Touch);
    assert!(root_state.touch_open);

    let positioner_state = runtime.positioner_state(SelectSide::Bottom, SelectAlign::Start);
    assert_eq!(positioner_state.open_source, SelectOpenChangeSource::Touch);
    assert!(positioner_state.touch_open);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ComparatorValue {
    id: u8,
    key: &'static str,
}

#[test]
fn custom_value_comparator_controls_selection_equality() {
    let mut runtime = SelectRuntime::new(
        SelectSelectionMode::Single,
        Some(ComparatorValue {
            id: 99,
            key: "banana",
        }),
        Vec::new(),
        true,
    );
    runtime.sync_value_comparator(Some(Rc::new(
        |left: &ComparatorValue, right: &ComparatorValue| left.key == right.key,
    )));
    runtime.sync_selected_from_context(Some(ComparatorValue {
        id: 100,
        key: "banana",
    }));
    runtime.sync_children(
        vec![
            SelectItemMetadata::new(
                ComparatorValue {
                    id: 1,
                    key: "apple",
                },
                Some("Apple".into()),
                false,
                0,
            ),
            SelectItemMetadata::new(
                ComparatorValue {
                    id: 2,
                    key: "banana",
                },
                Some("Banana".into()),
                false,
                1,
            ),
        ],
        Vec::new(),
        None,
        false,
        None,
    );
    runtime.reconcile_single(runtime.selected_value(), false);
    let props = SelectProps::new(
        None,
        None,
        false,
        false,
        false,
        true,
        SelectSelectionMode::Single,
        true,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    let state = runtime.root_state(&props);
    assert_eq!(state.selected_index, Some(1));
    assert!(!state.dirty);
}

#[test]
fn controlled_single_reconciliation_uses_observed_value() {
    let mut runtime = SelectRuntime::new(
        SelectSelectionMode::Single,
        Some("apple"),
        Vec::new(),
        false,
    );
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("apple", Some("Apple".into()), false, 0),
            SelectItemMetadata::new("banana", Some("Banana".into()), false, 1),
        ],
        Vec::new(),
        None,
        false,
        None,
    );
    runtime.reconcile_single(Some("banana"), false);

    assert_eq!(runtime.selected_value(), Some("banana"));
}

#[test]
fn controlled_open_state_reconciliation_uses_observed_value() {
    let mut runtime =
        SelectRuntime::new(SelectSelectionMode::Single, None::<&str>, Vec::new(), false);
    runtime.sync_open_from_context(true);

    assert!(runtime.open_value());
}

#[test]
fn force_mounted_overlay_parts_report_mounted_state() {
    let runtime = SelectRuntime::new(SelectSelectionMode::Single, None::<&str>, Vec::new(), false);

    let portal_state = runtime.portal_state(true);
    let popup_state = runtime.popup_state(SelectSide::Bottom, SelectAlign::Start, true);
    let backdrop_state = runtime.backdrop_state(true);

    assert!(!portal_state.open);
    assert!(portal_state.mounted);
    assert!(!popup_state.open);
    assert!(popup_state.mounted);
    assert!(!backdrop_state.open);
    assert!(backdrop_state.mounted);
}

#[test]
fn value_change_details_can_cancel() {
    let mut details = SelectValueChangeDetails::new(
        SelectValueChangeReason::ItemPress,
        SelectValueChangeSource::Pointer,
        true,
    );
    details.cancel();

    assert!(details.is_canceled());
}

#[test]
fn open_change_details_can_cancel() {
    let mut details = SelectOpenChangeDetails::new(
        SelectOpenChangeReason::TriggerPress,
        SelectOpenChangeSource::Keyboard,
        true,
    );
    details.cancel();

    assert!(details.is_canceled());
}

#[test]
fn disabled_item_activation_is_noop() {
    let runtime = SelectRuntime::new(SelectSelectionMode::Single, None::<&str>, Vec::new(), false);
    let outcome = runtime.request_select_value(None, Vec::new(), "apple", true, false, false);

    assert!(!outcome.changed());
}

#[test]
fn multiple_selection_toggles_values_without_duplicates() {
    let runtime = SelectRuntime::new(
        SelectSelectionMode::Multiple,
        None::<&str>,
        vec!["apple"],
        true,
    );
    let outcome = runtime.request_select_value(None, vec!["apple"], "banana", false, false, false);
    let values = match outcome {
        SelectSelectionChange::Multiple(outcome) => outcome.into_values(),
        _ => panic!("expected multiple outcome"),
    };
    assert_eq!(values, vec!["apple", "banana"]);

    let runtime = SelectRuntime::new(SelectSelectionMode::Multiple, None::<&str>, values, true);
    let outcome =
        runtime.request_select_value(None, vec!["apple", "banana"], "apple", false, false, false);
    let values = match outcome {
        SelectSelectionChange::Multiple(outcome) => outcome.into_values(),
        _ => panic!("expected multiple outcome"),
    };
    assert_eq!(values, vec!["banana"]);
}

#[test]
fn item_registration_order_and_navigation_allow_disabled_items() {
    let mut runtime = SelectRuntime::new(SelectSelectionMode::Single, None, Vec::new(), true);
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("first", Some("First".into()), false, 2),
            SelectItemMetadata::new("disabled", Some("Disabled".into()), true, 0),
            SelectItemMetadata::new("second", Some("Second".into()), false, 1),
        ],
        Vec::new(),
        None,
        false,
        None,
    );
    runtime.reconcile_single(None, true);

    assert_eq!(runtime.highlighted_value(), Some("second"));
    runtime.move_highlight(SelectMove::Previous, true);
    assert_eq!(
        runtime.highlighted_value_with_disabled(),
        Some(("disabled", true))
    );
    assert!(!runtime
        .request_select_value(None, Vec::new(), "disabled", true, false, false)
        .changed());
}

#[test]
fn dynamic_item_removal_falls_back_to_none() {
    let mut runtime = SelectRuntime::new(
        SelectSelectionMode::Single,
        Some("missing"),
        Vec::new(),
        true,
    );
    runtime.sync_children(
        vec![SelectItemMetadata::new(
            "present",
            Some("Present".into()),
            false,
            0,
        )],
        Vec::new(),
        None,
        false,
        None,
    );
    runtime.reconcile_single(runtime.selected_value(), true);

    assert_eq!(runtime.selected_value(), None);
}

#[test]
fn repeated_typeahead_characters_cycle_matches() {
    let now = Instant::now();
    let mut runtime = SelectRuntime::new(SelectSelectionMode::Single, None, Vec::new(), true);
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("apple", Some("Apple".into()), false, 0),
            SelectItemMetadata::new("apricot", Some("Apricot".into()), false, 1),
            SelectItemMetadata::new("banana", Some("Banana".into()), false, 2),
        ],
        Vec::new(),
        None,
        false,
        None,
    );

    assert_eq!(
        runtime.apply_typeahead("a", now),
        SelectTypeaheadOutcome::Highlighted("apple")
    );
    assert_eq!(
        runtime.apply_typeahead("a", now + Duration::from_millis(100)),
        SelectTypeaheadOutcome::Highlighted("apricot")
    );
}

#[test]
fn typeahead_resets_after_timeout() {
    let now = Instant::now();
    let mut runtime = SelectRuntime::new(SelectSelectionMode::Single, None, Vec::new(), true);
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("apple", Some("Apple".into()), false, 0),
            SelectItemMetadata::new("banana", Some("Banana".into()), false, 1),
        ],
        Vec::new(),
        None,
        false,
        None,
    );

    assert_eq!(
        runtime.apply_typeahead("b", now),
        SelectTypeaheadOutcome::Highlighted("banana")
    );
    assert_eq!(
        runtime.apply_typeahead("a", now + Duration::from_millis(1_100)),
        SelectTypeaheadOutcome::Highlighted("apple")
    );
}

#[test]
fn typeahead_is_disabled_in_multiple_mode() {
    let mut runtime = SelectRuntime::new(
        SelectSelectionMode::Multiple,
        None::<&str>,
        Vec::new(),
        true,
    );
    runtime.sync_children(
        vec![SelectItemMetadata::new(
            "apple",
            Some("Apple".into()),
            false,
            0,
        )],
        Vec::new(),
        None,
        false,
        None,
    );

    assert_eq!(
        runtime.apply_typeahead("a", Instant::now()),
        SelectTypeaheadOutcome::Disabled
    );
}

#[test]
fn closed_typeahead_returns_commit_outcome() {
    let mut runtime = SelectRuntime::new(SelectSelectionMode::Single, None, Vec::new(), false);
    runtime.sync_children(
        vec![SelectItemMetadata::new(
            "apple",
            Some("Apple".into()),
            false,
            0,
        )],
        Vec::new(),
        None,
        false,
        None,
    );

    assert_eq!(
        runtime.apply_typeahead("a", Instant::now()),
        SelectTypeaheadOutcome::Commit("apple")
    );
}

#[test]
fn typeahead_highlights_matching_enabled_item() {
    let mut runtime = SelectRuntime::new(SelectSelectionMode::Single, None, Vec::new(), true);
    runtime.sync_children(
        vec![
            SelectItemMetadata::new("apple", Some("Apple".into()), false, 0),
            SelectItemMetadata::new("apricot", Some("Apricot".into()), true, 1),
            SelectItemMetadata::new("banana", Some("Banana".into()), false, 2),
        ],
        Vec::new(),
        None,
        false,
        None,
    );

    assert_eq!(runtime.highlight_by_typeahead("ba"), Some("banana"));
    assert_eq!(runtime.highlighted_value(), Some("banana"));
}
