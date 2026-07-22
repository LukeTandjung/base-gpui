use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::fieldset::{current_fieldset_context, FieldsetContext, FieldsetLegendStyleState};

#[derive(IntoElement)]
pub struct FieldsetLegend {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<FieldsetContext>,
    style_with_state: Option<Rc<dyn Fn(FieldsetLegendStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldsetLegend {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for FieldsetLegend {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for FieldsetLegend {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldsetLegend {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.or_else(current_fieldset_context);
        let state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.legend_state(props)))
            .unwrap_or_default();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl FieldsetLegend {
    /// Creates a legend for a Fieldset.
    ///
    /// AccessKit: like Base UI's plain-`div` legend, this element has no role
    /// and is deliberately kept out of the a11y tree. Expose the group's label
    /// via `FieldsetRoot::aria_label(...)` instead. When you do, render the
    /// legend's visible text with `Text::new_inaccessible(...)` (not
    /// `text!(...)`) so screen readers do not announce the label twice.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fieldset_context(mut self, context: FieldsetContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldsetLegendStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
