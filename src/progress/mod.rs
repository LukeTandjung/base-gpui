//! GPUI-native port of Base UI Progress.
//!
//! Purely presentational compound component: `ProgressRoot` derives clamped
//! value, percentage, `ProgressStatus`, and formatted text once per render;
//! the Track/Indicator/Value/Label parts are thin views of the same derived
//! state via `style_with_state`.
//!
//! Dropped or deferred Base UI props: `Intl.NumberFormat` `format`/`locale`
//! options (replaced by a plain `Fn(f64) -> String` callback), and CSS
//! class/style/data-attribute APIs (replaced by `style_with_state` and
//! `ProgressStyleState`). No built-in indeterminate animation — Base UI
//! ships none; indeterminate is status exposure only.
//!
//! Accessibility: `ProgressRoot` is the single accessible node
//! (`Role::ProgressIndicator` with `aria_min/max_numeric_value` always and
//! `aria_numeric_value` only when determinate — omitting it is the AccessKit
//! idiom for an indeterminate progressbar). Track, Indicator, Value, and
//! Label carry no role and stay out of the a11y tree, matching Base UI's
//! role-less divs / `aria-hidden` / `role="presentation"`. Gaps in this gpui
//! revision: `aria-valuetext` (no builder; omitted — AT gets numeric
//! value/min/max instead), `aria-labelledby` (no relationship builders;
//! replaced by the literal-string `ProgressRoot::label(...)` — pass the same
//! string rendered inside `ProgressLabel`, and render label/value text via
//! `Text::new_inaccessible(...)` to avoid double announcement), and the NVDA
//! visually-hidden announcement span / live-region updates (no announcement
//! API; blocked pending gpui upstream).

pub mod child;
pub mod context;
mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use child::{ProgressChild, ProgressTrackChild};
pub use context::ProgressContext;
pub use layers::{
    ProgressIndicator, ProgressLabel, ProgressRoot, ProgressTrack, ProgressValue,
    ProgressValueDisplayHandler,
};
pub use props::{ProgressFormatHandler, ProgressProps};
pub use runtime::{ProgressRuntime, ProgressStatus};
pub use style_state::ProgressStyleState;
