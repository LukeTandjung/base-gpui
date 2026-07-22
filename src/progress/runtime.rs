use crate::progress::{ProgressFormatHandler, ProgressStyleState};

/// Task-completion status, mirroring Base UI's exported `Progress.Status`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressStatus {
    /// The value is unknown (`None` or non-finite).
    Indeterminate,
    /// A determinate value below `max`.
    Progressing,
    /// The (clamped) value equals `max`.
    Complete,
}

/// Derivation-only runtime for Progress.
///
/// A plain struct of values computed once from `(value, min, max, format)` at
/// the top of root render. There is no mutable state, no commands, and no
/// callbacks — every part reads the same derived facts through [`Self::state`].
#[derive(Clone, Debug, PartialEq)]
pub struct ProgressRuntime {
    value: Option<f64>,
    clamped_value: Option<f64>,
    percentage: Option<f64>,
    formatted: Option<String>,
    min: f64,
    max: f64,
    status: ProgressStatus,
}

impl ProgressRuntime {
    /// Derives clamped value, percentage, status, and formatted text.
    ///
    /// `None` and non-finite values are indeterminate: no clamped value, no
    /// percentage, no formatted string, and `format` is never invoked.
    /// Determinate values are clamped into `[min, max]`; the percentage is
    /// `((value - min) / (max - min)) * 100` with `NaN` (degenerate
    /// `min == max` range) falling back to `0`, then clamped to `[0, 100]`.
    /// The default formatted string is the percent-of-range (e.g. `50%`);
    /// a custom `format` callback receives the raw unclamped value instead.
    pub fn new(
        value: Option<f64>,
        min: f64,
        max: f64,
        format: Option<&ProgressFormatHandler>,
    ) -> Self {
        let determinate = value.filter(|value| value.is_finite());
        let clamped_value = determinate.map(|value| value.max(min).min(max));
        let percentage = clamped_value.map(|clamped| {
            let raw = ((clamped - min) / (max - min)) * 100.0;
            if raw.is_nan() {
                0.0
            } else {
                raw.clamp(0.0, 100.0)
            }
        });
        let formatted = determinate.map(|raw| match format {
            Some(format) => format(raw),
            None => format!("{}%", percentage.unwrap_or(0.0)),
        });
        let status = match clamped_value {
            None => ProgressStatus::Indeterminate,
            Some(clamped) if clamped == max => ProgressStatus::Complete,
            Some(_) => ProgressStatus::Progressing,
        };
        Self {
            value: determinate,
            clamped_value,
            percentage,
            formatted,
            min,
            max,
            status,
        }
    }

    /// The one query: the shared style state observed by all five parts.
    pub fn state(&self) -> ProgressStyleState {
        ProgressStyleState {
            value: self.value,
            clamped_value: self.clamped_value,
            percentage: self.percentage,
            formatted: self.formatted.clone(),
            min: self.min,
            max: self.max,
            status: self.status,
        }
    }
}
