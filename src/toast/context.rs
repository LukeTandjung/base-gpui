use std::future::Future;
use std::rc::Rc;
use std::time::Instant;

use gpui::{App, Entity, FocusHandle, SharedString};

use crate::toast::{
    ToastFacts, ToastId, ToastOptions, ToastProviderProps, ToastRuntime, ToastTimerOp, ToastType,
};

/// Loading/success/error option forms for a promise toast: plain text or full
/// options, with success/error computed from the resolved value/error (the
/// Rust-closure replacement for Base UI's `resolvePromiseOptions`).
pub type ToastPromiseResolve<P, V> = Box<dyn FnOnce(&V) -> ToastOptions<P> + 'static>;

pub struct ToastPromiseOptions<P: Clone + 'static, T: 'static, E: 'static> {
    pub loading: ToastOptions<P>,
    pub success: ToastPromiseResolve<P, T>,
    pub error: ToastPromiseResolve<P, E>,
}

impl<P: Clone + 'static, T: 'static, E: 'static> ToastPromiseOptions<P, T, E> {
    pub fn new(
        loading: ToastOptions<P>,
        success: impl FnOnce(&T) -> ToastOptions<P> + 'static,
        error: impl FnOnce(&E) -> ToastOptions<P> + 'static,
    ) -> Self {
        Self {
            loading,
            success: Box::new(success),
            error: Box::new(error),
        }
    }

    /// The plain-text form: fixed titles for each phase.
    pub fn from_text(
        loading: impl Into<SharedString>,
        success: impl Into<SharedString>,
        error: impl Into<SharedString>,
    ) -> Self {
        let loading = loading.into();
        let success = success.into();
        let error = error.into();
        Self::new(
            ToastOptions::new().title(loading),
            move |_| ToastOptions::new().title(success),
            move |_| ToastOptions::new().title(error),
        )
    }
}

/// Thin injection vehicle for toast parts: the provider-owned runtime entity,
/// provider props, and the viewport focus handle. Value-changing wrappers
/// (`add` / `close` / `update_toast` / `promise`) resolve runtime outcomes
/// into user callbacks and generation-counted background timers; the GPUI
/// `useToastManager` analog for code that already has `&mut App`.
pub struct ToastContext<P: Clone + 'static> {
    runtime: Entity<ToastRuntime<P>>,
    props: Rc<ToastProviderProps>,
    viewport_focus: FocusHandle,
}

/// A weak counterpart to `ToastContext` held by `ToastManager` so a bound
/// manager outliving its provider window never leaks the runtime entity.
pub struct WeakToastContext<P: Clone + 'static> {
    runtime: gpui::WeakEntity<ToastRuntime<P>>,
    props: Rc<ToastProviderProps>,
    viewport_focus: FocusHandle,
}

impl<P: Clone + 'static> Clone for WeakToastContext<P> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            viewport_focus: self.viewport_focus.clone(),
        }
    }
}

impl<P: Clone + 'static> WeakToastContext<P> {
    /// The strong context, while the provider's runtime entity is alive.
    pub fn upgrade(&self) -> Option<ToastContext<P>> {
        self.runtime.upgrade().map(|runtime| ToastContext {
            runtime,
            props: Rc::clone(&self.props),
            viewport_focus: self.viewport_focus.clone(),
        })
    }
}

impl<P: Clone + 'static> Clone for ToastContext<P> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            viewport_focus: self.viewport_focus.clone(),
        }
    }
}

impl<P: Clone + 'static> ToastContext<P> {
    pub fn new(
        runtime: Entity<ToastRuntime<P>>,
        props: ToastProviderProps,
        viewport_focus: FocusHandle,
    ) -> Self {
        Self {
            runtime,
            props: Rc::new(props),
            viewport_focus,
        }
    }

    /// The focus handle owning viewport keyboard focus (F6 entry).
    pub fn viewport_focus(&self) -> FocusHandle {
        self.viewport_focus.clone()
    }

    /// A weak handle for long-lived holders (the imperative manager): does
    /// not keep the provider's runtime entity alive past its window.
    pub fn downgrade(&self) -> WeakToastContext<P> {
        WeakToastContext {
            runtime: self.runtime.downgrade(),
            props: Rc::clone(&self.props),
            viewport_focus: self.viewport_focus.clone(),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&ToastRuntime<P>, &ToastProviderProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut ToastRuntime<P>) -> Output,
    ) -> Output {
        let props = *self.props;
        self.runtime.update(cx, |runtime, cx| {
            runtime.sync_provider_props(props.timeout(), props.limit());
            let output = update(runtime);
            cx.notify();
            output
        })
    }

    /// Spawns generation-counted background timers for schedule ops; a fired
    /// timer with a stale generation is a no-op inside the runtime.
    pub fn process_timer_ops(&self, ops: Vec<ToastTimerOp>, cx: &mut App) {
        for op in ops {
            let ToastTimerOp::Schedule {
                id,
                generation,
                duration,
            } = op
            else {
                // Cancellation is generation-based: the stale task simply
                // no-ops when it fires.
                continue;
            };
            let context = self.clone();
            cx.spawn(async move |cx| {
                cx.background_executor().timer(duration).await;
                cx.update(|cx| {
                    context.handle_timer_fired(&id, generation, cx);
                });
            })
            .detach();
        }
    }

    fn handle_timer_fired(&self, id: &ToastId, generation: u64, cx: &mut App) {
        let outcome = self.update(cx, |runtime| runtime.timer_fired(id, generation));
        if let Some(outcome) = outcome {
            self.finish_close(outcome, cx);
        }
    }

    /// Fires `on_close` callbacks and completes removal. With no exit
    /// animation infrastructure active, removal is immediate after `ending`
    /// (documented divergence; the animation hookup is a follow-up).
    fn finish_close(&self, outcome: crate::toast::ToastCloseOutcome, cx: &mut App) {
        for on_close in &outcome.on_close {
            on_close(cx);
        }
        for id in &outcome.closed {
            let on_remove = self.update(cx, |runtime| runtime.remove_toast(id));
            if let Some(on_remove) = on_remove {
                on_remove(cx);
            }
        }
    }

    /// Adds (or upserts by id) a toast, returning its id.
    pub fn add(&self, options: ToastOptions<P>, cx: &mut App) -> ToastId {
        let now = Instant::now();
        let outcome = self.update(cx, |runtime| runtime.add_toast(options, now));
        self.process_timer_ops(outcome.timer_ops, cx);
        outcome.id
    }

    /// Closes one toast (`Some(id)`) or all toasts (`None`), firing `on_close`
    /// once per closed toast and `on_remove` on removal.
    pub fn close(&self, id: Option<&ToastId>, cx: &mut App) {
        let outcome = self.update(cx, |runtime| runtime.close_toast(id));
        self.finish_close(outcome, cx);
    }

    /// Partially updates a live toast; ignored for ending toasts.
    pub fn update_toast(&self, id: &ToastId, options: ToastOptions<P>, cx: &mut App) {
        let now = Instant::now();
        let ops = self.update(cx, |runtime| runtime.update_toast(id, options, now));
        self.process_timer_ops(ops, cx);
    }

    /// The public toast facts, newest first.
    pub fn toasts(&self, cx: &App) -> Vec<ToastFacts<P>> {
        self.read(cx, |runtime, _| runtime.toasts())
    }

    /// Adds a `Loading` toast (no timer), runs the future on GPUI executors,
    /// and updates the toast in place to the success/error options with a
    /// dismiss timer. The original output is preserved: the returned task
    /// yields the future's `Result`. A resolution arriving after the toast
    /// ended is ignored by the ending-update rule.
    pub fn promise<T: 'static, E: 'static>(
        &self,
        future: impl Future<Output = Result<T, E>> + 'static,
        options: ToastPromiseOptions<P, T, E>,
        cx: &mut App,
    ) -> gpui::Task<Result<T, E>> {
        let mut loading = options.loading;
        loading.toast_type = Some(ToastType::Loading);
        let id = self.add(loading, cx);
        let context = self.clone();
        let success = options.success;
        let error = options.error;
        cx.spawn(async move |cx| {
            let result = future.await;
            cx.update(|cx| {
                let mut update = match &result {
                    Ok(value) => {
                        let mut update = success(value);
                        update.toast_type = Some(ToastType::Success);
                        update
                    }
                    Err(err) => {
                        let mut update = error(err);
                        update.toast_type = Some(ToastType::Error);
                        update
                    }
                };
                update.id = Some(id.clone());
                context.update_toast(&id, update, cx);
            });
            result
        })
    }
}
