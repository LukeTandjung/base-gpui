use std::rc::Rc;

use gpui::{
    div, px, AccessibleAction, App, Div, ElementId, InteractiveElement as _, IntoElement,
    MouseButton, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::toast::child_wiring::ToastPartNode;
use crate::toast::{
    ToastCloseAction, ToastContext, ToastId, ToastPriority, ToastRootChild, ToastRootStyleState,
    ToastSwipeDirection, TOAST_ROOT_KEY_CONTEXT,
};

type ToastRootStyle = Rc<dyn Fn(ToastRootStyleState, Div) -> Div + 'static>;

/// One toast: swipe-to-dismiss via GPUI mouse handlers (default directions
/// `[Down, Right]`; an empty set disables swiping), Escape-to-close via key
/// dispatch on the root's key context, and prepaint self-height measurement
/// feeding the runtime's stacking metadata. Limited toasts render inert
/// (no handlers, not focusable).
#[derive(IntoElement)]
pub struct ToastRoot<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<ToastRootChild<P>>,
    swipe_directions: Vec<ToastSwipeDirection>,
    context: Option<ToastContext<P>>,
    toast_id: Option<ToastId>,
    style_with_state: Option<ToastRootStyle>,
}

impl<P: Clone + 'static> Default for ToastRoot<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            swipe_directions: vec![ToastSwipeDirection::Down, ToastSwipeDirection::Right],
            context: None,
            toast_id: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for ToastRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> ToastPartNode<P> for ToastRoot<P> {
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

impl<P: Clone + 'static> RenderOnce for ToastRoot<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(context), Some(toast_id)) = (self.context, self.toast_id) else {
            return div().into_any_element();
        };
        let (state, title, description) = context.read(cx, |runtime, _| {
            (
                runtime.root_state(&toast_id),
                runtime.title_state(&toast_id).title,
                runtime.description_state(&toast_id).description,
            )
        });
        let element_id = ElementId::Name(SharedString::from(format!(
            "toast-root-{}",
            toast_id.as_str()
        )));

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state.clone(), self.base),
            None => self.base,
        };

        let measure_context = context.clone();
        let measure_id = toast_id.clone();
        let base = base
            .relative()
            .left(px(state.swipe_movement_x))
            .top(px(state.swipe_movement_y))
            .on_children_prepainted(move |bounds, _window, cx| {
                if let Some(first) = bounds.first() {
                    let mut union = *first;
                    for child_bounds in bounds.iter().skip(1) {
                        union = union.union(child_bounds);
                    }
                    measure_context.update(cx, |runtime| {
                        runtime.set_toast_height(&measure_id, f32::from(union.size.height))
                    });
                }
            })
            .id(element_id)
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );

        // AccessKit gap in this gpui revision: no `aria-hidden` builder, so
        // unfocused high-priority roots drop their role instead (the node
        // leaves the a11y tree, approximating Base UI's `aria-hidden`). No
        // `aria-labelledby`/`aria-describedby` builders either, so the root
        // carries a flattened literal label built from title + description.
        let role = match state.priority {
            ToastPriority::High if !state.expanded => None,
            ToastPriority::High => Some(Role::AlertDialog),
            ToastPriority::Low => Some(Role::Dialog),
        };
        let label = match (&title, &description) {
            (Some(title), Some(description)) => Some(format!("{title}. {description}")),
            (Some(title), None) => Some(title.to_string()),
            (None, Some(description)) => Some(description.to_string()),
            (None, None) => None,
        };
        let mut base = base;
        if let Some(role) = role {
            base = base.role(role);
        }
        if let Some(label) = label {
            base = base.aria_label(label);
        }

        // Limited toasts stay mounted but inert: no swipe, no keyboard close,
        // no a11y dismiss action (perceivable but non-actionable).
        if state.limited {
            return base.into_any_element();
        }

        let a11y_close_context = context.clone();
        let a11y_close_id = toast_id.clone();
        let base = base.on_a11y_action(AccessibleAction::Click, move |_data, _window, cx| {
            a11y_close_context.close(Some(&a11y_close_id), cx);
        });

        let swipe_directions = self.swipe_directions;
        let down_context = context.clone();
        let down_id = toast_id.clone();
        let move_context = context.clone();
        let up_context = context.clone();
        let up_out_context = context.clone();
        let escape_context = context.clone();
        let escape_id = toast_id.clone();

        fn release<P: Clone + 'static>(context: &ToastContext<P>, cx: &mut App) {
            let release = context.update(cx, |runtime| runtime.release_swipe());
            if let Some(release) = release {
                if release.dismiss {
                    context.close(Some(&release.id), cx);
                }
            }
        }

        base.key_context(TOAST_ROOT_KEY_CONTEXT)
            .on_action(move |_: &ToastCloseAction, _window, cx| {
                escape_context.close(Some(&escape_id), cx);
            })
            .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                // Interactive descendants (Close/Action) stop propagation on
                // mouse-down, so a press over them never begins a swipe.
                down_context.update(cx, |runtime| {
                    runtime.begin_swipe(
                        &down_id,
                        f32::from(event.position.x),
                        f32::from(event.position.y),
                        swipe_directions.clone(),
                    )
                });
            })
            .on_mouse_move(move |event, _window, cx| {
                if !move_context.read(cx, |runtime, _| runtime.swiping()) {
                    return;
                }
                if event.pressed_button != Some(MouseButton::Left) {
                    move_context.update(cx, |runtime| runtime.cancel_swipe());
                    return;
                }
                move_context.update(cx, |runtime| {
                    runtime.move_swipe(f32::from(event.position.x), f32::from(event.position.y))
                });
            })
            .on_mouse_up(MouseButton::Left, move |_event, _window, cx| {
                if up_context.read(cx, |runtime, _| runtime.swiping()) {
                    release(&up_context, cx);
                }
            })
            .on_mouse_up_out(MouseButton::Left, move |_event, _window, cx| {
                if up_out_context.read(cx, |runtime, _| runtime.swiping()) {
                    release(&up_out_context, cx);
                }
            })
            .into_any_element()
    }
}

impl<P: Clone + 'static> ToastRoot<P> {
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

    /// Permitted swipe-to-dismiss directions; default `[Down, Right]`, an
    /// empty set disables swiping.
    pub fn swipe_direction(
        mut self,
        directions: impl IntoIterator<Item = ToastSwipeDirection>,
    ) -> Self {
        self.swipe_directions = directions.into_iter().collect();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToastRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
