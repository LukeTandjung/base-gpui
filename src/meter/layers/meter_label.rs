use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Text, Window,
};

use crate::meter::{MeterContext, MeterStyleState};

/// Plain styled text part; Base UI's `aria-labelledby` id plumbing is out of
/// scope for the GPUI port.
#[derive(IntoElement)]
pub struct MeterLabel {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<MeterContext>,
    style_with_state: Option<Rc<dyn Fn(MeterStyleState, Div) -> Div + 'static>>,
}

impl Default for MeterLabel {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::from([]),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for MeterLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for MeterLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("MeterLabel must be rendered inside MeterRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("label"));
        let style_state = context.read(|runtime| runtime.state());

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(id).children(self.children)
    }
}

impl MeterLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_meter_context(mut self, context: MeterContext) -> Self {
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

    /// Visible label text, kept out of the a11y tree so it is not announced
    /// twice — the same string should be passed to `MeterRoot::aria_label`.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.children
            .push(Text::new_inaccessible(text.into()).into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MeterStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
