use std::{rc::Rc, sync::Arc};

use gpui::{ElementId, SharedString};

use crate::meter::{MeterProps, MeterRuntime};

/// Thin injection vehicle: the derivation-only runtime behind `Rc`, read-only.
pub struct MeterContext {
    id: ElementId,
    runtime: Rc<MeterRuntime>,
}

impl Clone for MeterContext {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: Rc::clone(&self.runtime),
        }
    }
}

impl MeterContext {
    pub fn new(id: impl Into<ElementId>, props: &MeterProps) -> Self {
        Self {
            id: id.into(),
            runtime: Rc::new(MeterRuntime::new(
                props.value(),
                props.min(),
                props.max(),
                props.format(),
            )),
        }
    }

    pub fn read<Output>(&self, read: impl FnOnce(&MeterRuntime) -> Output) -> Output {
        read(self.runtime.as_ref())
    }

    pub fn id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn child_id(&self, child: impl Into<SharedString>) -> ElementId {
        ElementId::NamedChild(Arc::new(self.id.clone()), child.into())
    }
}
