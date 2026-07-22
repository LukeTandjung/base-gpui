use gpui::SharedString;

use crate::slider::{SliderOrientation, SliderValues};

/// Shared slider facts exposed to every part.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderRootStyleState {
    pub values: SliderValues,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub orientation: SliderOrientation,
    pub disabled: bool,
    pub dragging: bool,
    pub active_thumb_index: Option<usize>,
    pub focused: bool,
    pub touched: bool,
    pub dirty: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderControlStyleState {
    pub root: SliderRootStyleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderTrackStyleState {
    pub root: SliderRootStyleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderIndicatorStyleState {
    pub root: SliderRootStyleState,
    /// Main-axis start of the filled range as a `[0, 1]` fraction.
    pub start_fraction: f64,
    /// Main-axis end of the filled range as a `[0, 1]` fraction.
    pub end_fraction: f64,
    /// False for edge alignment before the first measurement.
    pub positioned: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderThumbStyleState {
    pub root: SliderRootStyleState,
    pub index: usize,
    pub value: f64,
    pub formatted_value: SharedString,
    pub focused: bool,
    pub active: bool,
    pub disabled: bool,
    /// Main-axis position as a `[0, 1]` fraction of the control.
    pub fraction: f64,
    /// False for edge alignment before the first measurement.
    pub positioned: bool,
    /// Half of the measured thumb main-axis size (0 before measurement).
    pub half_thumb_main_axis: f64,
    /// Edge alignment: resolved main-axis pixel offset within the travel.
    pub edge_offset: Option<f64>,
    /// Stacking priority: 2 active, 1 last-used, 0 otherwise.
    pub z_index: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderValueStyleState {
    pub root: SliderRootStyleState,
    pub values: Vec<f64>,
    pub formatted_values: Vec<SharedString>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SliderLabelStyleState {
    pub root: SliderRootStyleState,
}
