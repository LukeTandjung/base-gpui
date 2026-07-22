use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, px, AnyElement, App, Div, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::dialog::{
    child_wiring::{DialogChildNode, DialogChildWiring},
    DialogContext, DialogPortalChild, DialogPortalStyleState,
};

#[derive(IntoElement)]
pub struct DialogPortal<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<DialogPortalChild<P>>,
    context: Option<DialogContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(DialogPortalStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for DialogPortal<P> {
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

impl<P: Clone + 'static> Styled for DialogPortal<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogPortal<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let viewport_size = window.viewport_size();
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.portal_state(self.keep_mounted)))
            .unwrap_or_else(|| DialogPortalStyleState::new(false, self.keep_mounted));

        if !state.mounted {
            return div();
        }

        let base = self
            .base
            .absolute()
            .top_0()
            .left_0()
            .w(viewport_size.width)
            .h(viewport_size.height);
        let mut foreground_children: Vec<AnyElement> = Vec::new();
        let mut backdrop_children: Vec<AnyElement> = Vec::new();
        for child in self.children {
            match child {
                DialogPortalChild::Backdrop(backdrop) => {
                    backdrop_children.push((*backdrop).into_any_element());
                }
                other => foreground_children.push(other.into_element()),
            }
        }
        backdrop_children.extend(foreground_children);

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        }
        .children(backdrop_children);

        div().child(deferred(anchored().position(point(px(0.0), px(0.0))).child(base)).priority(1))
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogPortal<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        self.context = Some(context.clone());
        let keep_mounted = self.keep_mounted;
        self.children = self
            .children
            .into_iter()
            .map(|child| match keep_mounted {
                true => child.keep_mounted_from_portal(),
                false => child,
            })
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
        self.children = wiring.wire_portal_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> DialogPortal<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<DialogPortalChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DialogPortalChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogPortalStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
