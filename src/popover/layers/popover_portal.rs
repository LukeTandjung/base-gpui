use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::{PopoverChildNode, PopoverChildWiring},
    PopoverContext, PopoverPortalChild, PopoverPortalStyleState,
};

#[derive(IntoElement)]
pub struct PopoverPortal<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<PopoverPortalChild<P>>,
    context: Option<PopoverContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(PopoverPortalStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverPortal<P> {
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

impl<P: Clone + 'static> Styled for PopoverPortal<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverPortal<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.portal_state(self.keep_mounted)))
            .unwrap_or_else(|| PopoverPortalStyleState::new(false, self.keep_mounted));

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

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPortal<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.context = Some(context.clone());
        let keep_mounted = self.keep_mounted;
        self.children = self
            .children
            .into_iter()
            .map(|child| match keep_mounted {
                true => child.keep_mounted_from_portal(),
                false => child,
            })
            .map(|child| child.with_popover_context(context.clone()))
            .collect();
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_portal_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> PopoverPortal<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<PopoverPortalChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PopoverPortalChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverPortalStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
