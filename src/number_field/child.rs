use gpui::{AnyElement, IntoElement};

use crate::number_field::{
    NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput,
    NumberFieldScrubArea, NumberFieldScrubAreaCursor,
};

pub enum NumberFieldChild {
    Input(Box<NumberFieldInput>),
    Group(Box<NumberFieldGroup>),
    Increment(Box<NumberFieldIncrement>),
    Decrement(Box<NumberFieldDecrement>),
    ScrubArea(Box<NumberFieldScrubArea>),
    ScrubAreaCursor(Box<NumberFieldScrubAreaCursor>),
    Any(AnyElement),
}

impl IntoElement for NumberFieldChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Input(input) => (*input).into_any_element(),
            Self::Group(group) => (*group).into_any_element(),
            Self::Increment(increment) => (*increment).into_any_element(),
            Self::Decrement(decrement) => (*decrement).into_any_element(),
            Self::ScrubArea(scrub_area) => (*scrub_area).into_any_element(),
            Self::ScrubAreaCursor(cursor) => (*cursor).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<NumberFieldInput> for NumberFieldChild {
    fn from(value: NumberFieldInput) -> Self {
        Self::Input(Box::new(value))
    }
}

impl From<NumberFieldGroup> for NumberFieldChild {
    fn from(value: NumberFieldGroup) -> Self {
        Self::Group(Box::new(value))
    }
}

impl From<NumberFieldIncrement> for NumberFieldChild {
    fn from(value: NumberFieldIncrement) -> Self {
        Self::Increment(Box::new(value))
    }
}

impl From<NumberFieldDecrement> for NumberFieldChild {
    fn from(value: NumberFieldDecrement) -> Self {
        Self::Decrement(Box::new(value))
    }
}

impl From<NumberFieldScrubArea> for NumberFieldChild {
    fn from(value: NumberFieldScrubArea) -> Self {
        Self::ScrubArea(Box::new(value))
    }
}

impl From<NumberFieldScrubAreaCursor> for NumberFieldChild {
    fn from(value: NumberFieldScrubAreaCursor) -> Self {
        Self::ScrubAreaCursor(Box::new(value))
    }
}

pub enum NumberFieldGroupChild {
    Input(Box<NumberFieldInput>),
    Increment(Box<NumberFieldIncrement>),
    Decrement(Box<NumberFieldDecrement>),
    Any(AnyElement),
}

impl IntoElement for NumberFieldGroupChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Input(input) => (*input).into_any_element(),
            Self::Increment(increment) => (*increment).into_any_element(),
            Self::Decrement(decrement) => (*decrement).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<NumberFieldInput> for NumberFieldGroupChild {
    fn from(value: NumberFieldInput) -> Self {
        Self::Input(Box::new(value))
    }
}

impl From<NumberFieldIncrement> for NumberFieldGroupChild {
    fn from(value: NumberFieldIncrement) -> Self {
        Self::Increment(Box::new(value))
    }
}

impl From<NumberFieldDecrement> for NumberFieldGroupChild {
    fn from(value: NumberFieldDecrement) -> Self {
        Self::Decrement(Box::new(value))
    }
}
