use gpui::{
    anchored, deferred, div, point, px, App, Div, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::toast::child_wiring::ToastContextNode;
use crate::toast::{ToastContext, ToastPortalChild};

/// Hoists the toast viewport above app content via GPUI deferred + anchored
/// rendering (the `dialog_portal.rs` pattern). Draws nothing while no toasts
/// are mounted. Portal priority is 2 — above the dialog/drawer overlay
/// priority (1) so toasts stack over open dialogs.
#[derive(IntoElement)]
pub struct ToastPortal<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<ToastPortalChild<P>>,
    context: Option<ToastContext<P>>,
}

impl<P: Clone + 'static> Default for ToastPortal<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
        }
    }
}

impl<P: Clone + 'static> Styled for ToastPortal<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastContextNode<P> for ToastPortal<P> {
    fn with_toast_context(mut self, context: ToastContext<P>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_toast_context(context.clone()))
            .collect();
        self
    }
}

impl<P: Clone + 'static> RenderOnce for ToastPortal<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.as_ref() else {
            return div().into_any_element();
        };
        let mounted = context.read(cx, |runtime, _| runtime.viewport_state().toast_count > 0);
        if !mounted {
            return div().into_any_element();
        }
        let viewport_size = window.viewport_size();
        let base = self
            .base
            .absolute()
            .top_0()
            .left_0()
            .w(viewport_size.width)
            .h(viewport_size.height)
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );
        div()
            .child(deferred(anchored().position(point(px(0.0), px(0.0))).child(base)).priority(2))
            .into_any_element()
    }
}

impl<P: Clone + 'static> ToastPortal<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ToastPortalChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ToastPortalChild::Any(child.into_any_element()));
        self
    }
}
