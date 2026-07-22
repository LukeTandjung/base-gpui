//! GPUI-native port of Base UI Slider.
//!
//! Phase-1 prop subset on `SliderRoot`: `id`, `name`, `default_value`,
//! `value`, `on_value_change` (cancelable), `on_value_committed`, `min`,
//! `max`, `step`, `large_step`, `min_steps_between_values`, `orientation`,
//! `thumb_collision_behavior`, `thumb_alignment`, `disabled`, and `format`
//! (a plain `Fn(f64) -> SharedString` closure). `SliderThumb` supports
//! per-thumb `disabled`; `SliderValue` supports a custom `display` closure.
//!
//! Dropped or deferred Base UI props: `Intl.NumberFormat` `format`/`locale`
//! options (deferred, shared with Number Field), ARIA options (`aria-label`,
//! `getAriaLabel`, `getAriaValueText` — deferred to the AccessKit follow-up),
//! the `edge-client-only` thumb-alignment variant and all SSR/hydration
//! machinery, the hidden `<input type="range">` with `form`, `inputRef`, and
//! `tabIndex` (replaced by per-thumb `FocusHandle`s, GPUI actions, and Field
//! registration), and CSS class/style/data-attribute/CSS-variable APIs
//! (replaced by `style_with_state` and typed style-state structs).
//!
//! AccessKit: `SliderRoot` is `Role::Group` with `.aria_label(...)` and
//! `.aria_orientation(...)`; each `SliderThumb` is `Role::Slider` carrying
//! `aria_numeric_value`/min/max, orientation, position-in-set/size-of-set,
//! an optional per-thumb `.aria_label(...)`, and `Increment`/`Decrement`/
//! `SetValue` a11y actions routed through the neighbor-clamped keyboard
//! path. Blocked pending gpui upstream (no builders in this revision):
//! `aria-labelledby`/`aria-describedby` id references (fallback: literal
//! `.aria_label` strings), `aria-valuetext`/`getAriaValueText` (fallback:
//! raw numeric value plus set position), `disabled`/`aria-disabled` on the
//! thumb (fallback: a11y actions are not registered while disabled, so AT
//! sees an inert but not announced-as-disabled slider), and `aria-live` on
//! `SliderValue` (fallback: omitted; the focused thumb's numeric value
//! updates instead).

pub mod actions;
pub mod child;
mod child_wiring;
pub mod context;
mod layers;
pub mod math;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use actions::{
    init, SliderEnd, SliderHome, SliderPageDown, SliderPageUp, SliderStepDown, SliderStepDownLarge,
    SliderStepLeft, SliderStepLeftLarge, SliderStepRight, SliderStepRightLarge, SliderStepUp,
    SliderStepUpLarge, SLIDER_THUMB_KEY_CONTEXT,
};
pub use child::{SliderChild, SliderControlChild, SliderTrackChild};
pub use context::SliderContext;
pub use layers::{
    SliderControl, SliderIndicator, SliderLabel, SliderRoot, SliderThumb, SliderTrack, SliderValue,
    SliderValueDisplayHandler,
};
pub use math::{
    clamp, fraction_to_value, get_decimal_precision, get_new_value, get_pushed_thumb_values,
    get_slider_value, position_to_fraction, resolve_thumb_collision, round_to_precision,
    round_value_to_step, validate_minimum_distance, value_to_fraction, SliderCollisionResult,
    SliderThumbCollisionBehavior,
};
pub use props::{
    SliderFormatHandler, SliderProps, SliderValueChangeHandler, SliderValueCommitHandler,
};
pub use runtime::{
    format_slider_value, values_equal, SliderChangeReason, SliderKeyboardStep, SliderOrientation,
    SliderProposal, SliderRuntime, SliderThumbAlignment, SliderThumbMeta, SliderValueChangeDetails,
    SliderValueCommitDetails, SliderValues,
};
pub use style_state::{
    SliderControlStyleState, SliderIndicatorStyleState, SliderLabelStyleState,
    SliderRootStyleState, SliderThumbStyleState, SliderTrackStyleState, SliderValueStyleState,
};
