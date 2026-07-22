use std::rc::Rc;
use std::time::Instant;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::toast::child_wiring::{ToastContextNode, ToastPartNode};
use crate::toast::{
    ToastContext, ToastExitViewportAction, ToastFacts, ToastRoot, ToastViewportStyleState,
    TOAST_VIEWPORT_KEY_CONTEXT,
};

type ToastViewportStyle = Rc<dyn Fn(ToastViewportStyleState, Div) -> Div + 'static>;
type ToastContentBuilder<P> = Rc<dyn Fn(&ToastFacts<P>) -> ToastRoot<P> + 'static>;

/// The toast stack region: renders one `ToastRoot` per toast through the typed
/// content builder (the explicit Rust replacement for Base UI's render-prop
/// children), pauses all dismiss timers while hovered or keyboard-focused, and
/// exits focus via Shift+Tab restoring the previously focused element.
#[derive(IntoElement)]
pub struct ToastViewport<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    context: Option<ToastContext<P>>,
    content_builder: Option<ToastContentBuilder<P>>,
    aria_label: SharedString,
    style_with_state: Option<ToastViewportStyle>,
}

impl<P: Clone + 'static> Default for ToastViewport<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("toast-viewport"),
            base: div(),
            context: None,
            content_builder: None,
            aria_label: SharedString::from("Notifications"),
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for ToastViewport<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastContextNode<P> for ToastViewport<P> {
    fn with_toast_context(mut self, context: ToastContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> RenderOnce for ToastViewport<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, _| runtime.viewport_state());
        let toasts = context.toasts(cx);

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let hover_context = context.clone();
        let exit_context = context.clone();
        let viewport_focus = context.viewport_focus();

        let children: Vec<_> = match self.content_builder.as_ref() {
            Some(builder) => toasts
                .iter()
                .map(|facts| {
                    builder(facts)
                        .with_toast(context.clone(), facts.id.clone())
                        .into_any_element()
                })
                .collect(),
            None => Vec::new(),
        };

        base.id(self.id)
            // AccessKit gap in this gpui revision: no live-region API
            // (`aria-live`/`aria-atomic`/`aria-relevant` and Base UI's hidden
            // duplicated `role="alert"` tree), so added toasts are not
            // auto-announced; AT users rely on discovering the region.
            .role(Role::Region)
            .aria_label(self.aria_label)
            .on_hover(move |hovered, _window, cx| {
                // Hover pauses every dismiss timer; leave resumes with the
                // recorded remaining durations. NOTE: with immediate removal
                // after `ending` there is no deferred `flushMouseLeave` phase
                // to wait for in this pass.
                let ops = hover_context
                    .update(cx, |runtime| runtime.set_hovering(*hovered, Instant::now()));
                hover_context.process_timer_ops(ops, cx);
            })
            .key_context(TOAST_VIEWPORT_KEY_CONTEXT)
            .track_focus(&viewport_focus)
            .on_action(move |_: &ToastExitViewportAction, window, cx| {
                let (previous, ops) = exit_context.update(cx, |runtime| {
                    (
                        runtime.take_previous_focus(),
                        runtime.set_focused(false, Instant::now()),
                    )
                });
                exit_context.process_timer_ops(ops, cx);
                if let Some(previous) = previous {
                    previous.focus(window, cx);
                }
            })
            .children(children)
            .into_any_element()
    }
}

impl<P: Clone + 'static> ToastViewport<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Accessible label for the notifications region; defaults to
    /// "Notifications" (Base UI viewport `aria-label` parity).
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = label.into();
        self
    }

    /// The typed per-toast content builder: receives typed toast facts and
    /// returns the `ToastRoot` subtree for that toast.
    pub fn content_builder(
        mut self,
        builder: impl Fn(&ToastFacts<P>) -> ToastRoot<P> + 'static,
    ) -> Self {
        self.content_builder = Some(Rc::new(builder));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToastViewportStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
