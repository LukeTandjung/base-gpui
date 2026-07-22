use crate::number_field::{
    NumberFieldChild, NumberFieldContext, NumberFieldDecrement, NumberFieldGroup,
    NumberFieldGroupChild, NumberFieldIncrement, NumberFieldInput, NumberFieldScrubArea,
    NumberFieldScrubAreaCursor,
};

pub trait NumberFieldChildNode: Sized {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self;
}

pub fn wire_children(
    children: Vec<NumberFieldChild>,
    context: NumberFieldContext,
) -> Vec<NumberFieldChild> {
    children
        .into_iter()
        .map(|child| child.with_number_field_context(context.clone()))
        .collect()
}

pub fn wire_group_children(
    children: Vec<NumberFieldGroupChild>,
    context: NumberFieldContext,
) -> Vec<NumberFieldGroupChild> {
    children
        .into_iter()
        .map(|child| child.with_number_field_context(context.clone()))
        .collect()
}

impl NumberFieldChildNode for NumberFieldChild {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        match self {
            Self::Input(input) => {
                Self::Input(Box::new((*input).with_number_field_context(context)))
            }
            Self::Group(group) => {
                Self::Group(Box::new((*group).with_number_field_context(context)))
            }
            Self::Increment(increment) => {
                Self::Increment(Box::new((*increment).with_number_field_context(context)))
            }
            Self::Decrement(decrement) => {
                Self::Decrement(Box::new((*decrement).with_number_field_context(context)))
            }
            Self::ScrubArea(scrub_area) => {
                Self::ScrubArea(Box::new((*scrub_area).with_number_field_context(context)))
            }
            Self::ScrubAreaCursor(cursor) => {
                Self::ScrubAreaCursor(Box::new((*cursor).with_number_field_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl NumberFieldChildNode for NumberFieldGroupChild {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        match self {
            Self::Input(input) => {
                Self::Input(Box::new((*input).with_number_field_context(context)))
            }
            Self::Increment(increment) => {
                Self::Increment(Box::new((*increment).with_number_field_context(context)))
            }
            Self::Decrement(decrement) => {
                Self::Decrement(Box::new((*decrement).with_number_field_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl NumberFieldChildNode for NumberFieldInput {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        self.with_number_field_context(context)
    }
}

impl NumberFieldChildNode for NumberFieldGroup {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        self.with_number_field_context(context)
    }
}

impl NumberFieldChildNode for NumberFieldIncrement {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        self.with_number_field_context(context)
    }
}

impl NumberFieldChildNode for NumberFieldDecrement {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        self.with_number_field_context(context)
    }
}

impl NumberFieldChildNode for NumberFieldScrubArea {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        self.with_number_field_context(context)
    }
}

impl NumberFieldChildNode for NumberFieldScrubAreaCursor {
    fn with_number_field_context(self, context: NumberFieldContext) -> Self {
        self.with_number_field_context(context)
    }
}
