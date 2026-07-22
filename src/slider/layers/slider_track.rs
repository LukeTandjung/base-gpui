use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::slider::{SliderContext, SliderTrackChild, SliderTrackStyleState};

#[derive(IntoElement)]
pub struct SliderTrack {
    id: Option<ElementId>,
    base: Div,
    children: Vec<SliderTrackChild>,
    context: Option<SliderContext>,
    style_with_state: Option<Rc<dyn Fn(SliderTrackStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderTrack {
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

impl Styled for SliderTrack {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SliderTrack {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("SliderTrack must be rendered inside SliderControl");
        let id = self.id.unwrap_or_else(|| context.child_id("track"));
        let style_state = context.read(cx, |runtime, props| runtime.track_state(props));

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(id).relative().children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl SliderTrack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_slider_context(mut self, context: SliderContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn map_children(
        mut self,
        map: impl FnOnce(Vec<SliderTrackChild>) -> Vec<SliderTrackChild>,
    ) -> Self {
        self.children = map(self.children);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn child(mut self, child: impl Into<SliderTrackChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<SliderTrackChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SliderTrackChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderTrackStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
