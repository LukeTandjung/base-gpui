//! GPUI port of Base UI Select.
//!
//! # AccessKit gaps in the pinned gpui revision
//!
//! The a11y wiring covers `Role::ComboBox` + `aria_expanded`/`aria_label` on
//! the trigger (plus `Expand`/`Collapse` a11y actions), `Role::ListBox` on the
//! list, `Role::ListBoxOption` + `aria_selected`/`aria_position_in_set`/
//! `aria_size_of_set`/`aria_label` on items, and `Role::Group` + `aria_label`
//! on groups. Base UI ARIA features with no gpui builder are omitted rather
//! than faked:
//!
//! - `aria-haspopup="listbox"` (trigger): no builder; `Role::ComboBox` +
//!   `aria_expanded` is the closest available signal.
//! - `aria-controls` (trigger → listbox): no id-reference builders; blocked
//!   pending gpui upstream support.
//! - `aria-labelledby` (trigger and group): substituted with literal
//!   `.aria_label(...)` strings.
//! - `aria-activedescendant` (highlighted item): items carry real focus via
//!   `track_focus`, so assistive technology tracks the focused option instead.
//! - `aria-multiselectable` (list in multiple mode): no builder;
//!   `SelectRootStyleState::selection_mode` remains style-state only.
//! - `aria-readonly` / `aria-required` / `aria-disabled` (trigger, items): no
//!   builders and gpui never sets a disabled flag; interactions stay
//!   behavioral no-ops while disabled/read-only.
//! - Value announcement on change (`SelectValue` live text): no
//!   live-region/announcement API; blocked pending gpui upstream.

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
mod key;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, SelectActivateHighlighted, SelectClose, SelectMoveFirst, SelectMoveLast, SelectMoveNext,
    SelectMovePrevious, SelectOpen, SelectToggleOpen, SELECT_KEY_CONTEXT,
};
pub use child::{
    SelectChild, SelectGroupChild, SelectItemChild, SelectListChild, SelectPopupChild,
    SelectPortalChild, SelectPositionerChild, SelectTriggerChild,
};
pub use context::SelectContext;
pub use layers::{
    SelectArrow, SelectBackdrop, SelectGroup, SelectGroupLabel, SelectIcon, SelectItem,
    SelectItemIndicator, SelectItemText, SelectLabel, SelectList, SelectPopup, SelectPortal,
    SelectPositioner, SelectRoot, SelectScrollDownArrow, SelectScrollUpArrow, SelectSeparator,
    SelectTrigger, SelectValue,
};
pub use props::{
    SelectAlign, SelectLabelResolver, SelectMultipleValueFormatter, SelectOpenChangeHandler,
    SelectProps, SelectSelectionMode, SelectSide, SelectValueChangeHandler, SelectValueComparator,
    SelectValueSerializer, SelectValuesChangeHandler,
};
pub use runtime::{
    SelectGroupMetadata, SelectItemMetadata, SelectMove, SelectMultipleValuesChangeOutcome,
    SelectOpenChangeDetails, SelectOpenChangeOutcome, SelectOpenChangeReason,
    SelectOpenChangeSource, SelectRuntime, SelectSelectionChange, SelectSingleValueChangeOutcome,
    SelectTypeaheadOutcome, SelectValueChangeDetails, SelectValueChangeReason,
    SelectValueChangeSource,
};
pub use style_state::{
    SelectArrowStyleState, SelectBackdropStyleState, SelectGroupLabelStyleState,
    SelectGroupStyleState, SelectIconStyleState, SelectItemIndicatorStyleState,
    SelectItemStyleState, SelectItemTextStyleState, SelectListStyleState, SelectPopupStyleState,
    SelectPortalStyleState, SelectPositionerStyleState, SelectRootStyleState,
    SelectScrollArrowDirection, SelectScrollArrowStyleState, SelectSeparatorStyleState,
    SelectTriggerStyleState, SelectValueStyleState,
};
