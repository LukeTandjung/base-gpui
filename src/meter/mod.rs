//! GPUI-native port of Base UI Meter.
//!
//! Purely presentational compound component: `MeterRoot` derives clamped
//! value, percentage, and formatted text once per render; the
//! Track/Indicator/Value/Label parts are thin views of the same derived
//! state via `style_with_state`. Unlike Progress, Meter is always
//! determinate — there is no nullable value and no status.
//!
//! Accessibility: `MeterRoot` is the only node in the a11y tree
//! (`Role::Meter` with `aria_numeric_value`/min/max from the clamped state);
//! Track/Indicator are presentational and Value/Label render inaccessible
//! text. Gaps deferred pending gpui upstream: no `aria-valuetext` builder
//! (a `getAriaValueText`-style override is unsupported; the formatted string
//! is folded into the root's `aria_label` as a fallback) and no
//! `aria-labelledby` relationship builder (replaced by the literal-string
//! `MeterRoot::aria_label` prop). The visually-hidden NVDA span is a
//! DOM-specific hack and is not ported.
//!
//! Dropped or deferred Base UI props: `getAriaValueText` and the
//! visually-hidden NVDA span (see above), `aria-labelledby` id plumbing,
//! `Intl.NumberFormat`
//! `format`/`locale` options (replaced by a plain `Fn(f64) -> String`
//! callback), and CSS class/style/data-attribute APIs (replaced by
//! `style_with_state` and `MeterStyleState`).

pub mod child;
pub mod context;
mod layers;
pub mod props;
pub mod runtime;
pub mod style_state;

#[cfg(test)]
mod tests;

pub use child::{MeterChild, MeterTrackChild};
pub use context::MeterContext;
pub use layers::{
    MeterIndicator, MeterLabel, MeterRoot, MeterTrack, MeterValue, MeterValueDisplayHandler,
};
pub use props::{MeterFormatHandler, MeterProps};
pub use runtime::MeterRuntime;
pub use style_state::MeterStyleState;
