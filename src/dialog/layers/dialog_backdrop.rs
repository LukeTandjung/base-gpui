use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, MouseButton, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::dialog::{
    child_wiring::DialogChildNode, DialogBackdropStyleState, DialogContext, DialogOpenChangeReason,
    DialogOpenChangeSource,
};

#[derive(IntoElement)]
pub struct DialogBackdrop<P: Clone + 'static = ()> {
    base: Div,
    context: Option<DialogContext<P>>,
    keep_mounted: bool,
    force_rendered: bool,
    style_with_state: Option<Rc<dyn Fn(DialogBackdropStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for DialogBackdrop<P> {
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

impl<P: Clone + 'static> Styled for DialogBackdrop<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogBackdrop<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.backdrop_state(self.keep_mounted, self.force_rendered)
                })
            })
            .unwrap_or_else(|| {
                DialogBackdropStyleState::new(false, self.keep_mounted, false, self.force_rendered)
            });
        if !state.mounted {
            return div().into_any_element();
        }

        let context = self.context;
        let pointer_dismissal_enabled = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.pointer_dismissal_enabled()))
            .unwrap_or(true);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        if !state.rendered {
            return div().into_any_element();
        }
        let base = base.id("dialog-backdrop");
        if !pointer_dismissal_enabled {
            return base.into_any_element();
        }

        base.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
            cx.stop_propagation();
            if let Some(context) = context.as_ref() {
                context.close(
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

impl<P: Clone + 'static> DialogChildNode<P> for DialogBackdrop<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> DialogBackdrop<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn force_render(mut self, force_render: bool) -> Self {
        self.force_rendered = force_render;
        self
    }

    pub fn force_rendered(self, force_rendered: bool) -> Self {
        self.force_render(force_rendered)
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
