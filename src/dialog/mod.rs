//! Dialog component ported from Base UI.
//!
//! # Accessibility gaps (blocked pending gpui upstream)
//!
//! The pinned gpui revision exposes no builders for the following Base UI ARIA
//! attributes, so they are intentionally omitted:
//!
//! - `aria-haspopup="dialog"` on the trigger; `Role::Dialog` on the popup conveys
//!   the destination once open.
//! - `aria-controls` (trigger → popup) — no id-reference/relationship builders.
//! - `aria-labelledby` / `aria-describedby` (popup → title/description) — fallback
//!   is the literal [`layers::DialogPopup::aria_label`] string; the runtime's
//!   title/description id metadata is retained for a future relationship API.
//! - `disabled` / `aria-disabled` on trigger and close — the existing `disabled`
//!   guards suppress the runtime transition (AT clicks are no-ops), but the
//!   disabled state itself is not conveyed to assistive technology.
//! - `aria-modal` / outside-content inertness — `DialogModalMode::Modal` blocks
//!   pointer input and traps Tab focus, but does not hide outside content from
//!   the AccessKit tree.
//! - Kept-mounted closed popup content is left role-less so it stays out of the
//!   accessibility tree without a hidden/inert API.

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
    init, DialogCloseAction, DialogFocusNextAction, DialogFocusPreviousAction, DialogOpenAction,
    DIALOG_POPUP_KEY_CONTEXT, DIALOG_TRIGGER_KEY_CONTEXT,
};
pub use child::{DialogChild, DialogPopupChild, DialogPortalChild, DialogViewportChild};
pub use context::{create_dialog_handle, DialogContext, DialogHandle};
pub use layers::{
    DialogBackdrop, DialogClose, DialogDescription, DialogPayloadContentBuilder, DialogPopup,
    DialogPortal, DialogRoot, DialogTitle, DialogTrigger, DialogViewport,
};
pub use props::{DialogOpenChangeCompleteHandler, DialogOpenChangeHandler, DialogProps};
pub use runtime::{
    scoped_dialog_part_id, scoped_dialog_trigger_id, DialogOpenChangeDetails,
    DialogOpenChangeOutcome, DialogOpenChangeReason, DialogOpenChangeSource, DialogRuntime,
    DialogTriggerMetadata,
};
pub use style_state::{
    DialogBackdropStyleState, DialogCloseStyleState, DialogDescriptionStyleState, DialogModalMode,
    DialogPopupStyleState, DialogPortalStyleState, DialogRootStyleState, DialogTitleStyleState,
    DialogTriggerStyleState, DialogViewportStyleState,
};
