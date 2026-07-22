use gpui::AnyElement;

use crate::progress::{ProgressIndicator, ProgressLabel, ProgressTrack, ProgressValue};

/// Typed children accepted by `ProgressRoot`.
pub enum ProgressChild {
    Track(Box<ProgressTrack>),
    Value(Box<ProgressValue>),
    Label(Box<ProgressLabel>),
    Any(AnyElement),
}

impl From<ProgressTrack> for ProgressChild {
    fn from(value: ProgressTrack) -> Self {
        Self::Track(Box::new(value))
    }
}

impl From<ProgressValue> for ProgressChild {
    fn from(value: ProgressValue) -> Self {
        Self::Value(Box::new(value))
    }
}

impl From<ProgressLabel> for ProgressChild {
    fn from(value: ProgressLabel) -> Self {
        Self::Label(Box::new(value))
    }
}

/// Typed children accepted by `ProgressTrack`.
pub enum ProgressTrackChild {
    Indicator(Box<ProgressIndicator>),
    Any(AnyElement),
}

impl From<ProgressIndicator> for ProgressTrackChild {
    fn from(value: ProgressIndicator) -> Self {
        Self::Indicator(Box::new(value))
    }
}
