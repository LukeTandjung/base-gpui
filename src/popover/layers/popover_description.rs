use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::{PopoverChildNode, PopoverChildWiring},
    PopoverContext, PopoverDescriptionStyleState,
};

#[derive(IntoElement)]
pub struct PopoverDescription<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PopoverContext<P>>,
    style_with_state: Option<Rc<dyn Fn(PopoverDescriptionStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverDescription<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("popover-description"),
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for PopoverDescription<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PopoverDescription<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverDescription<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.description_state()))
            .unwrap_or(PopoverDescriptionStyleState);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id(self.id).children(self.children)
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverDescription<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self.id = wiring.scope_child_id(&self.id);
        wiring.register_description(self.id.clone());
        self
    }
}

impl<P: Clone + 'static> PopoverDescription<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverDescriptionStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
