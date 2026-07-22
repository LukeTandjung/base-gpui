//! # AccessKit notes
//!
//! The menu family sets `Role::Button`/`Role::MenuItem` on triggers,
//! `Role::Menu` on popups, `Role::MenuItem{,CheckBox,Radio}` on items, and
//! `Role::Group` on (radio) groups. Backdrop, arrow, indicators, group label,
//! portal, positioner, and the structural roots carry no role and stay out of
//! the accessibility tree (mirroring Base UI's `role="presentation"` /
//! `aria-hidden`). Gaps in the pinned gpui revision, omitted rather than
//! faked (blocked on upstream builders):
//! - `aria-haspopup="menu"` / `aria-controls` on triggers and submenu
//!   triggers: `aria_expanded` plus the popup's `Role::Menu` stand in.
//! - `aria-labelledby`: replaced by literal-string `.aria_label(...)` sourced
//!   from the registered group-label metadata.
//! - `disabled`/`aria-disabled`: disabled parts are inert (withheld tab
//!   stops, activation no-ops) but the disabled state is not announced.

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

pub use actions::{
    init, MenuActivateHighlighted, MenuArrowLeft, MenuArrowRight, MenuCloseAction, MenuMoveFirst,
    MenuMoveLast, MenuMoveNext, MenuMovePrevious, MenuSpaceActivate, MENU_KEY_CONTEXT,
};
pub use child::{
    MenuCheckboxItemChild, MenuChild, MenuContextAreaBuild, MenuGroupChild, MenuPopupChild,
    MenuPortalChild, MenuPositionerChild, MenuRadioGroupChild, MenuRadioItemChild,
    MenuSubmenuRootChild,
};
pub use context::MenuContext;
pub use layers::{
    MenuArrow, MenuBackdrop, MenuCheckboxItem, MenuCheckboxItemIndicator, MenuGroup,
    MenuGroupLabel, MenuItem, MenuLinkItem, MenuPopup, MenuPortal, MenuPositioner, MenuRadioGroup,
    MenuRadioItem, MenuRadioItemIndicator, MenuRoot, MenuSeparator, MenuSubmenuRoot,
    MenuSubmenuTrigger, MenuTrigger,
};
pub use props::{
    MenuActivationHandler, MenuAlign, MenuCheckedChangeHandler, MenuOpenChangeCompleteHandler,
    MenuOpenChangeHandler, MenuOrientation, MenuProps, MenuSide, MenuValueChangeHandler,
};
pub use runtime::{
    scoped_menu_id, MenuChildHoverDirective, MenuContextMenuMouseUp, MenuHoverTarget,
    MenuInstantKind, MenuItemActivation, MenuItemChangeDetails, MenuItemKind, MenuItemMetadata,
    MenuMenubarLink, MenuMenubarOpenFn, MenuMove, MenuOpenChangeDetails, MenuOpenChangeOutcome,
    MenuOpenChangeReason, MenuOpenChangeSource, MenuParentKind, MenuRuntime, MenuSubmenuLink,
    MenuTriggerMetadata, MenuTypeaheadOutcome, CONTEXT_MENU_GRACE,
    CONTEXT_MENU_INITIAL_POINT_TOLERANCE,
};
pub use style_state::{
    MenuArrowStyleState, MenuBackdropStyleState, MenuCheckboxItemIndicatorStyleState,
    MenuCheckboxItemStyleState, MenuGroupLabelStyleState, MenuGroupStyleState, MenuItemStyleState,
    MenuLinkItemStyleState, MenuPopupStyleState, MenuPortalStyleState, MenuPositionerStyleState,
    MenuRadioGroupStyleState, MenuRadioItemIndicatorStyleState, MenuRadioItemStyleState,
    MenuRootStyleState, MenuSubmenuTriggerStyleState, MenuTriggerStyleState,
};
