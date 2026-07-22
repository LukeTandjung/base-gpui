use gpui::{AnyElement, IntoElement};

use crate::checkbox::CheckboxIndicator;

pub enum CheckboxChild {
    Indicator(CheckboxIndicator),
}

impl IntoElement for CheckboxChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Indicator(indicator) => indicator.into_any_element(),
        }
    }
}

impl From<CheckboxIndicator> for CheckboxChild {
    fn from(value: CheckboxIndicator) -> Self {
        Self::Indicator(value)
    }
}
