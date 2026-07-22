use gpui::SharedString;

use crate::combobox::{
    ComboboxItemMetadata, ComboboxProps, ComboboxRuntime, ComboboxSelectionMode,
};

#[test]
fn items_are_ordered_by_registration_index() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        None::<&str>,
        Vec::new(),
        SharedString::default(),
        true,
    );
    runtime.sync_children(
        vec![
            ComboboxItemMetadata::new("banana", Some("Banana".into()), false, 1),
            ComboboxItemMetadata::new("apple", Some("Apple".into()), false, 0),
        ],
        None,
        false,
    );

    assert_eq!(runtime.filtered_indices(), vec![0, 1]);
}

#[test]
fn label_resolver_backfills_missing_registered_labels() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        None::<&str>,
        Vec::new(),
        SharedString::default(),
        true,
    );
    runtime.sync_filtering(
        None,
        false,
        Some(std::rc::Rc::new(|value: &&str| {
            SharedString::from(value.to_uppercase())
        })),
        None,
        crate::combobox::ComboboxAutoHighlight::Off,
    );
    runtime.sync_children(
        vec![ComboboxItemMetadata::new("apple", None, false, 0)],
        None,
        false,
    );
    runtime.commit_input_value("APP".into());

    assert_eq!(runtime.filtered_indices(), vec![0]);
    assert_eq!(
        runtime.label_for_value(&"apple"),
        Some(SharedString::from("APPLE"))
    );
}

#[test]
fn group_metadata_is_sorted_and_labelled() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        None::<&str>,
        Vec::new(),
        SharedString::default(),
        false,
    );
    let mut group_one = crate::combobox::ComboboxGroupMetadata::new(1);
    group_one.set_label(Some("Berries".into()));
    runtime.sync_groups(vec![
        group_one,
        crate::combobox::ComboboxGroupMetadata::new(0),
    ]);

    let state = runtime.group_state(Some(1), 2);
    assert_eq!(state.label, Some(SharedString::from("Berries")));
    assert_eq!(runtime.group_metadata()[0].index(), 0);
}

#[test]
fn item_state_reports_visibility_from_the_filtered_set() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        None::<&str>,
        Vec::new(),
        SharedString::default(),
        true,
    );
    runtime.sync_children(
        vec![
            ComboboxItemMetadata::new("apple", Some("Apple".into()), false, 0),
            ComboboxItemMetadata::new("banana", Some("Banana".into()), false, 1),
        ],
        None,
        false,
    );
    runtime.commit_input_value("ban".into());
    let props = ComboboxProps::<&str>::new();

    assert!(
        !runtime
            .item_state(Some(&"apple"), false, Some(0), &props)
            .visible
    );
    assert!(
        runtime
            .item_state(Some(&"banana"), false, Some(1), &props)
            .visible
    );
}

#[test]
fn highlight_survives_children_resync_when_value_persists() {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        None::<&str>,
        Vec::new(),
        SharedString::default(),
        true,
    );
    let items = vec![
        ComboboxItemMetadata::new("apple", Some("Apple".into()), false, 0),
        ComboboxItemMetadata::new("banana", Some("Banana".into()), false, 1),
    ];
    runtime.sync_children(items.clone(), None, false);
    runtime.highlight_item(Some(1), false);

    // Re-registration with shifted indices keeps the highlighted value.
    let shifted = vec![
        ComboboxItemMetadata::new("banana", Some("Banana".into()), false, 0),
        ComboboxItemMetadata::new("apple", Some("Apple".into()), false, 1),
    ];
    runtime.sync_children(shifted, None, false);
    assert_eq!(runtime.highlighted_value(), Some("banana"));
}
