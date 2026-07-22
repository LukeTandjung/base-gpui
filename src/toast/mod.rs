//! Toast: stacked, auto-dismissing notifications.
//!
//! AccessKit gap (blocked pending gpui upstream): this gpui revision exposes
//! no live-region/announcement API (`aria-live`, `aria-atomic`,
//! `aria-relevant`, or Base UI's hidden duplicated `role="alert"` tree), so
//! added toasts are **not** auto-announced to screen readers. The viewport is
//! exposed as a labeled `Role::Region` ("Notifications") that AT users must
//! discover; see `docs/accesskit-gpui-reference.md`.

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod manager;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, ToastCloseAction, ToastExitViewportAction, ToastFocusViewportAction,
    TOAST_ROOT_KEY_CONTEXT, TOAST_VIEWPORT_KEY_CONTEXT,
};
pub use child::{ToastPortalChild, ToastProviderChild, ToastRootChild};
pub use context::{ToastContext, ToastPromiseOptions, ToastPromiseResolve, WeakToastContext};
pub use layers::{
    ToastAction, ToastClose, ToastContent, ToastDescription, ToastPortal, ToastProvider, ToastRoot,
    ToastTitle, ToastViewport,
};
pub use manager::{create_toast_manager, ToastManager};
pub use props::ToastProviderProps;
pub use runtime::{
    ToastActionDef, ToastActionHandler, ToastAddOutcome, ToastCallback, ToastCloseOutcome,
    ToastFacts, ToastId, ToastOptions, ToastRuntime, ToastSwipeRelease, ToastTimerOp,
    TOAST_DEFAULT_LIMIT, TOAST_DEFAULT_TIMEOUT,
};
pub use style_state::{
    ToastActionStyleState, ToastCloseStyleState, ToastContentStyleState,
    ToastDescriptionStyleState, ToastPriority, ToastRootStyleState, ToastSwipeDirection,
    ToastTitleStyleState, ToastTransitionStatus, ToastType, ToastViewportStyleState,
};
