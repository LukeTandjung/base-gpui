pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod props;
pub mod provider;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

// Reused Dialog parts under Drawer names — literal re-exports, no forked
// implementations (Base UI `drawer/index.parts.ts`). A Drawer intentionally
// shares the Dialog handle type.
pub use crate::dialog::{
    create_dialog_handle as create_drawer_handle, DialogClose as DrawerClose,
    DialogDescription as DrawerDescription, DialogHandle as DrawerHandle,
    DialogTitle as DrawerTitle, DialogTrigger as DrawerTrigger,
};

pub use child::{DrawerChild, DrawerPopupChild, DrawerPortalChild, DrawerViewportChild};
pub use context::{DrawerContext, DrawerNestedReport, DrawerNestedReporter};
pub use layers::{
    DrawerBackdrop, DrawerContent, DrawerIndent, DrawerIndentBackground, DrawerPopup, DrawerPortal,
    DrawerProvider, DrawerRoot, DrawerSwipeArea, DrawerViewport,
};
pub use props::{DrawerProps, DrawerSnapPointChangeDetails, DrawerSnapPointChangeHandler};
pub use provider::{drawer_provider_registry, DrawerProviderRegistry};
pub use runtime::{
    closest_resolved_snap_point, drawer_now_ms, resolve_snap_points, signed_sqrt_damping,
    snap_point_swipe_offset, DrawerOpenSwipeRelease, DrawerRuntime, DrawerSwipeReleaseOutcome,
    ResolvedSnapPoint,
};
pub use style_state::{
    DrawerBackdropStyleState, DrawerContentStyleState, DrawerIndentBackgroundStyleState,
    DrawerIndentStyleState, DrawerPopupFacts, DrawerPopupStyleState, DrawerSnapPoint,
    DrawerSwipeAreaStyleState, DrawerSwipeDirection, DrawerViewportStyleState,
};
