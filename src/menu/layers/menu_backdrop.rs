use std::rc::Rc;

use gpui::{
    div, App, ClickEvent, Div, InteractiveElement as _, IntoElement, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{MenuChildNode, MenuChildWiring},
    MenuBackdropStyleState, MenuContext, MenuOpenChangeReason, MenuOpenChangeSource,
};

#[derive(IntoElement)]
pub struct MenuBackdrop<P: Clone + 'static = ()> {
    base: Div,
    context: Option<MenuContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(MenuBackdropStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuBackdrop<P> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuBackdrop<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuBackdrop<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.backdrop_state(self.keep_mounted)))
            .unwrap_or_else(|| MenuBackdropStyleState::new(false, self.keep_mounted, true));
        if !state.mounted {
            return div().into_any_element();
        }

        let context = self.context;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        let base = base.id("menu-backdrop");

        // A hover-opened menu's backdrop never captures pointer events.
        if !state.interactive {
            return base.into_any_element();
        }

        base.on_click(move |event, window, cx| {
            if !matches!(event, ClickEvent::Mouse(_)) {
                return;
            }
            if let Some(context) = context.as_ref() {
                context.close(
                    MenuOpenChangeReason::OutsidePress,
                    MenuOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            }
        })
        .into_any_element()
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuBackdrop<P> {
    fn wire_menu_child(
        mut self,
        _wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuBackdrop<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
