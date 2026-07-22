//! Toolbar component family ported from Base UI.
//!
//! # Accessibility notes
//!
//! - The root exposes `Role::Toolbar` + `aria_orientation`, the group
//!   `Role::Group`, and items `Role::Button` / `Role::Link`; the reused
//!   `Separator` owns the separator node and the reused `Input` owns the
//!   text-input node.
//! - `disabled` gap: this gpui revision has no disabled a11y builder, so the
//!   merged disabled state (`toolbar || group || item`) is exposed only
//!   through the typed style states. Disabled items keep their (inert) Click
//!   handlers registered, so assistive technology sees a button whose click
//!   does nothing rather than a vanishing node.
//! - `aria-haspopup` gap: no gpui builder exists; trigger-hosting buttons
//!   (menu/select/dialog triggers inside `ToolbarButton`) cannot advertise a
//!   popup until gpui gains one upstream. Blocked pending that addition.

mod actions;
mod child;
mod child_wiring;
mod context;
mod layers;
mod props;
mod runtime;
mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, ToolbarActivateFocused, ToolbarFocusDown, ToolbarFocusLeft, ToolbarFocusRight,
    ToolbarFocusUp, TOOLBAR_ITEM_KEY_CONTEXT, TOOLBAR_KEY_CONTEXT,
};
pub use child::{ToolbarChild, ToolbarGroupChild};
pub use context::ToolbarContext;
pub use layers::{
    ToolbarButton, ToolbarGroup, ToolbarInput, ToolbarLink, ToolbarRoot, ToolbarSeparator,
};
pub use props::{ToolbarOrientation, ToolbarProps};
pub use runtime::{ToolbarItemMetadata, ToolbarMove, ToolbarRuntime};
pub use style_state::{
    ToolbarButtonStyleState, ToolbarGroupStyleState, ToolbarInputStyleState, ToolbarLinkStyleState,
    ToolbarRootStyleState,
};
