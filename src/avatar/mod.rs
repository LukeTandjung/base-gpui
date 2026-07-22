pub mod child;
mod child_wiring;
pub mod context;
pub mod layers;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use child::AvatarChild;
pub use context::AvatarContext;
pub use layers::{AvatarFallback, AvatarImage, AvatarLoadingStatusChangeHandler, AvatarRoot};
pub use runtime::{AvatarRuntime, AvatarStatusOutcome};
pub use style_state::{
    AvatarFallbackStyleState, AvatarImageLoadingStatus, AvatarImageStyleState, AvatarRootStyleState,
};
