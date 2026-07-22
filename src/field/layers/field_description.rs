use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::field::{
    context::current_field_context, item_context::current_field_item_disabled, FieldContext,
    FieldDescriptionStyleState,
};

#[derive(IntoElement)]
pub struct FieldDescription {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<FieldContext>,
    style_with_state: Option<Rc<dyn Fn(FieldDescriptionStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldDescription {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for FieldDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for FieldDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.or_else(current_field_context);
        if let Some(context) = context.as_ref() {
            context.register_description(cx);
        }

        let item_disabled = current_field_item_disabled();
        let root_state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)))
            .unwrap_or_default();
        let state =
            FieldDescriptionStyleState::new(root_state, root_state.disabled || item_disabled);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl FieldDescription {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldDescriptionStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
