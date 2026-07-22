use std::rc::Rc;

use gpui::{
    div, AnyElement, App, ClickEvent, Div, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Text,
    Window,
};

use crate::field::{
    context::current_field_context, item_context::current_field_item_disabled, FieldContext,
    FieldLabelStyleState,
};

#[derive(IntoElement)]
pub struct FieldLabel {
    base: Div,
    children: Vec<AnyElement>,
    text: Option<SharedString>,
    context: Option<FieldContext>,
    style_with_state: Option<Rc<dyn Fn(FieldLabelStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldLabel {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            text: None,
            context: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for FieldLabel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for FieldLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.or_else(current_field_context);
        if let Some(context) = context.as_ref() {
            context.register_label(self.text.clone(), cx);
        }

        let item_disabled = current_field_item_disabled();
        let root_state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)))
            .unwrap_or_default();
        let state = FieldLabelStyleState::new(root_state, root_state.disabled || item_disabled);
        let focus_context = context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id("field-label")
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                if let Some(context) = focus_context.as_ref() {
                    context.focus_control(window, cx);
                }
            })
            // The label text is registered on the field runtime so the
            // control can expose it via `.aria_label(...)`; render the
            // visible copy without an a11y node to avoid the name being
            // announced twice.
            .children(self.text.map(Text::new_inaccessible))
            .children(self.children)
    }
}

impl FieldLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    /// Sets the visible label text and registers it on the field runtime so
    /// the registered control can expose it as its accessible name. Rendered
    /// with `Text::new_inaccessible` to avoid double-announcing once the
    /// control carries the `.aria_label`.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldLabelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
