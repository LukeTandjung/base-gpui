use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::toolbar::{ToolbarProps, ToolbarRuntime};

/// Thin injection vehicle for the toolbar runtime and props. Toolbar
/// vocabulary (registration, highlight movement) lives on the runtime; this
/// type only reads and updates it. Nested compound children (a future
/// ToggleGroup) detect an enclosing toolbar through the presence of this
/// context and suppress their own roving focus.
pub struct ToolbarContext {
    runtime: Entity<ToolbarRuntime>,
    props: Rc<ToolbarProps>,
}

impl Clone for ToolbarContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
        }
    }
}

impl ToolbarContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: ToolbarProps,
    ) -> Self {
        let runtime = window.use_keyed_state(id.into(), cx, |_, _| ToolbarRuntime::new());

        Self {
            runtime,
            props: Rc::new(props),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&ToolbarRuntime, &ToolbarProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut ToolbarRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);
            cx.notify();
            output
        })
    }
}
