use std::{rc::Rc, sync::Arc};

use gpui::{ElementId, SharedString};

use crate::progress::{ProgressProps, ProgressRuntime};

/// Thin injection vehicle: the derivation-only runtime behind `Rc`, read-only.
pub struct ProgressContext {
    id: ElementId,
    runtime: Rc<ProgressRuntime>,
}

impl Clone for ProgressContext {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: Rc::clone(&self.runtime),
        }
    }
}

impl ProgressContext {
    pub fn new(id: impl Into<ElementId>, props: &ProgressProps) -> Self {
        Self {
            id: id.into(),
            runtime: Rc::new(ProgressRuntime::new(
                props.value(),
                props.min(),
                props.max(),
                props.format(),
            )),
        }
    }

    pub fn read<Output>(&self, read: impl FnOnce(&ProgressRuntime) -> Output) -> Output {
        read(self.runtime.as_ref())
    }

    pub fn id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn child_id(&self, child: impl Into<SharedString>) -> ElementId {
        ElementId::NamedChild(Arc::new(self.id.clone()), child.into())
    }
}
