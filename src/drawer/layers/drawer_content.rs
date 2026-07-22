use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::drawer::{child_wiring::DrawerChildNode, DrawerContentStyleState, DrawerContext};

/// The swipe-exclusion marker: a pointer-down inside a `DrawerContent` subtree
/// never starts a swipe (typed replacement for Base UI's
/// `DRAWER_CONTENT_ATTRIBUTE` selector matching — the marker stops the
/// mouse-down from reaching the viewport gesture engine).
#[derive(IntoElement)]
pub struct DrawerContent<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<DrawerContext<P>>,
    style_with_state: Option<Rc<dyn Fn(DrawerContentStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for DrawerContent<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for DrawerContent<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for DrawerContent<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DrawerContent<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                let open = context.dialog().read(cx, |runtime, _| runtime.open_value());
                context.read(cx, |runtime, _| runtime.content_state(open))
            })
            .unwrap_or_default();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.on_mouse_down(MouseButton::Left, |_event, _window, cx| {
            // Block swipe-start over interactive drawer content.
            cx.stop_propagation();
        })
        .children(self.children)
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerContent<P> {
    fn with_drawer_context(mut self, context: DrawerContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> DrawerContent<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DrawerContentStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
