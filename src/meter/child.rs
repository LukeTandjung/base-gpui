use gpui::AnyElement;

use crate::meter::{MeterIndicator, MeterLabel, MeterTrack, MeterValue};

/// Typed children accepted by `MeterRoot`.
pub enum MeterChild {
    Track(Box<MeterTrack>),
    Value(Box<MeterValue>),
    Label(Box<MeterLabel>),
    Any(AnyElement),
}

impl From<MeterTrack> for MeterChild {
    fn from(value: MeterTrack) -> Self {
        Self::Track(Box::new(value))
    }
}

impl From<MeterValue> for MeterChild {
    fn from(value: MeterValue) -> Self {
        Self::Value(Box::new(value))
    }
}

impl From<MeterLabel> for MeterChild {
    fn from(value: MeterLabel) -> Self {
        Self::Label(Box::new(value))
    }
}

/// Typed children accepted by `MeterTrack`.
pub enum MeterTrackChild {
    Indicator(Box<MeterIndicator>),
    Any(AnyElement),
}

impl From<MeterIndicator> for MeterTrackChild {
    fn from(value: MeterIndicator) -> Self {
        Self::Indicator(Box::new(value))
    }
}
