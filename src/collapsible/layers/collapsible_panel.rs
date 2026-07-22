use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Div, Empty, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::collapsible::{
    child_wiring::CollapsibleChildNode, CollapsibleContext, CollapsiblePanelStyleState,
};

#[derive(IntoElement)]
pub struct CollapsiblePanel {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<CollapsibleContext>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(CollapsiblePanelStyleState, Div) -> Div + 'static>>,
}

impl Default for CollapsiblePanel {
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

impl ParentElement for CollapsiblePanel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for CollapsiblePanel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for CollapsiblePanel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            base,
            children,
            context,
            keep_mounted,
            style_with_state,
        } = self;

        let state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.panel_state(keep_mounted)))
            .unwrap_or_else(|| CollapsiblePanelStyleState::new(true, true));
        let should_render = state.present;
        let hidden = state.closed;

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        if should_render {
            // A kept-mounted closed panel is hidden with `invisible()`, but gpui
            // registers accessibility nodes during prepaint (before the hidden
            // visibility check in paint), so accessible child text would still
            // leak to assistive technology. Gate the children on the closed
            // state as well so closed content is not announced.
            base.when(!hidden, |this| this.children(children))
                .when(hidden, |this| this.invisible())
                .into_any_element()
        } else {
            Empty.into_any_element()
        }
    }
}

impl CollapsibleChildNode for CollapsiblePanel {
    fn with_collapsible_context(mut self, context: CollapsibleContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl CollapsiblePanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(CollapsiblePanelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
