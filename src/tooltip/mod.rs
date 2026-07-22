//! Accessibility: the trigger is exposed as `Role::Button` (with an optional
//! `TooltipTrigger::aria_label`) and the open popup as `Role::Tooltip`; all
//! other layers stay out of the accessibility tree, matching Base UI's
//! ARIA-light tooltip. Known gaps in the pinned gpui revision:
//! - `disabled`: no `.aria_disabled(...)` builder exists, so AT cannot see a
//!   disabled trigger state. Disabled triggers are removed from tab order and
//!   omit the Click a11y action instead; blocked pending an upstream gpui
//!   `set_disabled` addition.
//! - `aria-describedby` trigger→popup relationship: no relationship builders
//!   exist. Base UI does not emit this for Tooltip either; tooltip content is
//!   not an accessible description or name for the trigger.
//! - Live-region announcements of tooltip content on open: no announcement
//!   API exists, so tooltips remain sighted-user visual hints.

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{init, TooltipCloseAction, TOOLTIP_KEY_CONTEXT};
pub use child::{TooltipChild, TooltipPopupChild, TooltipPortalChild, TooltipPositionerChild};
pub use context::{create_tooltip_handle, TooltipContext, TooltipHandle};
pub use layers::{
    TooltipPopup, TooltipPortal, TooltipPositioner, TooltipProvider, TooltipProviderChild,
    TooltipRoot, TooltipTrigger, TooltipViewport,
};
pub use props::{
    TooltipAlign, TooltipDelayGroup, TooltipInstant, TooltipOpenChangeCompleteHandler,
    TooltipOpenChangeHandler, TooltipPayloadContentBuilder, TooltipProps, TooltipProviderConfig,
    TooltipSide, TooltipTrackCursorAxis, DEFAULT_TOOLTIP_CLOSE_DELAY, DEFAULT_TOOLTIP_DELAY,
    DEFAULT_TOOLTIP_TIMEOUT,
};
pub use runtime::{
    scoped_trigger_id, TooltipActivationDirection, TooltipBoundsKind, TooltipFocusChange,
    TooltipHoverTarget, TooltipOpenChangeDetails, TooltipOpenChangeOutcome,
    TooltipOpenChangeReason, TooltipOpenChangeSource, TooltipRuntime, TooltipTriggerMetadata,
};
pub use style_state::{
    TooltipPopupStyleState, TooltipPortalStyleState, TooltipPositionerStyleState,
    TooltipProviderStyleState, TooltipRootStyleState, TooltipTriggerStyleState,
    TooltipViewportStyleState,
};
