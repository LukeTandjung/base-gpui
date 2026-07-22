//! OTP Field: a group of one-character slots editing a single OTP string.
//!
//! Accessibility (AccessKit): the root is `Role::Group` with a literal
//! `.aria_label(...)` prop; each slot is `Role::TextInput` with
//! `aria_position_in_set` / `aria_size_of_set`, `aria_selected` for the
//! virtual active slot, a per-slot label that never leaks masked characters,
//! and a `Click` a11y action routed through the same slot-activation command
//! as the pointer path. Root `SetValue` routes through the paste path.
//!
//! Known accessibility gaps in this gpui revision (blocked pending gpui
//! upstream, do not fake): no `aria-labelledby` / `aria-describedby`
//! relationship wiring to `FieldLabel` / `FieldDescription` / `FieldError`
//! (mirror the label text via the root `.aria_label(...)` instead); no
//! disabled / read-only / required node properties (a disabled field only
//! guards its `Click` action); no `aria-invalid`; no live-region API, so
//! completion and Field error text are visual-only.

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
mod layers;
pub mod otp;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, OTPFieldBackspace, OTPFieldClear, OTPFieldDelete, OTPFieldFirst, OTPFieldLast,
    OTPFieldNext, OTPFieldPrevious, OTP_FIELD_KEY_CONTEXT,
};
pub use child::OTPFieldChild;
pub use context::OTPFieldContext;
pub use layers::{OTPFieldInput, OTPFieldRoot};
pub use otp::{
    clamp_code_points, code_point_count, filter_validation, normalize_otp_value, remove_at_index,
    replace_at_index, strip_whitespace, OTPFieldNormalizeValueHandler, OTPFieldValidationType,
    OTPNormalizedValue, OTPReplaceOutcome,
};
pub use props::{
    OTPFieldProps, OTPFieldValueChangeHandler, OTPFieldValueCompleteHandler,
    OTPFieldValueInvalidHandler,
};
pub use runtime::{
    OTPFieldChangeDetails, OTPFieldChangeReason, OTPFieldMove, OTPFieldRuntime,
    OTPFieldUpdateOutcome, OTPFieldValueChange,
};
pub use style_state::{OTPFieldInputStyleState, OTPFieldRootStyleState};

// Matches Base UI's `index.parts.ts`: the OTP Field family re-exports Separator.
pub use crate::separator::Separator;
