use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::toast::child_wiring::ToastPartNode;
use crate::toast::{ToastCloseStyleState, ToastContext, ToastId};

type ToastCloseStyle = Rc<dyn Fn(ToastCloseStyleState, Div) -> Div + 'static>;

/// The toast close button: a focusable GPUI interactive element closing its
/// own toast on click. Stops mouse-down propagation so a press never begins a
/// swipe on the root.
#[derive(IntoElement)]
pub struct ToastClose<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ToastContext<P>>,
    toast_id: Option<ToastId>,
    aria_label: SharedString,
    style_with_state: Option<ToastCloseStyle>,
}

impl<P: Clone + 'static> Default for ToastClose<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            toast_id: None,
            aria_label: SharedString::from("Close"),
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for ToastClose<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastPartNode<P> for ToastClose<P> {
    fn with_toast(mut self, context: ToastContext<P>, id: ToastId) -> Self {
        self.context = Some(context);
        self.toast_id = Some(id);
        self
    }
}

impl<P: Clone + 'static> RenderOnce for ToastClose<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(context), Some(toast_id)) = (self.context, self.toast_id) else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, _| runtime.close_state(&toast_id));
        let element_id = ElementId::Name(SharedString::from(format!(
            "toast-close-{}",
            toast_id.as_str()
        )));
        let expanded = state.expanded;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        let mut base = base.id(element_id);
        // AccessKit gap in this gpui revision: no `aria-hidden` builder, so
        // the collapsed close button drops its role instead (the node leaves
        // the a11y tree, approximating Base UI's collapsed `aria-hidden`).
        if expanded {
            base = base.role(Role::Button);
        }
        base.aria_label(self.aria_label)
            .focusable()
            .tab_stop(true)
            .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                cx.stop_propagation();
            })
            .on_click(move |_event, _window, cx| {
                context.close(Some(&toast_id), cx);
            })
            .children(self.children)
            .into_any_element()
    }
}

impl<P: Clone + 'static> ToastClose<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Accessible label for the close button; defaults to "Close". Callers
    /// rendering a visible text child that duplicates this label should pass
    /// it as `Text::new_inaccessible(...)` (or override the label) to avoid
    /// double-announcing.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToastCloseStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
