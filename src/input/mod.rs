//! Field-aware Base UI `Input` component.
//!
//! # Accessibility
//!
//! The rendered control appears in the AccessKit tree as `Role::TextInput`
//! (set on the primitive input layer) with `Action::Focus` auto-registered
//! via focus tracking. Set an accessible name with [`Input::aria_label`];
//! there is no `aria-labelledby` id wiring in this gpui revision, so when
//! using `FieldLabel` pass the same label text to `.aria_label(...)`.
//!
//! Known AccessKit gaps in this gpui revision (each omit-and-document or
//! blocked pending gpui upstream support):
//! - `disabled` / `aria-disabled`: not surfaced to AT; the control only
//!   drops `tab_index` and edit/mouse listeners while disabled.
//! - `aria-readonly`, `aria-required`, `aria-placeholder`,
//!   `aria-multiline`: no builders; `read_only`/`required`/`placeholder`
//!   stay behavior/style-only.
//! - `aria-invalid`: no builder; `InputStyleState::invalid` remains a
//!   visual-only signal.
//! - Label/description association (`aria-labelledby`/`aria-describedby`):
//!   no relationship builders; only the literal `.aria_label(...)` string.
//! - Text value, caret, and selection reporting: no gpui builder, so AT
//!   cannot read the input's contents or caret position yet.

mod layers;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use layers::Input;
pub use style_state::InputStyleState;
