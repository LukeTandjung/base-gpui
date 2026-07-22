//! # Accessibility notes (AccessKit follow-up)
//!
//! Base UI Field's ARIA wiring is id-reference based and this gpui revision
//! has no relationship builders, so parts of it are intentionally omitted
//! (see `issues/port-baseui-field.md`, "AccessKit accessibility follow-up"):
//!
//! - `aria-labelledby`: replaced by literal label-text plumbing —
//!   `FieldLabel::text(...)` registers the label text on `FieldRuntime`, and
//!   registered controls can read it via `FieldContext::label_text(...)` to
//!   set `.aria_label(...)` on their interactive element.
//! - `aria-describedby` (description/error message ids): omitted; the
//!   `FieldDescription`/`FieldError` text remains visible text only.
//! - `aria-invalid`: omitted; no gpui builder. `FieldValidityData` exposes
//!   the state for styling.
//! - `aria-required` / `required`: omitted; preserved as
//!   `FieldControlRegistration` metadata for a future gpui addition.
//! - `disabled` / `aria-disabled`: omitted; disabled cascading suppresses
//!   interaction but AT is not told the control is disabled.
//! - Live announcement of `FieldError` appearing: blocked pending a gpui
//!   announcement/live-region API.
//!
//! No Field part carries a role of its own (matching Base UI's generic
//! `<div>`/`<p>` output); the wrapped control is the only part meant to
//! appear in the a11y tree.

pub mod child;
mod child_wiring;
pub mod context;
pub mod item_context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;
pub mod validation;

#[cfg(test)]
mod tests;

pub use child::{FieldChild, FieldItemChild};
pub use context::{current_field_context, FieldContext};
pub use item_context::current_field_item_disabled;
pub use layers::{
    FieldControl, FieldDescription, FieldError, FieldItem, FieldLabel, FieldRoot, FieldValidity,
};
pub use props::{FieldProps, FieldValidationHandler};
pub use runtime::{FieldControlRegistration, FieldRuntime};
pub use style_state::{
    FieldDescriptionStyleState, FieldErrorStyleState, FieldItemStyleState, FieldLabelStyleState,
    FieldRootStyleState, FieldValidityStyleState,
};
pub use validation::{
    FieldErrorMatch, FieldValidationMode, FieldValidationResult, FieldValidityData,
    FieldValidityKey, FieldValidityState, FieldValue,
};
