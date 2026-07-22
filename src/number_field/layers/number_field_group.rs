use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::number_field::{
    child_wiring::wire_group_children, NumberFieldContext, NumberFieldGroupChild,
    NumberFieldGroupStyleState,
};

#[derive(IntoElement)]
pub struct NumberFieldGroup {
    id: Option<ElementId>,
    base: Div,
    children: Vec<NumberFieldGroupChild>,
    context: Option<NumberFieldContext>,
    style_with_state: Option<Rc<dyn Fn(NumberFieldGroupStyleState, Div) -> Div + 'static>>,
}

impl Default for NumberFieldGroup {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for NumberFieldGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for NumberFieldGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("NumberFieldGroup must be rendered inside NumberFieldRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("group"));
        let style_state = context.read(cx, |runtime, props| runtime.group_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // Matches Base UI's `role="group"` on the group part.
        base.id(id)
            .role(Role::Group)
            .children(wire_group_children(self.children, context))
    }
}

impl NumberFieldGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_number_field_context(mut self, context: NumberFieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn child(mut self, child: impl Into<NumberFieldGroupChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<NumberFieldGroupChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NumberFieldGroupChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NumberFieldGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
