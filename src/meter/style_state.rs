/// Shared style state observed by every Meter part (Root, Track, Indicator,
/// Value, Label). Base UI's part states are all empty and every context
/// value is shared, so one struct serves all five parts — the GPUI
/// translation of `MeterRootContext`.
#[derive(Clone, Debug, PartialEq)]
pub struct MeterStyleState {
    /// The raw value as passed to the root (may be out of range or `NaN`).
    pub value: f64,
    /// The value clamped into `[min, max]` (`NaN` falls back to `min`).
    pub clamped_value: f64,
    /// Percent-of-range in `[0, 100]`.
    pub percentage: f64,
    /// Formatted display text.
    pub formatted: String,
    pub min: f64,
    pub max: f64,
}
