use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, Window};

use crate::dialog::{
    scoped_dialog_trigger_id, DialogOpenChangeDetails, DialogOpenChangeReason,
    DialogOpenChangeSource, DialogProps, DialogRuntime,
};

pub struct DialogContext<P: Clone + 'static> {
    id: ElementId,
    runtime: Entity<DialogRuntime<P>>,
    props: Rc<DialogProps<P>>,
    controlled_open: Rc<Option<bool>>,
    controlled_trigger_id: Rc<Option<Option<ElementId>>>,
}

impl<P: Clone + 'static> Clone for DialogContext<P> {
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

impl<P: Clone + 'static> DialogContext<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled_open: Option<bool>,
        default_open: bool,
        controlled_trigger_id: Option<Option<ElementId>>,
        default_trigger_id: Option<ElementId>,
        props: DialogProps<P>,
    ) -> Self {
        let id = id.into();
        let open = controlled_open.unwrap_or(default_open);
        let active_trigger_id = controlled_trigger_id.clone().unwrap_or(default_trigger_id);
        let modal_mode = props.modal_mode();
        let disable_pointer_dismissal = props.disable_pointer_dismissal();
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            DialogRuntime::new(
                open,
                active_trigger_id,
                modal_mode,
                disable_pointer_dismissal,
            )
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
        scoped_dialog_trigger_id(&self.id, trigger_id)
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&DialogRuntime<P>, &DialogProps<P>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut DialogRuntime<P>) -> Output,
    ) -> Output {
        let controlled_open = *self.controlled_open.as_ref();
        let controlled_trigger_id = self.controlled_trigger_id.as_ref().clone();
        let props = Rc::clone(&self.props);

        self.runtime.update(cx, |runtime, cx| {
            runtime.sync_props(props.as_ref());
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            if let Some(trigger_id) = controlled_trigger_id.clone() {
                runtime.sync_trigger_id_from_context(trigger_id);
            }

            let output = update(runtime);

            runtime.sync_props(props.as_ref());
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
        reason: DialogOpenChangeReason,
        source: DialogOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let controlled_open = *self.controlled_open.as_ref();
        let controlled_trigger_id = self.controlled_trigger_id.as_ref().clone();
        let previous_focus = if next_open { window.focused(cx) } else { None };
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
                runtime.request_close(current)
            }
        });

        self.apply_outcome(
            outcome,
            previous_focus,
            reason,
            source,
            controlled_open,
            controlled_trigger_id,
            window,
            cx,
        )
    }

    pub fn open_with_payload(
        &self,
        payload: P,
        reason: DialogOpenChangeReason,
        source: DialogOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let controlled_open = *self.controlled_open.as_ref();
        let controlled_trigger_id = self.controlled_trigger_id.as_ref().clone();
        let previous_focus = window.focused(cx);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            let current = controlled_open.unwrap_or_else(|| runtime.open_value());
            runtime.request_open_with_payload(current, payload)
        });

        self.apply_outcome(
            outcome,
            previous_focus,
            reason,
            source,
            controlled_open,
            controlled_trigger_id,
            window,
            cx,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn apply_outcome(
        &self,
        outcome: crate::dialog::DialogOpenChangeOutcome<P>,
        previous_focus: Option<gpui::FocusHandle>,
        reason: DialogOpenChangeReason,
        source: DialogOpenChangeSource,
        controlled_open: Option<bool>,
        controlled_trigger_id: Option<Option<ElementId>>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if !outcome.changed() {
            return false;
        }

        let (open, active_trigger_id, source_trigger_id, payload) = outcome.into_parts();
        let mut details = DialogOpenChangeDetails::new(
            reason,
            source,
            source_trigger_id.clone(),
            payload.clone(),
            true,
        );

        if let Some(on_open_change) = self.props.on_open_change() {
            on_open_change(open, &mut details, window, cx);
        }

        if details.is_canceled() {
            return false;
        }

        let prevent_unmount = !open && details.prevents_unmount_on_close();
        self.runtime.update(cx, |runtime, cx| {
            if open {
                runtime.capture_previous_focus(previous_focus);
            }
            runtime.commit_open(
                open,
                active_trigger_id.clone(),
                payload.clone(),
                prevent_unmount,
                controlled_open.is_none(),
                controlled_trigger_id.is_none(),
            );
            cx.notify();
        });

        if let Some(on_open_change_complete) = self.props.on_open_change_complete() {
            on_open_change_complete(open, &details, window, cx);
        }

        if !open && reason != DialogOpenChangeReason::FocusOut {
            let restore_focus = self.read(cx, |runtime, _| {
                runtime
                    .active_trigger_focus_handle()
                    .or_else(|| runtime.previous_focus_handle())
            });
            if let Some(focus_handle) = restore_focus {
                focus_handle.focus(window, cx);
            }
        }

        true
    }

    pub fn open_trigger(
        &self,
        trigger_id: ElementId,
        reason: DialogOpenChangeReason,
        source: DialogOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_open(true, Some(trigger_id), reason, source, window, cx)
    }

    pub fn close(
        &self,
        reason: DialogOpenChangeReason,
        source: DialogOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_open(false, None, reason, source, window, cx)
    }

    pub fn focus_popup_neighbor(&self, reverse: bool, window: &mut Window, cx: &mut App) -> bool {
        let current_focus = window.focused(cx);
        let next_focus = self.read(cx, |runtime, _| {
            runtime.popup_focus_neighbor(current_focus.as_ref(), reverse)
        });
        if let Some(next_focus) = next_focus {
            next_focus.focus(window, cx);
            true
        } else {
            false
        }
    }
}

pub struct DialogHandle<P: Clone + 'static>(Rc<RefCell<Option<DialogHandleState<P>>>>);

impl<P: Clone + 'static> Clone for DialogHandle<P> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<P: Clone + 'static> Default for DialogHandle<P> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

struct DialogHandleState<P: Clone + 'static> {
    context: DialogContext<P>,
}

impl<P: Clone + 'static> DialogHandle<P> {
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
            DialogOpenChangeReason::ImperativeAction,
            DialogOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn open_with_payload(&self, payload: P, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        context.open_with_payload(
            payload,
            DialogOpenChangeReason::ImperativeAction,
            DialogOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn close(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        context.close(
            DialogOpenChangeReason::ImperativeAction,
            DialogOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn unmount(&self, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        context.update(cx, |runtime| runtime.force_unmount());
        true
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.context()
            .map(|context| context.read(cx, |runtime, _| runtime.open_value()))
            .unwrap_or(false)
    }

    pub fn bind(&self, context: DialogContext<P>) {
        *self.0.borrow_mut() = Some(DialogHandleState { context });
    }

    pub fn context(&self) -> Option<DialogContext<P>> {
        self.0.borrow().as_ref().map(|state| state.context.clone())
    }
}

pub fn create_dialog_handle<P: Clone + 'static>() -> DialogHandle<P> {
    DialogHandle::new()
}
