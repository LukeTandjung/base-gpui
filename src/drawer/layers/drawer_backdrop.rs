use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, MouseButton, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::dialog::{DialogBackdropStyleState, DialogOpenChangeReason, DialogOpenChangeSource};
use crate::drawer::{child_wiring::DrawerChildNode, DrawerBackdropStyleState, DrawerContext};

/// The drawer overlay: styled by live swipe progress and suppressed when nested
/// unless `force_render(true)`. Outside-press dismissal is disabled during a
/// swipe-area gesture.
#[derive(IntoElement)]
pub struct DrawerBackdrop<P: Clone + 'static = ()> {
    base: Div,
    context: Option<DrawerContext<P>>,
    keep_mounted: bool,
    force_rendered: bool,
    style_with_state: Option<Rc<dyn Fn(DrawerBackdropStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for DrawerBackdrop<P> {
    fn default() -> Self {
        Self {
            base: div().occlude(),
            context: None,
            keep_mounted: false,
            force_rendered: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DrawerBackdrop<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DrawerBackdrop<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let dialog_state = self
            .context
            .as_ref()
            .map(|context| {
                context.dialog().read(cx, |runtime, _| {
                    runtime.backdrop_state(self.keep_mounted, self.force_rendered)
                })
            })
            .unwrap_or_else(|| {
                DialogBackdropStyleState::new(false, self.keep_mounted, false, self.force_rendered)
            });
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.backdrop_state(dialog_state, self.force_rendered)
                })
            })
            .unwrap_or_else(|| {
                DrawerBackdropStyleState::from_dialog(
                    dialog_state,
                    false,
                    self.force_rendered,
                    0.0,
                    None,
                    false,
                    false,
                )
            });
        if !state.mounted || !state.rendered {
            return div().into_any_element();
        }

        let context = self.context;
        let pointer_dismissal_enabled = context
            .as_ref()
            .map(|context| {
                context
                    .dialog()
                    .read(cx, |runtime, _| runtime.pointer_dismissal_enabled())
                    && !context.read(cx, |runtime, _| runtime.dismissal_suppressed())
            })
            .unwrap_or(true);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let base = base.id("drawer-backdrop");
        if !pointer_dismissal_enabled {
            return base.into_any_element();
        }

        base.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
            cx.stop_propagation();
            if let Some(context) = context.as_ref() {
                context.dialog().close(
                    DialogOpenChangeReason::OutsidePress,
                    DialogOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            }
        })
        .into_any_element()
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerBackdrop<P> {
    fn with_drawer_context(mut self, context: DrawerContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> DrawerBackdrop<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    /// Renders a nested drawer's backdrop anyway (default `false`).
    pub fn force_render(mut self, force_render: bool) -> Self {
        self.force_rendered = force_render;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
