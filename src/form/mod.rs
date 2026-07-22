//! GPUI-native port of Base UI `Form`.
//!
//! Accessibility notes: the form container exposes `Role::Form` plus an
//! optional literal-string `.aria_label(...)` (gpui has no `aria-labelledby`
//! id-reference wiring). Per-field `aria-invalid`, `aria-required`, and
//! `aria-describedby` (error text -> control association) have no builders in
//! the pinned gpui AccessKit surface, so submit-blocking validity is not yet
//! visible to assistive technology; this gap is tracked in the Field issue
//! (`issues/port-baseui-field.md`), not silently absent. Live-region
//! announcement of submit failure is likewise unavailable; the focus move to
//! the first invalid control is the only AT signal today. Submit triggers rely
//! on their own auto-registered `on_click` Click action — the Form container
//! itself registers no a11y actions and is not focusable.

pub mod actions;
pub mod context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;
pub mod submit;
pub mod value;

#[cfg(test)]
mod tests;

pub use actions::{init, FormSubmitAction, FormValidateAction, FORM_KEY_CONTEXT};
pub use context::{current_form_context, FormContext};
pub use layers::Form;
pub use props::{FormErrors, FormProps, FormSubmitHandler};
pub use runtime::{
    FormFieldRegistration, FormFieldSnapshot, FormFieldValidationHandler, FormRuntime,
    FormSubmissionResult,
};
pub use style_state::FormStyleState;
pub use submit::{FormSubmitDetails, FormSubmitReason};
pub use value::{FormValue, FormValues};
