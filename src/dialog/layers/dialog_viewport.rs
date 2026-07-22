use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::dialog::{
    child_wiring::{DialogChildNode, DialogChildWiring},
    DialogContext, DialogViewportChild, DialogViewportStyleState,
};

type DialogViewportStyle<P> = Rc<dyn Fn(DialogViewportStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct DialogViewport<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<DialogViewportChild<P>>,
    context: Option<DialogContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<DialogViewportStyle<P>>,
}

impl<P: Clone + 'static> Default for DialogViewport<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DialogViewport<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogViewport<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.viewport_state(self.keep_mounted)))
            .unwrap_or_else(|| {
                DialogViewportStyleState::new(false, self.keep_mounted, false, 0, None, None)
            });
        if !state.mounted {
            return div();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogViewport<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_dialog_context(context.clone()))
            .collect();
        self
    }

    fn wire_dialog_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_viewport_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> DialogViewport<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<DialogViewportChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DialogViewportChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogViewportStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
