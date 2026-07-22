use crate::meter::{MeterFormatHandler, MeterStyleState};

/// Derivation-only runtime for Meter.
///
/// A plain struct of values computed once from `(value, min, max, format)` at
/// the top of root render. There is no mutable state, no commands, and no
/// callbacks — every part reads the same derived facts through [`Self::state`].
#[derive(Clone, Debug, PartialEq)]
pub struct MeterRuntime {
    value: f64,
    clamped_value: f64,
    percentage: f64,
    formatted: String,
    min: f64,
    max: f64,
}

impl MeterRuntime {
    /// Derives clamped value, percentage, and formatted text.
    ///
    /// A `NaN` value falls back to `min` (matching Base UI) before being
    /// clamped into `[min, max]`. The percentage is
    /// `((value - min) / (max - min)) * 100` with `NaN` (degenerate
    /// `min == max` range) falling back to `0`, then clamped to `[0, 100]`.
    /// The default formatted string is the percent-of-range (e.g. `50%`);
    /// a custom `format` callback receives the raw unclamped value instead.
    pub fn new(value: f64, min: f64, max: f64, format: Option<&MeterFormatHandler>) -> Self {
        let fallback = if value.is_nan() { min } else { value };
        let clamped_value = fallback.max(min).min(max);
        let raw_percentage = ((clamped_value - min) / (max - min)) * 100.0;
        let percentage = if raw_percentage.is_nan() {
            0.0
        } else {
            raw_percentage.clamp(0.0, 100.0)
        };
        let formatted = match format {
            Some(format) => format(value),
            None => format!("{percentage}%"),
        };
        Self {
            value,
            clamped_value,
            percentage,
            formatted,
            min,
            max,
        }
    }

    /// The one query: the shared style state observed by all five parts.
    pub fn state(&self) -> MeterStyleState {
        MeterStyleState {
            value: self.value,
            clamped_value: self.clamped_value,
            percentage: self.percentage,
            formatted: self.formatted.clone(),
            min: self.min,
            max: self.max,
        }
    }
}
