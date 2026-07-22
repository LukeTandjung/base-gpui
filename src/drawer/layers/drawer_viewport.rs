use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::dialog::child_wiring::DialogChildWiring;
use crate::dialog::{DialogOpenChangeReason, DialogOpenChangeSource, DialogViewportStyleState};
use crate::drawer::{
    child_wiring::DrawerChildNode, drawer_now_ms, DrawerContext, DrawerSwipeReleaseOutcome,
    DrawerViewportChild, DrawerViewportStyleState,
};

type DrawerViewportStyle<P> = Rc<dyn Fn(DrawerViewportStyleState<P>, Div) -> Div + 'static>;

/// The drawer positioning container and swipe-to-dismiss gesture engine: mouse
/// events translate into runtime gesture commands; release outcomes route
/// through the dialog context (`Swipe` reason) or the drawer snap-point change.
#[derive(IntoElement)]
pub struct DrawerViewport<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<DrawerViewportChild<P>>,
    context: Option<DrawerContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<DrawerViewportStyle<P>>,
}

impl<P: Clone + 'static> Default for DrawerViewport<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("drawer-viewport"),
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DrawerViewport<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

fn release_swipe<P: Clone + 'static>(
    context: &DrawerContext<P>,
    window: &mut Window,
    cx: &mut App,
) {
    let outcome = context.update(cx, |runtime| runtime.release_swipe(drawer_now_ms()));
    match outcome {
        DrawerSwipeReleaseOutcome::Rest => {}
        DrawerSwipeReleaseOutcome::Snap { snap_point } => {
            context.set_snap_point(Some(snap_point), DialogOpenChangeReason::Swipe, window, cx);
        }
        DrawerSwipeReleaseOutcome::Dismiss {
            restore_snap_point, ..
        } => {
            // The close request is cancelable through `on_open_change`; a
            // canceled or externally rejected close restores the drag position
            // and the pending snap point. NOTE: with a controlled root the
            // dismissal is optimistic — an async external close falls back to a
            // non-animated close, as in Base UI.
            let closed = context.dialog().close(
                DialogOpenChangeReason::Swipe,
                DialogOpenChangeSource::Pointer,
                window,
                cx,
            );
            let still_open = context.dialog().read(cx, |runtime, _| runtime.open_value());
            if !closed || still_open {
                context.update(cx, |runtime| runtime.revert_dismiss(restore_snap_point));
            }
        }
    }
}

impl<P: Clone + 'static> RenderOnce for DrawerViewport<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let dialog_state = self
            .context
            .as_ref()
            .map(|context| {
                context
                    .dialog()
                    .read(cx, |runtime, _| runtime.viewport_state(self.keep_mounted))
            })
            .unwrap_or_else(|| {
                DialogViewportStyleState::new(false, self.keep_mounted, false, 0, None, None)
            });
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.viewport_state(dialog_state.clone())
                })
            })
            .unwrap_or_else(|| {
                DrawerViewportStyleState::from_dialog(
                    dialog_state,
                    false,
                    0,
                    false,
                    false,
                    0.0,
                    Default::default(),
                )
            });
        if !state.mounted {
            return div().into_any_element();
        }

        let open = state.open;
        let down_context = self.context.clone();
        let cancel_context = self.context.clone();
        let move_context = self.context.clone();
        let up_context = self.context.clone();
        let up_out_context = self.context.clone();
        let measure_context = self.context.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.on_children_prepainted(move |bounds, _window, cx| {
            if let (Some(context), Some(first)) = (measure_context.as_ref(), bounds.first()) {
                let mut union = *first;
                for child_bounds in bounds.iter().skip(1) {
                    union = union.union(child_bounds);
                }
                context.update(cx, |runtime| {
                    runtime
                        .set_popup_size(f32::from(union.size.width), f32::from(union.size.height))
                });
            }
        })
        .id(self.id)
        .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
            if !open {
                return;
            }
            if let Some(context) = down_context.as_ref() {
                context.update(cx, |runtime| {
                    runtime.begin_swipe(
                        f32::from(event.position.x),
                        f32::from(event.position.y),
                        drawer_now_ms(),
                        true,
                        false,
                    )
                });
            }
        })
        .on_mouse_down(MouseButton::Right, move |_event, _window, cx| {
            // A non-primary button appearing mid-drag cancels the swipe and
            // restores position.
            if let Some(context) = cancel_context.as_ref() {
                if context.read(cx, |runtime, _| runtime.swiping()) {
                    context.update(cx, |runtime| runtime.cancel_swipe());
                }
            }
        })
        .on_mouse_move(move |event, _window, cx| {
            let Some(context) = move_context.as_ref() else {
                return;
            };
            if !context.read(cx, |runtime, _| {
                runtime.swiping() && !runtime.open_gesture_active()
            }) {
                return;
            }
            if !event
                .pressed_button
                .map_or(false, |button| button == MouseButton::Left)
            {
                // Losing the primary button without a release event cancels.
                context.update(cx, |runtime| runtime.cancel_swipe());
                return;
            }
            context.update(cx, |runtime| {
                runtime.move_swipe(
                    f32::from(event.position.x),
                    f32::from(event.position.y),
                    drawer_now_ms(),
                )
            });
        })
        .on_mouse_up(MouseButton::Left, move |_event, window, cx| {
            let Some(context) = up_context.as_ref() else {
                return;
            };
            if context.read(cx, |runtime, _| {
                runtime.swiping() && !runtime.open_gesture_active()
            }) {
                release_swipe(context, window, cx);
            }
        })
        .on_mouse_up_out(MouseButton::Left, move |_event, window, cx| {
            let Some(context) = up_out_context.as_ref() else {
                return;
            };
            if context.read(cx, |runtime, _| {
                runtime.swiping() && !runtime.open_gesture_active()
            }) {
                release_swipe(context, window, cx);
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

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerViewport<P> {
    fn with_drawer_context(mut self, context: DrawerContext<P>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_drawer_context(context.clone()))
            .collect();
        self
    }

    fn wire_drawer_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_drawer_child(wiring, window, cx))
            .collect();
        self
    }
}

impl<P: Clone + 'static> DrawerViewport<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<DrawerViewportChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DrawerViewportChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerViewportStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
