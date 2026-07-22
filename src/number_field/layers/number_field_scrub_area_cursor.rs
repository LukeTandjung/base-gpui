use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::number_field::{NumberFieldContext, NumberFieldScrubAreaCursorStyleState};

#[derive(IntoElement)]
pub struct NumberFieldScrubAreaCursor {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NumberFieldContext>,
    style_with_state:
        Option<Rc<dyn Fn(NumberFieldScrubAreaCursorStyleState, Div) -> Div + 'static>>,
}

impl Default for NumberFieldScrubAreaCursor {
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

impl Styled for NumberFieldScrubAreaCursor {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for NumberFieldScrubAreaCursor {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("NumberFieldScrubAreaCursor must be rendered inside NumberFieldRoot");
        let id = self
            .id
            .unwrap_or_else(|| context.child_id("scrub-area-cursor"));
        let style_state = context.read(cx, |runtime, props| runtime.scrub_cursor_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // No a11y role on purpose: Base UI marks the scrub cursor `aria-hidden`. Without
        // a role this element stays out of the AccessKit tree, the gpui equivalent.
        base.id(id).children(self.children)
    }
}

impl NumberFieldScrubAreaCursor {
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

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NumberFieldScrubAreaCursorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
