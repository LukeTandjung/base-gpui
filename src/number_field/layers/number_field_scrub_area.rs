use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AnyElement, App, Div, ElementId, Entity, InteractiveElement as _, IntoElement,
    MouseButton, ParentElement, Point, RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::number_field::{
    NumberFieldContext, NumberFieldScrubAreaStyleState, NumberFieldScrubDirection,
};

#[derive(IntoElement)]
pub struct NumberFieldScrubArea {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NumberFieldContext>,
    direction: NumberFieldScrubDirection,
    pixel_sensitivity: f64,
    style_with_state: Option<Rc<dyn Fn(NumberFieldScrubAreaStyleState, Div) -> Div + 'static>>,
}

impl Default for NumberFieldScrubArea {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::new(),
            context: None,
            direction: NumberFieldScrubDirection::Horizontal,
            pixel_sensitivity: 2.0,
            style_with_state: None,
        }
    }
}

impl Styled for NumberFieldScrubArea {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for NumberFieldScrubArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("NumberFieldScrubArea must be rendered inside NumberFieldRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("scrub-area"));
        let style_state = context.read(cx, |runtime, props| {
            runtime.scrub_area_state(props, self.direction)
        });
        let last_position: Entity<Option<Point<gpui::Pixels>>> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("scrub-position")),
            cx,
            |_, _| None,
        );
        let down_context = context.clone();
        let move_context = context.clone();
        let up_context = context.clone();
        let down_position = last_position.clone();
        let move_position = last_position.clone();
        let up_position = last_position.clone();
        let direction = self.direction;
        let pixel_sensitivity = self.pixel_sensitivity;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        // No a11y role on purpose: Base UI renders the scrub area as `role="presentation"`.
        // Without a role this element stays out of the AccessKit tree, the gpui equivalent.
        base.id(id)
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                *down_position.as_mut(cx) = Some(event.position);
                down_context.focus_handle().focus(window, cx);
                down_context.set_scrubbing(true, window, cx);
            })
            .on_mouse_move(move |event, window, cx| {
                let previous = *move_position.read(cx);
                let Some(previous) = previous else {
                    return;
                };
                let delta = match direction {
                    NumberFieldScrubDirection::Horizontal => {
                        (event.position.x - previous.x).as_f32() as f64
                    }
                    NumberFieldScrubDirection::Vertical => {
                        (previous.y - event.position.y).as_f32() as f64
                    }
                };
                *move_position.as_mut(cx) = Some(event.position);
                move_context.scrub_by_pixels(delta, pixel_sensitivity, direction, window, cx);
            })
            .on_mouse_up(MouseButton::Left, move |_event, window, cx| {
                *up_position.as_mut(cx) = None;
                up_context.set_scrubbing(false, window, cx);
            })
            .children(self.children)
    }
}

impl NumberFieldScrubArea {
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

    pub fn direction(mut self, direction: NumberFieldScrubDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn pixel_sensitivity(mut self, pixel_sensitivity: f64) -> Self {
        self.pixel_sensitivity = pixel_sensitivity;
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
        style: impl Fn(NumberFieldScrubAreaStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
