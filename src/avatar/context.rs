use gpui::{App, ElementId, Entity, Window};

use crate::avatar::AvatarRuntime;

pub struct AvatarContext {
    runtime: Entity<AvatarRuntime>,
}

impl Clone for AvatarContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
        }
    }
}

impl AvatarContext {
    pub fn new(id: impl Into<ElementId>, cx: &mut App, window: &mut Window) -> Self {
        let runtime = window.use_keyed_state(id, cx, |_, _| AvatarRuntime::new());

        Self { runtime }
    }

    pub fn read<Output>(&self, cx: &App, read: impl FnOnce(&AvatarRuntime) -> Output) -> Output {
        read(self.runtime.read(cx))
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut AvatarRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);
            cx.notify();
            output
        })
    }
}
