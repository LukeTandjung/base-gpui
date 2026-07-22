use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::collapsible::{
    CollapsibleOpenChangeDetails, CollapsibleOpenChangeReason, CollapsibleOpenChangeSource,
    CollapsibleProps, CollapsibleRuntime,
};

pub struct CollapsibleContext {
    runtime: Entity<CollapsibleRuntime>,
    props: Rc<CollapsibleProps>,
    controlled: Rc<Option<bool>>,
}

impl Clone for CollapsibleContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl CollapsibleContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<bool>,
        default: bool,
        props: CollapsibleProps,
    ) -> Self {
        let open = controlled.or(Some(default));
        let runtime = window.use_keyed_state(id, cx, |_, _| CollapsibleRuntime::new(open));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&CollapsibleRuntime, &CollapsibleProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut CollapsibleRuntime) -> Output,
    ) -> Output {
        let controlled = *self.controlled.as_ref();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(open) = controlled {
                runtime.sync_open_from_context(Some(open));
            }

            let output = update(runtime);

            if let Some(open) = controlled {
                runtime.sync_open_from_context(Some(open));
            }

            cx.notify();
            output
        })
    }

    pub fn toggle(&self, source: CollapsibleOpenChangeSource, window: &mut Window, cx: &mut App) {
        let controlled = *self.controlled.as_ref();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            let current = controlled.or(runtime.open_value());

            runtime.sync_open_from_context(current);
            let outcome = runtime.request_toggle(props.disabled());

            if let Some(open) = controlled {
                runtime.sync_open_from_context(Some(open));
            }

            outcome
        });

        if !outcome.changed() {
            return;
        }

        let next_open = outcome.open();
        let mut details = CollapsibleOpenChangeDetails::new(
            CollapsibleOpenChangeReason::TriggerPress,
            source,
            true,
        );

        if let Some(on_open_change) = self.props.on_open_change() {
            on_open_change(next_open, &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if controlled.is_none() {
            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_open(next_open);
                cx.notify();
            });
        }
    }
}
