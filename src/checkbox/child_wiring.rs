use crate::checkbox::{CheckboxChild, CheckboxContext};

pub trait CheckboxChildNode: Sized {
    fn with_checkbox_context(self, context: CheckboxContext) -> Self;
}

impl CheckboxChildNode for CheckboxChild {
    fn with_checkbox_context(self, context: CheckboxContext) -> Self {
        match self {
            Self::Indicator(indicator) => Self::Indicator(indicator.with_checkbox_context(context)),
        }
    }
}
