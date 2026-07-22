use crate::switch::{SwitchChild, SwitchContext};

pub trait SwitchChildNode: Sized {
    fn with_switch_context(self, context: SwitchContext) -> Self;
}

impl SwitchChildNode for SwitchChild {
    fn with_switch_context(self, context: SwitchContext) -> Self {
        match self {
            Self::Thumb(thumb) => Self::Thumb(thumb.with_switch_context(context)),
        }
    }
}
