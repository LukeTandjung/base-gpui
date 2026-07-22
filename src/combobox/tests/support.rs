use gpui::SharedString;

use crate::combobox::{ComboboxItemMetadata, ComboboxRuntime, ComboboxSelectionMode};

pub fn fruit_items() -> Vec<ComboboxItemMetadata<&'static str>> {
    vec![
        ComboboxItemMetadata::new("apple", Some("Apple".into()), false, 0),
        ComboboxItemMetadata::new("banana", Some("Banana".into()), false, 1),
        ComboboxItemMetadata::new("cherry", Some("Cherry".into()), true, 2),
        ComboboxItemMetadata::new("blueberry", Some("Blueberry".into()), false, 3),
    ]
}

pub fn single_runtime(
    selected: Option<&'static str>,
    input: impl Into<SharedString>,
    open: bool,
) -> ComboboxRuntime<&'static str> {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Single,
        selected,
        Vec::new(),
        input.into(),
        open,
    );
    runtime.sync_children(fruit_items(), None, false);
    runtime
}

pub fn multiple_runtime(
    values: Vec<&'static str>,
    input: impl Into<SharedString>,
    open: bool,
) -> ComboboxRuntime<&'static str> {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::Multiple,
        None,
        values,
        input.into(),
        open,
    );
    runtime.sync_children(fruit_items(), None, false);
    runtime
}
