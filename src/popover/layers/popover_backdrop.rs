use std::rc::Rc;

use gpui::{
    div, App, ClickEvent, Div, InteractiveElement as _, IntoElement, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::PopoverChildNode, PopoverBackdropStyleState, PopoverContext,
    PopoverOpenChangeReason, PopoverOpenChangeSource,
};

#[derive(IntoElement)]
pub struct PopoverBackdrop<P: Clone + 'static = ()> {
    base: Div,
    context: Option<PopoverContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(PopoverBackdropStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverBackdrop<P> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PopoverBackdrop<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverBackdrop<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.backdrop_state(self.keep_mounted)))
            .unwrap_or_else(|| PopoverBackdropStyleState::new(false, self.keep_mounted, true));
        if !state.mounted {
            return div().into_any_element();
        }
        let context = self.context;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let base = base.id("popover-backdrop");
        if !state.interactive {
            return div().into_any_element();
        }

        base.on_click(move |event, window, cx| {
            if !matches!(event, ClickEvent::Mouse(_)) {
                return;
            }
            if let Some(context) = context.as_ref() {
                context.close(
                    PopoverOpenChangeReason::OutsidePress,
                    PopoverOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            }
        })
        .into_any_element()
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverBackdrop<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> PopoverBackdrop<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
