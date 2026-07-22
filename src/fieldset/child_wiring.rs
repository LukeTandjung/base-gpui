use crate::fieldset::{FieldsetChild, FieldsetContext, FieldsetLegend};

pub trait FieldsetChildNode: Sized {
    fn with_fieldset_context(self, context: FieldsetContext) -> Self;
}

pub fn wire_children(children: Vec<FieldsetChild>, context: FieldsetContext) -> Vec<FieldsetChild> {
    children
        .into_iter()
        .map(|child| child.with_fieldset_context(context.clone()))
        .collect()
}

impl FieldsetChildNode for FieldsetChild {
    fn with_fieldset_context(self, context: FieldsetContext) -> Self {
        match self {
            Self::Legend(legend) => Self::Legend(legend.with_fieldset_context(context)),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl FieldsetChildNode for FieldsetLegend {
    fn with_fieldset_context(self, context: FieldsetContext) -> Self {
        self.with_fieldset_context(context)
    }
}
