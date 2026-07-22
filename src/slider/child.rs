use gpui::{AnyElement, IntoElement};

use crate::slider::{
    SliderControl, SliderIndicator, SliderLabel, SliderThumb, SliderTrack, SliderValue,
};

pub enum SliderChild {
    Control(Box<SliderControl>),
    Value(Box<SliderValue>),
    Label(Box<SliderLabel>),
    Any(AnyElement),
}

impl IntoElement for SliderChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Control(control) => (*control).into_any_element(),
            Self::Value(value) => (*value).into_any_element(),
            Self::Label(label) => (*label).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<SliderControl> for SliderChild {
    fn from(value: SliderControl) -> Self {
        Self::Control(Box::new(value))
    }
}

impl From<SliderValue> for SliderChild {
    fn from(value: SliderValue) -> Self {
        Self::Value(Box::new(value))
    }
}

impl From<SliderLabel> for SliderChild {
    fn from(value: SliderLabel) -> Self {
        Self::Label(Box::new(value))
    }
}

pub enum SliderControlChild {
    Track(Box<SliderTrack>),
    Thumb(Box<SliderThumb>),
    Any(AnyElement),
}

impl IntoElement for SliderControlChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Track(track) => (*track).into_any_element(),
            Self::Thumb(thumb) => (*thumb).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<SliderTrack> for SliderControlChild {
    fn from(value: SliderTrack) -> Self {
        Self::Track(Box::new(value))
    }
}

impl From<SliderThumb> for SliderControlChild {
    fn from(value: SliderThumb) -> Self {
        Self::Thumb(Box::new(value))
    }
}

pub enum SliderTrackChild {
    Indicator(Box<SliderIndicator>),
    Any(AnyElement),
}

impl IntoElement for SliderTrackChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Indicator(indicator) => (*indicator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<SliderIndicator> for SliderTrackChild {
    fn from(value: SliderIndicator) -> Self {
        Self::Indicator(Box::new(value))
    }
}
