use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Text, Window,
};

use crate::toast::child_wiring::ToastPartNode;
use crate::toast::{ToastActionStyleState, ToastContext, ToastId};

type ToastActionStyle = Rc<dyn Fn(ToastActionStyleState, Div) -> Div + 'static>;

/// The toast action button: renders the record's typed action definition (its
/// own children override the label), invokes the action callback on
/// activation, and renders nothing when no action content exists.
#[derive(IntoElement)]
pub struct ToastAction<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ToastContext<P>>,
    toast_id: Option<ToastId>,
    style_with_state: Option<ToastActionStyle>,
}

impl<P: Clone + 'static> Default for ToastAction<P> {
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

impl<P: Clone + 'static> Styled for ToastAction<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastPartNode<P> for ToastAction<P> {
    fn with_toast(mut self, context: ToastContext<P>, id: ToastId) -> Self {
        self.context = Some(context);
        self.toast_id = Some(id);
        self
    }
}

impl<P: Clone + 'static> RenderOnce for ToastAction<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(context), Some(toast_id)) = (self.context, self.toast_id) else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, _| runtime.action_state(&toast_id));
        let Some(action) = state.action.clone() else {
            if self.children.is_empty() {
                return div().into_any_element();
            }
            return div().children(self.children).into_any_element();
        };
        let element_id = ElementId::Name(SharedString::from(format!(
            "toast-action-{}",
            toast_id.as_str()
        )));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        let on_click = action.on_click();
        let base = base
            .id(element_id)
            .role(Role::Button)
            .aria_label(action.label())
            .focusable()
            .tab_stop(true)
            .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                cx.stop_propagation();
            })
            .on_click(move |_event, window, cx| {
                on_click(window, cx);
            });
        if self.children.is_empty() {
            // The button already carries the label via `.aria_label`; render
            // the visible text inaccessibly to avoid double-announcing.
            base.child(Text::new_inaccessible(action.label()))
                .into_any_element()
        } else {
            base.children(self.children).into_any_element()
        }
    }
}

impl<P: Clone + 'static> ToastAction<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToastActionStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
