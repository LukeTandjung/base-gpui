use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::progress::{ProgressContext, ProgressStyleState, ProgressTrackChild};

#[derive(IntoElement)]
pub struct ProgressTrack {
    id: Option<ElementId>,
    base: Div,
    children: Vec<ProgressTrackChild>,
    context: Option<ProgressContext>,
    style_with_state: Option<Rc<dyn Fn(ProgressStyleState, Div) -> Div + 'static>>,
}

impl Default for ProgressTrack {
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

impl Styled for ProgressTrack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ProgressTrack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("ProgressTrack must be rendered inside ProgressRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("track"));

        let children: Vec<AnyElement> = self
            .children
            .into_iter()
            .map(|child| match child {
                ProgressTrackChild::Indicator(indicator) => indicator
                    .with_progress_context(context.clone())
                    .into_any_element(),
                ProgressTrackChild::Any(any) => any,
            })
            .collect();

        let style_state = context.read(|runtime| runtime.state());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(id).children(children)
    }
}

impl ProgressTrack {
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

    pub fn child(mut self, child: impl Into<ProgressTrackChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ProgressTrackChild::Any(child.into_any_element()));
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
