use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::progress::{ProgressContext, ProgressStyleState};

/// Plain styled text part; Base UI's `aria-labelledby` id plumbing is out of
/// scope for the GPUI port. Not in the a11y tree (no role), matching Base
/// UI's `role="presentation"`. Pass label text as
/// `Text::new_inaccessible(...)` (not `text!(...)`) and mirror the same
/// string via `ProgressRoot::label(...)` so screen readers announce it once,
/// through the root's `aria_label`.
#[derive(IntoElement)]
pub struct ProgressLabel {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ProgressContext>,
    style_with_state: Option<Rc<dyn Fn(ProgressStyleState, Div) -> Div + 'static>>,
}

impl Default for ProgressLabel {
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

impl Styled for ProgressLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressLabel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("ProgressLabel must be rendered inside ProgressRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("label"));
        let style_state = context.read(|runtime| runtime.state());

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(id).children(self.children)
    }
}

impl ProgressLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_progress_context(mut self, context: ProgressContext) -> Self {
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

    pub fn style_with_state(
        mut self,
        style: impl Fn(ProgressStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
