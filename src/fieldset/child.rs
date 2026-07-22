use gpui::{AnyElement, IntoElement};

use crate::fieldset::FieldsetLegend;

pub enum FieldsetChild {
    Legend(FieldsetLegend),
    Any(AnyElement),
}

impl IntoElement for FieldsetChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Legend(legend) => legend.into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl From<FieldsetLegend> for FieldsetChild {
    fn from(value: FieldsetLegend) -> Self {
        Self::Legend(value)
    }
}
