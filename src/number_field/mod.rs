//! Number Field component family.
//!
//! # Accessibility notes
//!
//! `NumberFieldGroup` reports `Role::Group` and the stepper buttons report
//! `Role::Button` with default `"Increase"` / `"Decrease"` labels (overridable via
//! `.aria_label(...)`). The root and scrub parts intentionally carry no role,
//! matching Base UI's plain `div` / `role="presentation"` / `aria-hidden` markup.
//!
//! Known gaps in the pinned gpui revision (blocked pending gpui upstream):
//! - `aria-labelledby`: no id-reference wiring, so `FieldLabel` cannot name the
//!   input; callers must pass a literal label string instead.
//! - `aria-controls` (stepper button -> input): no builder; the group role and
//!   button labels make the relationship inferable.
//! - `aria-invalid`, `disabled` / `aria-disabled`, and `required` announcements:
//!   no builders. Disabled/read-only state is still enforced — the runtime no-ops
//!   value-changing interactions — but it is not announced to assistive tech.
//! - `aria-roledescription: "Number field"`: no builder; `Role::SpinButton`
//!   already conveys the numeric-stepper semantics.
//! - The input's `Role::SpinButton` / numeric-value / label / Increment-Decrement
//!   a11y action wiring requires role and aria pass-through builders on the generic
//!   `input()` primitive, which does not expose them yet.

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
mod layers;
pub mod number;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, NumberFieldMax, NumberFieldMin, NumberFieldStepDown, NumberFieldStepDownLarge,
    NumberFieldStepDownSmall, NumberFieldStepUp, NumberFieldStepUpLarge, NumberFieldStepUpSmall,
    NUMBER_FIELD_KEY_CONTEXT,
};
pub use child::{NumberFieldChild, NumberFieldGroupChild};
pub use context::NumberFieldContext;
pub use layers::{
    NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput,
    NumberFieldRoot, NumberFieldScrubArea, NumberFieldScrubAreaCursor,
};
pub use number::{
    clamp_value, clean_floating_point_noise, format_number, normalize_optional_value,
    option_values_equal, parse_number, snap_to_step_grid, step_value, NumberFieldStep,
    NumberParseError,
};
pub use props::{NumberFieldProps, NumberFieldValueChangeHandler, NumberFieldValueCommitHandler};
pub use runtime::{
    NumberFieldChangeDetails, NumberFieldChangeReason, NumberFieldCommitDetails,
    NumberFieldCommitReason, NumberFieldRuntime, NumberFieldScrubDirection, NumberFieldStepAmount,
    NumberFieldStepDirection, NumberFieldUpdateOutcome, NumberFieldValueChange,
    NumberFieldValueCommit,
};
pub use style_state::{
    NumberFieldDecrementStyleState, NumberFieldGroupStyleState, NumberFieldIncrementStyleState,
    NumberFieldInputStyleState, NumberFieldRootStyleState, NumberFieldScrubAreaCursorStyleState,
    NumberFieldScrubAreaStyleState,
};
