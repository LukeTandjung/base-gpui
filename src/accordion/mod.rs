//! Accessible collapsible-panel component ported from Base UI's Accordion.
//!
//! # Accessibility gaps (blocked pending gpui upstream support)
//!
//! The pinned gpui revision has no AccessKit builders for these Base UI ARIA
//! attributes, so they are intentionally omitted:
//!
//! - `aria-controls` (trigger → panel): no relationship builder. AT users still
//!   get the essential disclosure pattern via `Role::Button` + `aria_expanded`.
//! - `aria-labelledby` (panel ← trigger): no relationship builder. Callers may
//!   set a literal label on the panel instead once gpui exposes one.
//! - `disabled` / `aria-disabled`: no disabled-state builder. Disabled triggers
//!   keep `tab_stop(false)` / `tab_index(-1)` and the runtime rejects toggles,
//!   but assistive technology will not announce them as dimmed/disabled.

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

pub use actions::{init, AccordionToggle, ACCORDION_TRIGGER_KEY_CONTEXT};
pub use child::{AccordionHeaderChild, AccordionItemChild, AccordionRootChild};
pub use context::{AccordionContext, AccordionItemContext};
pub use layers::{AccordionHeader, AccordionItem, AccordionPanel, AccordionRoot, AccordionTrigger};
pub use props::{AccordionItemOpenChangeHandler, AccordionProps, AccordionValueChangeHandler};
pub use runtime::{
    AccordionChangeDetails, AccordionChangeReason, AccordionChangeSource, AccordionItemMetadata,
    AccordionItemOpenChangeDetails, AccordionRuntime, AccordionToggleOutcome,
    AccordionValueChangeDetails,
};
pub use style_state::{
    AccordionHeaderStyleState, AccordionItemStyleState, AccordionOrientation,
    AccordionPanelStyleState, AccordionRootStyleState, AccordionTriggerStyleState,
};
