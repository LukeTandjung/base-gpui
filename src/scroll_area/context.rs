//! Thin injection vehicle for Scroll Area parts: the shared runtime entity
//! plus the props, exposing only `read` / `update` / `refresh`. Scroll Area
//! has no controlled value, so there is no `select`-analogue and the context
//! carries no component vocabulary.

use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::scroll_area::{ScrollAreaProps, ScrollAreaRuntime};

pub struct ScrollAreaContext {
    runtime: Entity<ScrollAreaRuntime>,
    props: Rc<ScrollAreaProps>,
}

impl Clone for ScrollAreaContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
        }
    }
}

impl ScrollAreaContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: ScrollAreaProps,
    ) -> Self {
        let runtime = window.use_keyed_state(id, cx, |_, _| ScrollAreaRuntime::new());

        Self {
            runtime,
            props: Rc::new(props),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&ScrollAreaRuntime, &ScrollAreaProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    /// Run a command and notify. Use from event and timer handlers.
    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut ScrollAreaRuntime, &ScrollAreaProps) -> Output,
    ) -> Output {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime, props.as_ref());
            cx.notify();
            output
        })
    }

    /// Run a refresh command that reports whether anything changed, and
    /// notify only when it did. Use from render-top reconciliation and
    /// layout observation so repaints settle once state is stable.
    pub fn refresh(
        &self,
        cx: &mut App,
        refresh: impl FnOnce(&mut ScrollAreaRuntime, &ScrollAreaProps) -> bool,
    ) -> bool {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            let changed = refresh(runtime, props.as_ref());
            if changed {
                cx.notify();
            }
            changed
        })
    }
}
