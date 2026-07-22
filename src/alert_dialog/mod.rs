//! Alert Dialog component ported from Base UI, composing the Dialog module.
//!
//! # Accessibility gaps (blocked pending gpui upstream)
//!
//! The pinned gpui revision exposes no builders for the following Base UI ARIA
//! attributes, so they are intentionally omitted (see `dialog/mod.rs` for the
//! shared Dialog gaps, all inherited here):
//!
//! - `aria-haspopup="dialog"` on the trigger; `Role::Button` +
//!   `aria_expanded` is the best available signal.
//! - `aria-controls` (trigger → popup) — no id-reference/relationship builders.
//! - `aria-labelledby` / `aria-describedby` (popup → title/description) —
//!   fallback is the literal [`AlertDialogPopup::aria_label`] title string.
//! - `disabled` / `aria-disabled` on trigger and close — the `disabled` guards
//!   suppress the runtime transition, but AT is not told the control is disabled.
//! - `aria-hidden` on outside content while modal — the focus trap is the only
//!   modality signal.

pub mod actions;
pub mod handle;
pub mod layers;

#[cfg(test)]
mod tests;

pub use crate::dialog::{
    DialogBackdrop as AlertDialogBackdrop, DialogClose as AlertDialogClose,
    DialogDescription as AlertDialogDescription, DialogPortal as AlertDialogPortal,
    DialogTitle as AlertDialogTitle, DialogViewport as AlertDialogViewport,
};
pub use actions::init;
pub use handle::{create_alert_dialog_handle, AlertDialogHandle};
pub use layers::{AlertDialogPopup, AlertDialogRoot, AlertDialogTrigger};
