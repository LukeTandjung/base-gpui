use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::dialog::{
    child_wiring::{DialogChildNode, DialogChildWiring},
    DialogContext, DialogDescriptionStyleState,
};

type DialogDescriptionStyle<P> = Rc<dyn Fn(DialogDescriptionStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct DialogDescription<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<DialogContext<P>>,
    style_with_state: Option<DialogDescriptionStyle<P>>,
}

impl<P: Clone + 'static> Default for DialogDescription<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("dialog-description"),
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for DialogDescription<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for DialogDescription<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogDescription<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.description_state()))
            .unwrap_or_else(|| DialogDescriptionStyleState::new(false, None));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id(self.id).children(self.children)
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogDescription<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_dialog_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self.id = wiring.scope_child_id(&self.id);
        wiring.register_description(self.id.clone());
        self
    }
}

impl<P: Clone + 'static> DialogDescription<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogDescriptionStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
