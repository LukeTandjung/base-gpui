use gpui::{AnyElement, IntoElement};

use crate::{otp_field::OTPFieldInput, separator::Separator};

pub enum OTPFieldChild {
    Input(Box<OTPFieldInput>),
    Separator(Box<Separator>),
    Any(AnyElement),
}

impl IntoElement for OTPFieldChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Input(input) => (*input).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<OTPFieldInput> for OTPFieldChild {
    fn from(value: OTPFieldInput) -> Self {
        Self::Input(Box::new(value))
    }
}

impl From<Separator> for OTPFieldChild {
    fn from(value: Separator) -> Self {
        Self::Separator(Box::new(value))
    }
}
