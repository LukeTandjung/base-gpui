use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window,
};

use crate::meter::{MeterContext, MeterStyleState, MeterTrackChild};

#[derive(IntoElement)]
pub struct MeterTrack {
    id: Option<ElementId>,
    base: Div,
    children: Vec<MeterTrackChild>,
    context: Option<MeterContext>,
    style_with_state: Option<Rc<dyn Fn(MeterStyleState, Div) -> Div + 'static>>,
}

impl Default for MeterTrack {
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

impl Styled for MeterTrack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for MeterTrack {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("MeterTrack must be rendered inside MeterRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("track"));

        let children: Vec<AnyElement> = self
            .children
            .into_iter()
            .map(|child| match child {
                MeterTrackChild::Indicator(indicator) => indicator
                    .with_meter_context(context.clone())
                    .into_any_element(),
                MeterTrackChild::Any(any) => any,
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

impl MeterTrack {
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

    pub fn child(mut self, child: impl Into<MeterTrackChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MeterTrackChild::Any(child.into_any_element()));
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
