use gpui::{AnyElement, IntoElement};

use crate::avatar::{AvatarFallback, AvatarImage};

pub enum AvatarChild {
    Image(AvatarImage),
    Fallback(AvatarFallback),
    Any(AnyElement),
}

impl IntoElement for AvatarChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Image(image) => image.into_any_element(),
            Self::Fallback(fallback) => fallback.into_any_element(),
            Self::Any(element) => element,
        }
    }
}

impl From<AvatarImage> for AvatarChild {
    fn from(value: AvatarImage) -> Self {
        Self::Image(value)
    }
}

impl From<AvatarFallback> for AvatarChild {
    fn from(value: AvatarFallback) -> Self {
        Self::Fallback(value)
    }
}

impl From<AnyElement> for AvatarChild {
    fn from(value: AnyElement) -> Self {
        Self::Any(value)
    }
}
