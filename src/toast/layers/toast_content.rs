use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::toast::child_wiring::ToastPartNode;
use crate::toast::{ToastContentStyleState, ToastContext, ToastId, ToastRootChild};

type ToastContentStyle = Rc<dyn Fn(ToastContentStyleState, Div) -> Div + 'static>;

/// The toast content container: watches its own prepainted size and reports
/// it into the runtime (the GPUI replacement for Base UI's
/// `ResizeObserver`/`MutationObserver` height watch), exposing `expanded` and
/// `behind` style state.
#[derive(IntoElement)]
pub struct ToastContent<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<ToastRootChild<P>>,
    context: Option<ToastContext<P>>,
    toast_id: Option<ToastId>,
    style_with_state: Option<ToastContentStyle>,
}

impl<P: Clone + 'static> Default for ToastContent<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            toast_id: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for ToastContent<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastPartNode<P> for ToastContent<P> {
    fn with_toast(mut self, context: ToastContext<P>, id: ToastId) -> Self {
        self.context = Some(context.clone());
        self.toast_id = Some(id.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_toast(context.clone(), id.clone()))
            .collect();
        self
    }
}

impl<P: Clone + 'static> RenderOnce for ToastContent<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(context), Some(toast_id)) = (self.context, self.toast_id) else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, _| runtime.content_state(&toast_id));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        base.on_children_prepainted(move |bounds, _window, cx| {
            if let Some(first) = bounds.first() {
                let mut union = *first;
                for child_bounds in bounds.iter().skip(1) {
                    union = union.union(child_bounds);
                }
                context.update(cx, |runtime| {
                    runtime.set_toast_height(&toast_id, f32::from(union.size.height))
                });
            }
        })
        .children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
        .into_any_element()
    }
}

impl<P: Clone + 'static> ToastContent<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ToastRootChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ToastRootChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToastContentStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
