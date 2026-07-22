use std::rc::Rc;

use gpui::{
    div, App, Bounds, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, Pixels, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::slider::{SliderContext, SliderControlChild, SliderControlStyleState};
use crate::utils::current_direction;

#[derive(IntoElement)]
pub struct SliderControl {
    id: Option<ElementId>,
    base: Div,
    children: Vec<SliderControlChild>,
    context: Option<SliderContext>,
    style_with_state: Option<Rc<dyn Fn(SliderControlStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderControl {
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

impl Styled for SliderControl {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SliderControl {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("SliderControl must be rendered inside SliderRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("control"));
        let direction = current_direction();
        let style_state = context.read(cx, |runtime, props| runtime.control_state(props));

        let child_thumb_indices = self
            .children
            .iter()
            .map(|child| match child {
                SliderControlChild::Thumb(thumb) => thumb.thumb_index(),
                _ => None,
            })
            .collect::<Vec<_>>();

        let control_bounds_context = context.clone();
        let thumb_bounds_context = context.clone();
        let down_context = context.clone();
        let move_context = context.clone();
        let up_context = context.clone();
        let up_out_context = context.clone();

        let inner = div()
            .relative()
            .size_full()
            .on_children_prepainted(move |bounds: Vec<Bounds<Pixels>>, _window, cx| {
                let thumb_bounds = bounds
                    .into_iter()
                    .zip(child_thumb_indices.iter().copied())
                    .filter_map(|(bounds, index)| index.map(|index| (index, bounds)))
                    .collect::<Vec<_>>();
                if !thumb_bounds.is_empty() {
                    thumb_bounds_context.update(cx, |runtime| {
                        runtime.set_thumb_bounds(thumb_bounds);
                    });
                }
            })
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.relative()
            .on_children_prepainted(move |bounds: Vec<Bounds<Pixels>>, _window, cx| {
                if let Some(bounds) = bounds.first().copied() {
                    control_bounds_context.update(cx, |runtime| {
                        runtime.set_control_bounds(bounds);
                    });
                }
            })
            .id(id)
            .on_mouse_down(MouseButton::Left, move |event, window, cx| {
                down_context.press_track(event.position, direction, window, cx);
            })
            .on_mouse_move(move |event, window, cx| {
                if event.pressed_button != Some(MouseButton::Left) {
                    return;
                }
                move_context.drag_to(event.position, direction, window, cx);
            })
            .on_mouse_up(MouseButton::Left, move |_event, window, cx| {
                up_context.release(window, cx);
            })
            // GPUI cannot observe DOM-style `buttons === 0` on move events for
            // a press that was released outside the window; `on_mouse_up_out`
            // covers the release-outside case instead.
            .on_mouse_up_out(MouseButton::Left, move |_event, window, cx| {
                up_out_context.release(window, cx);
            })
            .child(inner)
    }
}

impl SliderControl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_slider_context(mut self, context: SliderContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn map_children(
        mut self,
        map: impl FnOnce(Vec<SliderControlChild>) -> Vec<SliderControlChild>,
    ) -> Self {
        self.children = map(self.children);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn child(mut self, child: impl Into<SliderControlChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<SliderControlChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SliderControlChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderControlStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
