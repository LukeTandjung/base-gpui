use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use gpui::App;

use crate::toast::{
    ToastContext, ToastFacts, ToastId, ToastOptions, ToastPromiseOptions, ToastType,
    WeakToastContext,
};

enum ToastManagerOp<P: Clone + 'static> {
    Add(ToastOptions<P>),
    Close(Option<ToastId>),
    Update(ToastId, ToastOptions<P>),
}

struct ToastManagerState<P: Clone + 'static> {
    context: Option<WeakToastContext<P>>,
    queued: Vec<ToastManagerOp<P>>,
}

/// A late-bound imperative toast handle (the `createToastManager` analog):
/// creatable before any provider mounts and bound via
/// `ToastProvider::manager(...)`. Operations issued before a provider binds
/// are queued and flushed on bind (documented behavior — friendlier than Base
/// UI's drop-until-subscribe, never a panic). `add` assigns and returns the
/// toast id immediately, even while unbound.
pub struct ToastManager<P: Clone + 'static = ()>(Rc<RefCell<ToastManagerState<P>>>);

impl<P: Clone + 'static> Clone for ToastManager<P> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<P: Clone + 'static> Default for ToastManager<P> {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(ToastManagerState {
            context: None,
            queued: Vec::new(),
        })))
    }
}

impl<P: Clone + 'static> ToastManager<P> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds this manager to a provider's context and flushes queued ops.
    /// Called by `ToastProvider` during render; a single provider binding is
    /// assumed per manager — use one manager per provider subtree.
    pub fn bind(&self, context: ToastContext<P>, cx: &mut App) {
        let queued = {
            let mut state = self.0.borrow_mut();
            state.context = Some(context.downgrade());
            std::mem::take(&mut state.queued)
        };
        for op in queued {
            match op {
                ToastManagerOp::Add(options) => {
                    context.add(options, cx);
                }
                ToastManagerOp::Close(id) => context.close(id.as_ref(), cx),
                ToastManagerOp::Update(id, options) => context.update_toast(&id, options, cx),
            }
        }
    }

    fn context(&self) -> Option<ToastContext<P>> {
        self.0
            .borrow()
            .context
            .as_ref()
            .and_then(WeakToastContext::upgrade)
    }

    /// Whether a provider has bound this manager.
    pub fn bound(&self) -> bool {
        self.0.borrow().context.is_some()
    }

    /// Adds (or upserts) a toast, returning its id; queued until bound.
    pub fn add(&self, mut options: ToastOptions<P>, cx: &mut App) -> ToastId {
        let id = options.id.clone().unwrap_or_else(ToastId::generate);
        options.id = Some(id.clone());
        match self.context() {
            Some(context) => {
                context.add(options, cx);
            }
            None => self
                .0
                .borrow_mut()
                .queued
                .push(ToastManagerOp::Add(options)),
        }
        id
    }

    /// Closes one toast or all toasts; queued until bound.
    pub fn close(&self, id: Option<ToastId>, cx: &mut App) {
        match self.context() {
            Some(context) => context.close(id.as_ref(), cx),
            None => self.0.borrow_mut().queued.push(ToastManagerOp::Close(id)),
        }
    }

    /// Partially updates a live toast; queued until bound.
    pub fn update(&self, id: ToastId, options: ToastOptions<P>, cx: &mut App) {
        match self.context() {
            Some(context) => context.update_toast(&id, options, cx),
            None => self
                .0
                .borrow_mut()
                .queued
                .push(ToastManagerOp::Update(id, options)),
        }
    }

    /// Adds a `Loading` toast, runs the future, and transitions the toast to
    /// success/error in place. Works before binding: the loading add and the
    /// resolution update both route through this manager's queue. The
    /// returned task preserves the original `Result`.
    pub fn promise<T: 'static, E: 'static>(
        &self,
        future: impl Future<Output = Result<T, E>> + 'static,
        options: ToastPromiseOptions<P, T, E>,
        cx: &mut App,
    ) -> gpui::Task<Result<T, E>> {
        let mut loading = options.loading;
        loading.toast_type = Some(ToastType::Loading);
        let id = self.add(loading, cx);
        let manager = self.clone();
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
                manager.update(id.clone(), update, cx);
            });
            result
        })
    }

    /// The bound provider's toast facts; empty while unbound.
    pub fn toasts(&self, cx: &App) -> Vec<ToastFacts<P>> {
        self.context()
            .map(|context| context.toasts(cx))
            .unwrap_or_default()
    }
}

/// Creates a framework-independent toast manager usable before/outside the
/// provider render (Base UI `createToastManager` parity).
pub fn create_toast_manager<P: Clone + 'static>() -> ToastManager<P> {
    ToastManager::new()
}
