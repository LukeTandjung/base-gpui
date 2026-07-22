use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, Window};

use crate::preview_card::{
    scoped_trigger_id, PreviewCardOpenChangeDetails, PreviewCardOpenChangeReason,
    PreviewCardOpenChangeSource, PreviewCardProps, PreviewCardRuntime,
};

pub struct PreviewCardContext<P: Clone + 'static> {
    id: ElementId,
    runtime: Entity<PreviewCardRuntime<P>>,
    props: Rc<PreviewCardProps<P>>,
    controlled_open: Rc<Option<bool>>,
    controlled_trigger_id: Rc<Option<Option<ElementId>>>,
}

impl<P: Clone + 'static> Clone for PreviewCardContext<P> {
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

impl<P: Clone + 'static> PreviewCardContext<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled_open: Option<bool>,
        default_open: bool,
        controlled_trigger_id: Option<Option<ElementId>>,
        default_trigger_id: Option<ElementId>,
        props: PreviewCardProps<P>,
    ) -> Self {
        let id = id.into();
        let open = controlled_open.unwrap_or(default_open);
        let active_trigger_id = controlled_trigger_id.clone().unwrap_or(default_trigger_id);
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            PreviewCardRuntime::new(open, active_trigger_id)
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
        read: impl FnOnce(&PreviewCardRuntime<P>, &PreviewCardProps<P>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut PreviewCardRuntime<P>) -> Output,
    ) -> Output {
        let controlled_open = *self.controlled_open.as_ref();
        let controlled_trigger_id = self.controlled_trigger_id.as_ref().clone();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            if let Some(trigger_id) = controlled_trigger_id.clone() {
                runtime.sync_trigger_id_from_context(trigger_id);
            }

            let output = update(runtime);

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

    /// The one value-changing method: resolves controlled vs uncontrolled
    /// open, fires `on_open_change` (cancelable in uncontrolled mode),
    /// commits, records the instant classification, and fires
    /// `on_open_change_complete` (immediately: no transition infrastructure).
    pub fn set_open(
        &self,
        next_open: bool,
        trigger_id: Option<ElementId>,
        reason: PreviewCardOpenChangeReason,
        source: PreviewCardOpenChangeSource,
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
        let mut details = PreviewCardOpenChangeDetails::new(
            reason,
            source,
            source_trigger_id.clone(),
            payload,
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
            runtime.commit_open(
                open,
                scoped_trigger_id.clone(),
                source,
                prevent_unmount,
                controlled_open.is_none(),
                controlled_trigger_id.is_none(),
            );
            runtime.record_open_change(reason, source);
            cx.notify();
        });

        if let Some(on_open_change_complete) = self.props.on_open_change_complete() {
            on_open_change_complete(open, &details, window, cx);
        }

        true
    }

    pub fn open_trigger(
        &self,
        trigger_id: ElementId,
        reason: PreviewCardOpenChangeReason,
        source: PreviewCardOpenChangeSource,
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
        reason: PreviewCardOpenChangeReason,
        source: PreviewCardOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_open(false, None, reason, source, window, cx)
    }
}

/// Late-bound handle connecting detached triggers and imperative callers to
/// a root. `open` returns a recoverable `false` for an unknown trigger id
/// (documented deviation from Base UI's throw).
pub struct PreviewCardHandle<P: Clone + 'static>(Rc<RefCell<Option<PreviewCardHandleState<P>>>>);

impl<P: Clone + 'static> Clone for PreviewCardHandle<P> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<P: Clone + 'static> Default for PreviewCardHandle<P> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

struct PreviewCardHandleState<P: Clone + 'static> {
    context: PreviewCardContext<P>,
}

impl<P: Clone + 'static> PreviewCardHandle<P> {
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
            PreviewCardOpenChangeReason::ImperativeAction,
            PreviewCardOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    pub fn close(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        context.close(
            PreviewCardOpenChangeReason::ImperativeAction,
            PreviewCardOpenChangeSource::Imperative,
            window,
            cx,
        )
    }

    /// Base UI `actionsRef.unmount()`: closes and drops any keep-mounted
    /// grace so the popup unmounts.
    pub fn unmount(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(context) = self.context() else {
            return false;
        };
        let closed = context.close(
            PreviewCardOpenChangeReason::ImperativeAction,
            PreviewCardOpenChangeSource::Imperative,
            window,
            cx,
        );
        let cleared = context.update(cx, |runtime| runtime.clear_prevent_unmount());
        closed || cleared
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.context()
            .map(|context| context.read(cx, |runtime, _| runtime.open_value()))
            .unwrap_or(false)
    }

    pub fn bind(&self, context: PreviewCardContext<P>) {
        *self.0.borrow_mut() = Some(PreviewCardHandleState { context });
    }

    pub fn context(&self) -> Option<PreviewCardContext<P>> {
        self.0.borrow().as_ref().map(|state| state.context.clone())
    }
}

pub fn create_preview_card_handle<P: Clone + 'static>() -> PreviewCardHandle<P> {
    PreviewCardHandle::new()
}
