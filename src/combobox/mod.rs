//! # AccessKit accessibility notes
//!
//! Roles/aria props and a11y actions are wired per the port issue's follow-up
//! plan. The following Base UI ARIA attributes have **no gpui builder** in the
//! pinned revision and are intentionally omitted:
//!
//! - `aria-activedescendant` — highlight-following announcement while focus
//!   stays on the input is **blocked pending gpui upstream**; options keep
//!   `aria-selected` accurate as the fallback.
//! - `aria-controls`, `aria-labelledby`, `aria-describedby` — no id-reference
//!   wiring; literal-string `.aria_label(...)` props substitute (input,
//!   trigger, clear, group).
//! - `aria-haspopup="listbox"` — omitted; `Role::ComboBox` + `aria_expanded`
//!   convey the affordance.
//! - `aria-autocomplete` — omitted (matters more for the Autocomplete port).
//! - `aria-multiselectable` on the listbox — omitted; multiple-mode state is
//!   conveyed per-option via `aria-selected`.
//! - `disabled` / `aria-disabled` / `aria-readonly` / `aria-required` —
//!   never announced; disabled parts skip registering a11y action handlers,
//!   but the disabled state itself is **blocked pending gpui upstream**.
//! - Live regions (`ComboboxStatus` / `ComboboxEmpty`, `aria-live="polite"`)
//!   — no announcement API; they stay plain visual containers (**blocked
//!   pending gpui upstream**, shared gap with Toast/Form errors).
//! - Grid roles — deferred with grid mode itself.
pub mod actions;
pub mod child;
pub mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, ComboboxEscape, ComboboxMoveNext, ComboboxMovePrevious, COMBOBOX_KEY_CONTEXT,
};
pub use child::{
    ComboboxChild, ComboboxChipChild, ComboboxGroupChild, ComboboxInputGroupChild,
    ComboboxItemChild, ComboboxListChild, ComboboxPopupChild, ComboboxPortalChild,
    ComboboxPositionerChild,
};
pub use context::ComboboxContext;
pub use layers::{
    ComboboxArrow, ComboboxBackdrop, ComboboxChip, ComboboxChipRemove, ComboboxChips,
    ComboboxClear, ComboboxCollection, ComboboxEmpty, ComboboxGroup, ComboboxGroupLabel,
    ComboboxIcon, ComboboxInput, ComboboxInputGroup, ComboboxItem, ComboboxItemIndicator,
    ComboboxLabel, ComboboxList, ComboboxPopup, ComboboxPortal, ComboboxPositioner, ComboboxRoot,
    ComboboxSeparator, ComboboxStatus, ComboboxTrigger, ComboboxValue,
};
pub use props::{
    ComboboxAlign, ComboboxAutoHighlight, ComboboxFilter, ComboboxInputValueChangeHandler,
    ComboboxItemHighlightedHandler, ComboboxLabelResolver, ComboboxMultipleValueFormatter,
    ComboboxOpenChangeHandler, ComboboxProps, ComboboxSelectionMode, ComboboxSide,
    ComboboxValueChangeHandler, ComboboxValueSerializer, ComboboxValuesChangeHandler,
};
pub use runtime::{
    ComboboxChangeDetails, ComboboxChangeReason, ComboboxChangeSource, ComboboxChipMoveOutcome,
    ComboboxGroupMetadata, ComboboxHighlightReason, ComboboxItemHighlightDetails,
    ComboboxItemMetadata, ComboboxMove, ComboboxOpenChangeOutcome, ComboboxRuntime,
    ComboboxSelectionChange,
};
pub use style_state::{
    ComboboxArrowStyleState, ComboboxBackdropStyleState, ComboboxChipRemoveStyleState,
    ComboboxChipStyleState, ComboboxChipsStyleState, ComboboxClearStyleState,
    ComboboxEmptyStyleState, ComboboxGroupLabelStyleState, ComboboxGroupStyleState,
    ComboboxIconStyleState, ComboboxInputGroupStyleState, ComboboxInputStyleState,
    ComboboxItemIndicatorStyleState, ComboboxItemStyleState, ComboboxLabelStyleState,
    ComboboxListStyleState, ComboboxPopupStyleState, ComboboxPortalStyleState,
    ComboboxPositionerStyleState, ComboboxRootStyleState, ComboboxSeparatorStyleState,
    ComboboxStatusStyleState, ComboboxTriggerStyleState, ComboboxValueStyleState,
};
