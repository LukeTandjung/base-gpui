use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, MouseButton, ParentElement, RenderOnce, SharedString, StyleRefinement, Styled,
    Window,
};

use crate::dialog::child_wiring::DialogChildWiring;
use crate::dialog::{DialogOpenChangeReason, DialogOpenChangeSource};
use crate::drawer::{
    child_wiring::DrawerChildNode, drawer_now_ms, DrawerContext, DrawerSwipeAreaStyleState,
    DrawerSwipeDirection,
};

type DrawerSwipeAreaStyle = Rc<dyn Fn(DrawerSwipeAreaStyleState, Div) -> Div + 'static>;

/// An invisible edge area that opens a closed drawer by dragging. Registers as
/// a drawer trigger so `trigger_id` styling and focus return treat swipe-opens
/// like trigger opens.
#[derive(IntoElement)]
pub struct DrawerSwipeArea<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<DrawerContext<P>>,
    focus_handle: Option<FocusHandle>,
    disabled: bool,
    swipe_direction: Option<DrawerSwipeDirection>,
    style_with_state: Option<DrawerSwipeAreaStyle>,
}

impl<P: Clone + 'static> Default for DrawerSwipeArea<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("drawer-swipe-area"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            disabled: false,
            swipe_direction: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for DrawerSwipeArea<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for DrawerSwipeArea<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

fn release_open_swipe<P: Clone + 'static>(
    context: &DrawerContext<P>,
    window: &mut Window,
    cx: &mut App,
) {
    let release = context.update(cx, |runtime| runtime.release_open_swipe(drawer_now_ms()));
    if !release.keep_open && release.opened_by_gesture {
        context.dialog().close(
            DialogOpenChangeReason::Swipe,
            DialogOpenChangeSource::Pointer,
            window,
            cx,
        );
    }
}

impl<P: Clone + 'static> RenderOnce for DrawerSwipeArea<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let scoped_id = self.id.clone();
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| swipe_area_focus_handle(&scoped_id, window, cx));
        let open = self
            .context
            .as_ref()
            .map(|context| context.dialog().read(cx, |runtime, _| runtime.open_value()))
            .unwrap_or(false);
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.swipe_area_state(open, self.disabled, self.swipe_direction)
                })
            })
            .unwrap_or_else(|| {
                DrawerSwipeAreaStyleState::new(
                    open,
                    false,
                    self.swipe_direction.unwrap_or(DrawerSwipeDirection::Up),
                    self.disabled,
                )
            });

        let disabled = self.disabled;
        let direction_override = self.swipe_direction;
        let down_context = self.context.clone();
        let move_context = self.context.clone();
        let up_context = self.context.clone();
        let up_out_context = self.context.clone();
        let move_id = scoped_id.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id(scoped_id)
            .track_focus(&focus_handle)
            .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                if disabled {
                    return;
                }
                let Some(context) = down_context.as_ref() else {
                    return;
                };
                let already_open = context.dialog().read(cx, |runtime, _| runtime.open_value());
                if already_open {
                    return;
                }
                cx.stop_propagation();
                let direction = direction_override.unwrap_or_else(|| {
                    context
                        .read(cx, |runtime, _| runtime.swipe_direction())
                        .opposite()
                });
                context.update(cx, |runtime| {
                    runtime.begin_open_swipe(
                        f32::from(event.position.x),
                        f32::from(event.position.y),
                        drawer_now_ms(),
                        direction,
                    )
                });
            })
            .on_mouse_move(move |event, window, cx| {
                let Some(context) = move_context.as_ref() else {
                    return;
                };
                if disabled {
                    // Disabling a mid-gesture swipe area resets gesture state.
                    if context.read(cx, |runtime, _| runtime.open_gesture_active()) {
                        context.update(cx, |runtime| runtime.cancel_swipe());
                    }
                    return;
                }
                if !context.read(cx, |runtime, _| runtime.open_gesture_active()) {
                    return;
                }
                context.update(cx, |runtime| {
                    runtime.move_swipe(
                        f32::from(event.position.x),
                        f32::from(event.position.y),
                        drawer_now_ms(),
                    )
                });
                let request_open = context.update(cx, |runtime| runtime.take_open_request());
                if request_open {
                    context.dialog().open_trigger(
                        move_id.clone(),
                        DialogOpenChangeReason::Swipe,
                        DialogOpenChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .on_mouse_up(MouseButton::Left, move |_event, window, cx| {
                let Some(context) = up_context.as_ref() else {
                    return;
                };
                if context.read(cx, |runtime, _| runtime.open_gesture_active()) {
                    release_open_swipe(context, window, cx);
                }
            })
            .on_mouse_up_out(MouseButton::Left, move |_event, window, cx| {
                let Some(context) = up_out_context.as_ref() else {
                    return;
                };
                if context.read(cx, |runtime, _| runtime.open_gesture_active()) {
                    release_open_swipe(context, window, cx);
                }
            })
            .children(self.children)
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerSwipeArea<P> {
    fn with_drawer_context(mut self, context: DrawerContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_drawer_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let source_id = self.id.clone();
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = swipe_area_focus_handle(&scoped_id, window, cx);
        wiring.register_trigger(
            scoped_id.clone(),
            source_id,
            focus_handle.clone(),
            self.disabled,
            None,
        );
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<P: Clone + 'static> DrawerSwipeArea<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Overrides the open direction (default: opposite of the root
    /// `swipe_direction`).
    pub fn swipe_direction(mut self, swipe_direction: DrawerSwipeDirection) -> Self {
        self.swipe_direction = Some(swipe_direction);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerSwipeAreaStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn swipe_area_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
