use gpui::{AnyElement, IntoElement};

use crate::switch::SwitchThumb;

pub enum SwitchChild {
    Thumb(SwitchThumb),
}

impl IntoElement for SwitchChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Thumb(thumb) => thumb.into_any_element(),
        }
    }
}

impl From<SwitchThumb> for SwitchChild {
    fn from(value: SwitchThumb) -> Self {
        Self::Thumb(value)
    }
}
