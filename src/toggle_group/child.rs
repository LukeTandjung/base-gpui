use gpui::{AnyElement, IntoElement};

use crate::toggle::Toggle;

pub enum ToggleGroupChild<T: Clone + Eq + 'static> {
    Toggle(Toggle<T>),
}

impl<T: Clone + Eq + 'static> IntoElement for ToggleGroupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Toggle(toggle) => toggle.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<Toggle<T>> for ToggleGroupChild<T> {
    fn from(value: Toggle<T>) -> Self {
        Self::Toggle(value)
    }
}
