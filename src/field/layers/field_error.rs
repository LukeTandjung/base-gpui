use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, Empty, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::field::{
    context::current_field_context, FieldContext, FieldErrorMatch, FieldErrorStyleState,
    FieldValidityKey,
};

#[derive(IntoElement)]
pub struct FieldError {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<FieldContext>,
    matcher: FieldErrorMatch,
    style_with_state: Option<Rc<dyn Fn(FieldErrorStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldError {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            matcher: FieldErrorMatch::Default,
            style_with_state: None,
        }
    }
}

impl ParentElement for FieldError {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for FieldError {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldError {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.or_else(current_field_context);
        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    let root = runtime.root_state(props);
                    let validity = runtime.validity_data(props);
                    FieldErrorStyleState::new(
                        root,
                        runtime.error_present(props, self.matcher),
                        validity.errors,
                        validity.error,
                    )
                })
            })
            .unwrap_or_default();

        if !state.present {
            return Empty.into_any_element();
        }

        if let Some(context) = context.as_ref() {
            context.register_error(cx);
        }

        let mut children = self.children;
        if children.is_empty() && !state.error.is_empty() {
            children.push(state.error.clone().into_any_element());
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(children).into_any_element()
    }
}

impl FieldError {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn match_(mut self, key: FieldValidityKey) -> Self {
        self.matcher = FieldErrorMatch::Key(key);
        self
    }

    pub fn match_always(mut self, always: bool) -> Self {
        self.matcher = match always {
            true => FieldErrorMatch::Always,
            false => FieldErrorMatch::Default,
        };
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldErrorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
