use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Text, Window,
};

use crate::toast::child_wiring::ToastPartNode;
use crate::toast::{ToastContext, ToastId, ToastTitleStyleState};

type ToastTitleStyle = Rc<dyn Fn(ToastTitleStyleState, Div) -> Div + 'static>;

/// The toast title: renders the toast record's title by default (explicit
/// children override); renders nothing when there is no title content.
#[derive(IntoElement)]
pub struct ToastTitle<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ToastContext<P>>,
    toast_id: Option<ToastId>,
    style_with_state: Option<ToastTitleStyle>,
}

impl<P: Clone + 'static> Default for ToastTitle<P> {
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

impl<P: Clone + 'static> Styled for ToastTitle<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastPartNode<P> for ToastTitle<P> {
    fn with_toast(mut self, context: ToastContext<P>, id: ToastId) -> Self {
        self.context = Some(context);
        self.toast_id = Some(id);
        self
    }
}

impl<P: Clone + 'static> RenderOnce for ToastTitle<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(context), Some(toast_id)) = (self.context, self.toast_id) else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, _| runtime.title_state(&toast_id));
        if self.children.is_empty() && state.title.is_none() {
            return div().into_any_element();
        }
        let default_title = state.title.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        if self.children.is_empty() {
            // The root carries a flattened `.aria_label` built from the
            // record's title/description, so the default text renders
            // inaccessibly to avoid double-announcing. Caller-supplied
            // children are left alone.
            base.child(Text::new_inaccessible(default_title.unwrap_or_default()))
                .into_any_element()
        } else {
            base.children(self.children).into_any_element()
        }
    }
}

impl<P: Clone + 'static> ToastTitle<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToastTitleStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
