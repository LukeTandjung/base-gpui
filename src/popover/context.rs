use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, Window};

use crate::popover::{
    scoped_trigger_id, PopoverOpenChangeDetails, PopoverOpenChangeReason, PopoverOpenChangeSource,
    PopoverProps, PopoverRuntime,
};

pub struct PopoverContext<P: Clone + 'static> {
    id: ElementId,
    runtime: Entity<PopoverRuntime<P>>,
    props: Rc<PopoverProps<P>>,
    controlled_open: Rc<Option<bool>>,
    controlled_trigger_id: Rc<Option<Option<ElementId>>>,
}

impl<P: Clone + 'static> Clone for PopoverContext<P> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled_open: Rc::clone(&self.controlled_open),
            controlled_trigger_id: Rc::clone(&self.controlled_trigger_id),
        }
    }
}

impl<P: Clone + 'static> PopoverContext<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled_open: Option<bool>,
        default_open: bool,
        controlled_trigger_id: Option<Option<ElementId>>,
        default_trigger_id: Option<ElementId>,
        props: PopoverProps<P>,
    ) -> Self {
        let id = id.into();
        let open = controlled_open.unwrap_or(default_open);
        let active_trigger_id = controlled_trigger_id.clone().unwrap_or(default_trigger_id);
        let modal = props.modal();
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            PopoverRuntime::new(open, active_trigger_id, modal)
        });

        Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled_open: Rc::new(controlled_open),
            controlled_trigger_id: Rc::new(controlled_trigger_id),
        }
    }

    pub fn root_id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn scope_trigger_id(&self, trigger_id: &ElementId) -> ElementId {
        scoped_trigger_id(&self.id, trigger_id)
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&PopoverRuntime<P>, &PopoverProps<P>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut PopoverRuntime<P>) -> Output,
    ) -> Output {
        let controlled_open = *self.controlled_open.as_ref();
        let controlled_trigger_id = self.controlled_trigger_id.as_ref().clone();
        let modal = self.props.modal();

        self.runtime.update(cx, |runtime, cx| {
            runtime.sync_modal(modal);
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            if let Some(trigger_id) = controlled_trigger_id.clone() {
                runtime.sync_trigger_id_from_context(trigger_id);
            }

            let output = update(runtime);

            runtime.sync_modal(modal);
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            if let Some(trigger_id) = controlled_trigger_id {
                runtime.sync_trigger_id_from_context(trigger_id);
            }

            cx.notify();
            output
        })
    }

    pub fn set_open(
        &self,
        next_open: bool,
        trigger_id: Option<ElementId>,
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let controlled_open = *self.controlled_open.as_ref();
        let controlled_trigger_id = self.controlled_trigger_id.as_ref().clone();
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            if let Some(trigger_id) = controlled_trigger_id.clone() {
                runtime.sync_trigger_id_from_context(trigger_id);
            }
            let current = controlled_open.unwrap_or_else(|| runtime.open_value());
            if next_open {
                runtime.request_open(current, trigger_id)
            } else {
                runtime.request_close(current, trigger_id)
            }
        });

        if !outcome.changed() {
            return false;
        }

        let (open, scoped_trigger_id, source_trigger_id, payload) = outcome.into_parts();
        let mut details =
            PopoverOpenChangeDetails::new(reason, source, source_trigger_id.clone(), payload, true);
        if let Some(on_open_change) = self.props.on_open_change() {
            on_open_change(open, &mut details, window, cx);
        }

        if details.is_canceled() {
            return false;
        }

        let prevent_unmount = !open && details.prevents_unmount_on_close();
        self.runtime.update(cx, |runtime, cx| {
            runtime.commit_open(
                open,
                scoped_trigger_id.clone(),
                source,
                prevent_unmount,
                controlled_open.is_none(),
                controlled_trigger_id.is_none(),
            );
            runtime.record_open_change(reason, source);
            if open {
                runtime.request_popup_focus_on_open(reason, source);
            }
            cx.notify();
        });

        if let Some(on_open_change_complete) = self.props.on_open_change_complete() {
            on_open_change_complete(open, &details, window, cx);
        }

        if !open && reason != PopoverOpenChangeReason::FocusOut {
            if let Some(focus_handle) =
                self.read(cx, |runtime, _| runtime.active_trigger_focus_handle())
            {
                focus_handle.focus(window, cx);
            }
        }

        true
    }

    pub fn toggle_trigger(
        &self,
        trigger_id: ElementId,
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let should_close = self.read(cx, |runtime, _| {
            runtime.open_value() && runtime.active_trigger_id().as_ref() == Some(&trigger_id)
        });
        self.set_open(!should_close, Some(trigger_id), reason, source, window, cx)
    }

    pub fn open_trigger(
        &self,
        trigger_id: ElementId,
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if !self.read(cx, |runtime, _| runtime.can_open_trigger(&trigger_id)) {
            return false;
        }

        self.set_open(true, Some(trigger_id), reason, source, window, cx)
    }

    pub fn close(
        &self,
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_open(false, None, reason, source, window, cx)
    }
}

pub struct PopoverHandle<P: Clone + 'static>(Rc<RefCell<Option<PopoverHandleState<P>>>>);

impl<P: Clone + 'static> Clone for PopoverHandle<P> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<P: Clone + 'static> Default for PopoverHandle<P> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

struct PopoverHandleState<P: Clone + 'static> {
    context: PopoverContext<P>,
}

impl<P: Clone + 'static> PopoverHandle<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(
        &self,
        trigger_id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let scoped_trigger_id = context.scope_trigger_id(&trigger_id.into());
        context.open_trigger(
            scoped_trigger_id,
            PopoverOpenChangeReason::ImperativeAction,
            PopoverOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn close(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        context.close(
            PopoverOpenChangeReason::ImperativeAction,
            PopoverOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn toggle(
        &self,
        trigger_id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let scoped_trigger_id = context.scope_trigger_id(&trigger_id.into());
        context.toggle_trigger(
            scoped_trigger_id,
            PopoverOpenChangeReason::ImperativeAction,
            PopoverOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.context()
            .map(|context| context.read(cx, |runtime, _| runtime.open_value()))
            .unwrap_or(false)
    }

    pub fn bind(&self, context: PopoverContext<P>) {
        *self.0.borrow_mut() = Some(PopoverHandleState { context });
    }

    pub fn context(&self) -> Option<PopoverContext<P>> {
        self.0.borrow().as_ref().map(|state| state.context.clone())
    }
}

pub fn create_popover_handle<P: Clone + 'static>() -> PopoverHandle<P> {
    PopoverHandle::new()
}
