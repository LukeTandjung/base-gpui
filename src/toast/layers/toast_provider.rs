use std::time::{Duration, Instant};

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::toast::{
    ToastContext, ToastFocusViewportAction, ToastManager, ToastProviderChild, ToastProviderProps,
    ToastRuntime, TOAST_DEFAULT_LIMIT, TOAST_DEFAULT_TIMEOUT,
};

/// The per-subtree toast runtime owner (Base UI `ToastProvider` / per-app
/// `ToastStore` analog): creates the keyed runtime entity, syncs
/// `timeout`/`limit`, binds an optional imperative `ToastManager`, observes
/// window activation as the window blur/focus analog, and handles the F6
/// focus-viewport action. Renders no styled element of its own — just a
/// passthrough container for its children.
#[derive(IntoElement)]
pub struct ToastProvider<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<ToastProviderChild<P>>,
    timeout: Duration,
    limit: usize,
    manager: Option<ToastManager<P>>,
}

impl<P: Clone + 'static> Default for ToastProvider<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("toast-provider"),
            base: div(),
            children: Vec::new(),
            timeout: TOAST_DEFAULT_TIMEOUT,
            limit: TOAST_DEFAULT_LIMIT,
            manager: None,
        }
    }
}

impl<P: Clone + 'static> Styled for ToastProvider<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for ToastProvider<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let runtime =
            window.use_keyed_state(ElementId::from((self.id.clone(), "runtime")), cx, |_, _| {
                ToastRuntime::<P>::new()
            });
        let viewport_focus = window
            .use_keyed_state(
                ElementId::from((self.id.clone(), "viewport-focus")),
                cx,
                |_, cx| cx.focus_handle(),
            )
            .read(cx)
            .clone();

        let context = ToastContext::new(
            runtime,
            ToastProviderProps::new(self.timeout, self.limit),
            viewport_focus.clone(),
        );

        // Window activation is the `window blur/focus` analog: observed at
        // render time (no dedicated activation subscription in this pass).
        let window_active = window.is_window_active();
        let ops = context.update(cx, |runtime| {
            runtime.set_window_focused(window_active, Instant::now())
        });
        context.process_timer_ops(ops, cx);

        if let Some(manager) = self.manager.as_ref() {
            manager.bind(context.clone(), cx);
        }

        let focus_context = context.clone();
        self.base
            .id(self.id)
            .on_action(move |_: &ToastFocusViewportAction, window, cx| {
                if focus_context.toasts(cx).is_empty() {
                    return;
                }
                let previous = window.focused(cx);
                let ops = focus_context.update(cx, |runtime| {
                    runtime.record_previous_focus(previous);
                    runtime.set_focused(true, Instant::now())
                });
                focus_context.process_timer_ops(ops, cx);
                window.focus(&focus_context.viewport_focus(), cx);
            })
            .children(
                self.children
                    .into_iter()
                    .map(|child| child.with_toast_context(context.clone()))
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            )
    }
}

impl<P: Clone + 'static> ToastProvider<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<ToastProviderChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ToastProviderChild::Any(child.into_any_element()));
        self
    }

    /// Default auto-dismiss timeout for toasts without their own (5000 ms).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Maximum non-limited toasts (older ones are flagged `limited`; default 3).
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    /// Binds an imperative manager created with `create_toast_manager`.
    pub fn manager(mut self, manager: ToastManager<P>) -> Self {
        self.manager = Some(manager);
        self
    }
}
