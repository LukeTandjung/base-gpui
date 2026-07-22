pub mod child;
pub mod layers;

#[cfg(test)]
mod tests;

pub use child::AutocompleteChild;
pub use layers::{AutocompleteMode, AutocompleteRoot, AutocompleteValue};

// Reused Combobox parts, re-exported under Autocomplete names (Base UI
// `autocomplete/index.parts.ts` re-export map). `ComboboxLabel`,
// `ComboboxValue`, `ComboboxItemIndicator`, and the chips parts are
// deliberately not re-exported.
pub use crate::combobox::{
    ComboboxArrow as AutocompleteArrow, ComboboxBackdrop as AutocompleteBackdrop,
    ComboboxClear as AutocompleteClear, ComboboxCollection as AutocompleteCollection,
    ComboboxEmpty as AutocompleteEmpty, ComboboxGroup as AutocompleteGroup,
    ComboboxGroupLabel as AutocompleteGroupLabel, ComboboxIcon as AutocompleteIcon,
    ComboboxInput as AutocompleteInput, ComboboxInputGroup as AutocompleteInputGroup,
    ComboboxItem as AutocompleteItem, ComboboxList as AutocompleteList,
    ComboboxPopup as AutocompletePopup, ComboboxPortal as AutocompletePortal,
    ComboboxPositioner as AutocompletePositioner, ComboboxSeparator as AutocompleteSeparator,
    ComboboxStatus as AutocompleteStatus, ComboboxTrigger as AutocompleteTrigger,
};
