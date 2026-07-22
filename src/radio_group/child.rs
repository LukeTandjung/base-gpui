use gpui::{AnyElement, IntoElement};

use crate::radio_group::{RadioGroupIndicator, RadioGroupRadio};

pub enum RadioGroupChild<T: Clone + Eq + 'static> {
    Radio(RadioGroupRadio<T>),
    Indicator(RadioGroupIndicator),
}

impl<T: Clone + Eq + 'static> IntoElement for RadioGroupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Radio(radio) => radio.into_any_element(),
            Self::Indicator(indicator) => indicator.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<RadioGroupRadio<T>> for RadioGroupChild<T> {
    fn from(value: RadioGroupRadio<T>) -> Self {
        Self::Radio(value)
    }
}

impl<T: Clone + Eq + 'static> From<RadioGroupIndicator> for RadioGroupChild<T> {
    fn from(value: RadioGroupIndicator) -> Self {
        Self::Indicator(value)
    }
}

pub enum RadioGroupRadioChild {
    Indicator(RadioGroupIndicator),
}

impl IntoElement for RadioGroupRadioChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Indicator(indicator) => indicator.into_any_element(),
        }
    }
}

impl From<RadioGroupIndicator> for RadioGroupRadioChild {
    fn from(value: RadioGroupIndicator) -> Self {
        Self::Indicator(value)
    }
}
