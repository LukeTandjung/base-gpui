use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, px, Anchor, App, Bounds, Div, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, Point, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxAlign, ComboboxChangeReason, ComboboxChangeSource, ComboboxContext,
    ComboboxPositionerChild, ComboboxPositionerStyleState, ComboboxSide,
};

/// Anchored/deferred overlay positioning. Combobox-local equivalent of
/// `select_positioner.rs`, without the `align_item_with_trigger` mode and
/// without scroll arrows. The anchor is the input group when present, else
/// the input, with an explicit `.anchor(...)` override.
#[derive(IntoElement)]
pub struct ComboboxPositioner<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxPositionerChild<T>>,
    context: Option<ComboboxContext<T>>,
    anchor: Option<Bounds<Pixels>>,
    side: ComboboxSide,
    align: ComboboxAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    style_with_state: Option<Rc<dyn Fn(ComboboxPositionerStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxPositioner<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            anchor: None,
            side: ComboboxSide::Bottom,
            align: ComboboxAlign::Start,
            side_offset: px(0.0),
            align_offset: px(0.0),
            collision_padding: px(8.0),
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxPositioner<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxPositioner<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let available_size = window.viewport_size();
        context.record_available_size(available_size, cx);
        if let Some(anchor_override) = self.anchor {
            context.update(cx, |runtime| {
                runtime.set_anchor_override_bounds(Some(anchor_override));
            });
        }
        let state = context.read(cx, |runtime, _| {
            runtime.positioner_state(self.side, self.align)
        });
        if !state.open {
            return div();
        }

        let position = state
            .anchor_bounds
            .map(|bounds| resolved_position(self.side, self.align, bounds));
        let offset = resolved_offset(self.side, self.side_offset, self.align_offset);
        let anchor = resolved_anchor(self.side, self.align);
        let outside_context = context.clone();
        let measure_context = context.clone();
        let positioner_id = (context.root_id(), "combobox-positioner");
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(positioner_id)
        .on_mouse_down_out(move |_, window, cx| {
            outside_context.set_open(
                false,
                ComboboxChangeReason::OutsidePress,
                ComboboxChangeSource::Pointer,
                window,
                cx,
            );
        })
        .children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        let mut anchored = anchored()
            .anchor(anchor)
            .offset(offset)
            .snap_to_window_with_margin(self.collision_padding);
        if let Some(position) = position {
            anchored = anchored.position(position);
        }

        let measured = div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_context.update(cx, |runtime| {
                    runtime.set_popup_bounds(bounds) | runtime.set_available_size(available_size)
                });
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(base);

        div().child(deferred(anchored.child(measured)).priority(1))
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxPositioner<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_combobox_context(context.clone()))
            .collect();
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_positioner_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxPositioner<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxPositionerChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxPositionerChild::Any(child.into_any_element()));
        self
    }

    /// Explicit anchor override; defaults to input-group-else-input.
    pub fn anchor(mut self, anchor: Bounds<Pixels>) -> Self {
        self.anchor = Some(anchor);
        self
    }

    pub fn side(mut self, side: ComboboxSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: ComboboxAlign) -> Self {
        self.align = align;
        self
    }

    pub fn side_offset(mut self, side_offset: Pixels) -> Self {
        self.side_offset = side_offset;
        self
    }

    pub fn align_offset(mut self, align_offset: Pixels) -> Self {
        self.align_offset = align_offset;
        self
    }

    pub fn collision_padding(mut self, collision_padding: Pixels) -> Self {
        self.collision_padding = collision_padding;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn resolved_anchor(side: ComboboxSide, align: ComboboxAlign) -> Anchor {
    match (side, align) {
        (ComboboxSide::Bottom, ComboboxAlign::Start) => Anchor::TopLeft,
        (ComboboxSide::Bottom, ComboboxAlign::Center) => Anchor::TopCenter,
        (ComboboxSide::Bottom, ComboboxAlign::End) => Anchor::TopRight,
        (ComboboxSide::Top, ComboboxAlign::Start) => Anchor::BottomLeft,
        (ComboboxSide::Top, ComboboxAlign::Center) => Anchor::BottomCenter,
        (ComboboxSide::Top, ComboboxAlign::End) => Anchor::BottomRight,
        (ComboboxSide::Left, _) => Anchor::RightCenter,
        (ComboboxSide::Right, _) => Anchor::LeftCenter,
    }
}

fn resolved_position(
    side: ComboboxSide,
    align: ComboboxAlign,
    bounds: Bounds<Pixels>,
) -> Point<Pixels> {
    match (side, align) {
        (ComboboxSide::Bottom, ComboboxAlign::Start) => point(bounds.left(), bounds.bottom()),
        (ComboboxSide::Bottom, ComboboxAlign::Center) => bounds.bottom_center(),
        (ComboboxSide::Bottom, ComboboxAlign::End) => point(bounds.right(), bounds.bottom()),
        (ComboboxSide::Top, ComboboxAlign::Start) => bounds.origin,
        (ComboboxSide::Top, ComboboxAlign::Center) => bounds.top_center(),
        (ComboboxSide::Top, ComboboxAlign::End) => bounds.top_right(),
        (ComboboxSide::Left, _) => point(bounds.left(), bounds.center().y),
        (ComboboxSide::Right, _) => point(bounds.right(), bounds.center().y),
    }
}

fn resolved_offset(side: ComboboxSide, side_offset: Pixels, align_offset: Pixels) -> Point<Pixels> {
    match side {
        ComboboxSide::Bottom => point(align_offset, side_offset),
        ComboboxSide::Top => point(align_offset, -side_offset),
        ComboboxSide::Left => point(-side_offset, align_offset),
        ComboboxSide::Right => point(side_offset, align_offset),
    }
}
