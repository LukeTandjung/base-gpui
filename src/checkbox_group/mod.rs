//! Checkbox Group.
//!
//! Accessibility: the group div carries `Role::Group` plus an optional literal
//! `.aria_label(...)`. Known AccessKit gaps in the pinned gpui revision (see
//! `docs/accesskit-gpui-reference.md`): no `aria-labelledby` id-reference
//! wiring (automatic Field label association is not mirrored into the a11y
//! tree), and no `aria-disabled` builder — disabling remains behavior-level
//! only (blocked pending an upstream gpui `set_disabled` addition).

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

pub use actions::init;
pub use child::CheckboxGroupChild;
pub use context::{current_checkbox_group_context, CheckboxGroupContext};
pub use layers::CheckboxGroup;
pub use props::{CheckboxGroupProps, CheckboxGroupValueChangeHandler};
pub use runtime::{
    CheckboxGroupChildMetadata, CheckboxGroupParentStatus, CheckboxGroupRuntime,
    CheckboxGroupValueChangeDetails, CheckboxGroupValueChangeReason,
    CheckboxGroupValueChangeRequest,
};
pub use style_state::CheckboxGroupStyleState;
