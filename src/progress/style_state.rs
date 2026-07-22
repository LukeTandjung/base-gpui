use crate::progress::ProgressStatus;

/// Shared style state observed by every Progress part (Root, Track,
/// Indicator, Value, Label). Base UI applies the identical `{ status }`
/// state to all five parts, so one struct serves them all.
#[derive(Clone, Debug, PartialEq)]
pub struct ProgressStyleState {
    /// The raw finite value, `None` when indeterminate.
    pub value: Option<f64>,
    /// The value clamped into `[min, max]`, `None` when indeterminate.
    pub clamped_value: Option<f64>,
    /// Percent-of-range in `[0, 100]`, `None` when indeterminate.
    pub percentage: Option<f64>,
    /// Formatted display text, `None` when indeterminate.
    pub formatted: Option<String>,
    pub min: f64,
    pub max: f64,
    pub status: ProgressStatus,
}
