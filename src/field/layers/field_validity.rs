use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::field::{context::current_field_context, FieldContext, FieldValidityStyleState};

#[derive(IntoElement)]
pub struct FieldValidity {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<FieldContext>,
    style_with_state: Option<Rc<dyn Fn(FieldValidityStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldValidity {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for FieldValidity {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for FieldValidity {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldValidity {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.or_else(current_field_context);
        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    FieldValidityStyleState::new(
                        runtime.root_state(props),
                        runtime.validity_data(props),
                    )
                })
            })
            .unwrap_or_default();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl FieldValidity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldValidityStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
