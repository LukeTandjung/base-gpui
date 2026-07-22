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

/// A Combobox runtime configured the way `AutocompleteRoot` configures it:
/// `selection_mode = None`, no selection state.
pub fn none_mode_runtime(
    input: impl Into<SharedString>,
    open: bool,
) -> ComboboxRuntime<&'static str> {
    let mut runtime = ComboboxRuntime::new(
        ComboboxSelectionMode::None,
        None,
        Vec::new(),
        input.into(),
        open,
    );
    runtime.sync_children(fruit_items(), None, false);
    runtime
}
