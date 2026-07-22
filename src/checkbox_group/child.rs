use gpui::{AnyElement, IntoElement};

pub enum CheckboxGroupChild {
    Any(AnyElement),
}

impl IntoElement for CheckboxGroupChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Any(child) => child,
        }
    }
}
