use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, px, Anchor, App, Bounds, Div, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, Point, RenderOnce, Size, StyleRefinement, Styled, Window,
};

use crate::{
    select::{
        child_wiring::{SelectChildNode, SelectChildWiring},
        SelectAlign, SelectContext, SelectOpenChangeReason, SelectOpenChangeSource,
        SelectPositionerChild, SelectPositionerStyleState, SelectSide,
    },
    utils::{
        direction::{current_direction, TextDirection},
        modal_backdrop, OverlayDismissHandler,
    },
};

#[derive(IntoElement)]
pub struct SelectPositioner<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<SelectPositionerChild<T>>,
    context: Option<SelectContext<T>>,
    side: SelectSide,
    align: SelectAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    align_item_with_trigger: bool,
    style_with_state: Option<Rc<dyn Fn(SelectPositionerStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectPositioner<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: SelectSide::Bottom,
            align: SelectAlign::Start,
            side_offset: px(0.0),
            align_offset: px(0.0),
            collision_padding: px(8.0),
            align_item_with_trigger: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectPositioner<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectPositioner<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let available_size = window.viewport_size();
        context.record_available_size(available_size, cx);
        let (mut state, modal) = context.read(cx, |runtime, props| {
            (
                runtime.positioner_state(self.side, self.align),
                props.modal(),
            )
        });
        if !state.open {
            return div();
        }

        let (selected_item_text_bounds, value_bounds) = context.read(cx, |runtime, _| {
            (runtime.selected_item_text_bounds(), runtime.value_bounds())
        });
        let align_item_enabled = self.align_item_with_trigger && !state.touch_open;
        let align_item_placement = align_item_enabled
            .then(|| {
                align_item_placement(
                    self.side,
                    state.anchor_bounds,
                    state.popup_bounds,
                    selected_item_text_bounds,
                    value_bounds,
                    state.available_size,
                    current_direction(),
                )
            })
            .flatten();
        state.align_item_with_trigger_active = align_item_placement.is_some();
        state.align_item_transform_origin_y_percent = align_item_placement
            .as_ref()
            .map(|placement| placement.transform_origin_y_percent);
        let collision_side = resolve_collision_side(
            self.side,
            state.anchor_bounds,
            state.popup_bounds,
            state.available_size,
            self.collision_padding,
        );
        let effective_side = if align_item_placement.is_some() {
            self.side
        } else {
            collision_side
        };
        state.side = effective_side;
        let position = align_item_placement
            .as_ref()
            .map(|placement| placement.position)
            .or_else(|| {
                state
                    .anchor_bounds
                    .map(|bounds| resolved_position(effective_side, self.align, bounds))
            });
        let offset = resolved_offset(effective_side, self.side_offset, self.align_offset);
        let anchor = align_item_placement
            .as_ref()
            .map(|placement| placement.anchor)
            .unwrap_or_else(|| resolved_anchor(effective_side, self.align));
        let outside_context = context.clone();
        let modal_context = context.clone();
        let dismiss_handler: OverlayDismissHandler = Rc::new(move |window, cx| {
            modal_context.set_open(
                false,
                SelectOpenChangeReason::OutsidePress,
                SelectOpenChangeSource::Pointer,
                window,
                cx,
            );
        });
        let measure_context = context.clone();
        let popup_bounds = state.popup_bounds;
        let positioner_id = (context.root_id(), "select-positioner");
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(positioner_id)
        .on_mouse_down_out(move |_, window, cx| {
            outside_context.set_open(
                false,
                SelectOpenChangeReason::OutsidePress,
                SelectOpenChangeSource::Pointer,
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

        let anchored = anchored.child(measured);

        match modal {
            true => div().child(
                deferred(
                    div()
                        .child(modal_backdrop(
                            available_size,
                            popup_bounds,
                            dismiss_handler,
                        ))
                        .child(anchored),
                )
                .priority(1),
            ),
            false => div().child(deferred(anchored).priority(1)),
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectPositioner<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_select_context(context.clone()))
            .collect();
        self
    }

    fn wire_select_child(
        mut self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_positioner_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectPositioner<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<SelectPositionerChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SelectPositionerChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: SelectSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: SelectAlign) -> Self {
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

    pub fn align_item_with_trigger(mut self, align_item_with_trigger: bool) -> Self {
        self.align_item_with_trigger = align_item_with_trigger;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn resolve_collision_side(
    side: SelectSide,
    trigger_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    collision_padding: Pixels,
) -> SelectSide {
    let Some(trigger_bounds) = trigger_bounds else {
        return side;
    };
    let Some(popup_bounds) = popup_bounds else {
        return side;
    };
    let Some(available_size) = available_size else {
        return side;
    };

    let top_space = trigger_bounds.top() - collision_padding;
    let bottom_space = available_size.height - collision_padding - trigger_bounds.bottom();
    let left_space = trigger_bounds.left() - collision_padding;
    let right_space = available_size.width - collision_padding - trigger_bounds.right();

    match side {
        SelectSide::Bottom => {
            if popup_bounds.size.height <= bottom_space {
                SelectSide::Bottom
            } else if top_space > bottom_space {
                SelectSide::Top
            } else if popup_bounds.size.width > right_space && left_space > right_space {
                SelectSide::Left
            } else if right_space > bottom_space {
                SelectSide::Right
            } else {
                SelectSide::Bottom
            }
        }
        SelectSide::Top => {
            if popup_bounds.size.height <= top_space {
                SelectSide::Top
            } else if bottom_space > top_space {
                SelectSide::Bottom
            } else if popup_bounds.size.width > left_space && right_space > left_space {
                SelectSide::Right
            } else if left_space > top_space {
                SelectSide::Left
            } else {
                SelectSide::Top
            }
        }
        SelectSide::Right => {
            if popup_bounds.size.width <= right_space {
                SelectSide::Right
            } else if left_space > right_space {
                SelectSide::Left
            } else if popup_bounds.size.height > bottom_space && top_space > bottom_space {
                SelectSide::Top
            } else if bottom_space > right_space {
                SelectSide::Bottom
            } else {
                SelectSide::Right
            }
        }
        SelectSide::Left => {
            if popup_bounds.size.width <= left_space {
                SelectSide::Left
            } else if right_space > left_space {
                SelectSide::Right
            } else if popup_bounds.size.height > top_space && bottom_space > top_space {
                SelectSide::Bottom
            } else if top_space > left_space {
                SelectSide::Top
            } else {
                SelectSide::Left
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AlignItemPlacement {
    position: Point<Pixels>,
    anchor: Anchor,
    transform_origin_y_percent: f32,
}

fn align_item_placement(
    side: SelectSide,
    trigger_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    item_text_bounds: Option<Bounds<Pixels>>,
    value_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    direction: TextDirection,
) -> Option<AlignItemPlacement> {
    if !matches!(side, SelectSide::Bottom | SelectSide::Top) {
        return None;
    }

    let trigger_bounds = trigger_bounds?;
    let popup_bounds = popup_bounds?;
    let item_text_bounds = item_text_bounds?;
    let available_size = available_size?;

    let trigger_collision_threshold = px(20.0);
    let has_vertical_space = match side {
        SelectSide::Bottom => {
            trigger_bounds.top() >= trigger_collision_threshold
                && trigger_bounds.bottom() <= available_size.height - trigger_collision_threshold
                && trigger_bounds.bottom() + popup_bounds.size.height <= available_size.height
        }
        SelectSide::Top => {
            trigger_bounds.top() >= popup_bounds.size.height
                && trigger_bounds.top() >= trigger_collision_threshold
                && trigger_bounds.bottom() <= available_size.height - trigger_collision_threshold
        }
        SelectSide::Left | SelectSide::Right => false,
    };
    if !has_vertical_space {
        return None;
    }

    let inline_target_bounds = value_bounds.unwrap_or(trigger_bounds);
    let text_inline_delta = if direction.is_rtl() {
        item_text_bounds.right() - popup_bounds.left()
    } else {
        item_text_bounds.left() - popup_bounds.left()
    };
    let inline_target_edge = if direction.is_rtl() {
        inline_target_bounds.right()
    } else {
        inline_target_bounds.left()
    };
    let aligned_y = value_bounds.map(|bounds| {
        let value_center_delta = bounds.center().y - trigger_bounds.top();
        let text_center_delta = item_text_bounds.center().y - popup_bounds.top();
        trigger_bounds.top() + value_center_delta - text_center_delta
    });
    let position = match side {
        SelectSide::Bottom => point(
            inline_target_edge - text_inline_delta,
            aligned_y.unwrap_or_else(|| trigger_bounds.bottom()),
        ),
        SelectSide::Top => point(
            inline_target_edge - text_inline_delta,
            aligned_y.unwrap_or_else(|| trigger_bounds.top()),
        ),
        SelectSide::Left | SelectSide::Right => return None,
    };
    let anchor = match side {
        SelectSide::Bottom => Anchor::TopLeft,
        SelectSide::Top => Anchor::BottomLeft,
        SelectSide::Left | SelectSide::Right => return None,
    };

    let transform_origin_y_percent = if popup_bounds.size.height > Pixels::ZERO {
        (((item_text_bounds.center().y - popup_bounds.top()) / popup_bounds.size.height) * 100.0)
            .clamp(0.0, 100.0)
    } else {
        50.0
    };

    Some(AlignItemPlacement {
        position,
        anchor,
        transform_origin_y_percent,
    })
}

fn resolved_anchor(side: SelectSide, align: SelectAlign) -> Anchor {
    match (side, align) {
        (SelectSide::Bottom, SelectAlign::Start) => Anchor::TopLeft,
        (SelectSide::Bottom, SelectAlign::Center) => Anchor::TopCenter,
        (SelectSide::Bottom, SelectAlign::End) => Anchor::TopRight,
        (SelectSide::Top, SelectAlign::Start) => Anchor::BottomLeft,
        (SelectSide::Top, SelectAlign::Center) => Anchor::BottomCenter,
        (SelectSide::Top, SelectAlign::End) => Anchor::BottomRight,
        (SelectSide::Left, _) => Anchor::RightCenter,
        (SelectSide::Right, _) => Anchor::LeftCenter,
    }
}

fn resolved_position(
    side: SelectSide,
    align: SelectAlign,
    bounds: gpui::Bounds<Pixels>,
) -> Point<Pixels> {
    match (side, align) {
        (SelectSide::Bottom, SelectAlign::Start) => point(bounds.left(), bounds.bottom()),
        (SelectSide::Bottom, SelectAlign::Center) => bounds.bottom_center(),
        (SelectSide::Bottom, SelectAlign::End) => point(bounds.right(), bounds.bottom()),
        (SelectSide::Top, SelectAlign::Start) => bounds.origin,
        (SelectSide::Top, SelectAlign::Center) => bounds.top_center(),
        (SelectSide::Top, SelectAlign::End) => bounds.top_right(),
        (SelectSide::Left, _) => point(bounds.left(), bounds.center().y),
        (SelectSide::Right, _) => point(bounds.right(), bounds.center().y),
    }
}

fn resolved_offset(side: SelectSide, side_offset: Pixels, align_offset: Pixels) -> Point<Pixels> {
    match side {
        SelectSide::Bottom => point(align_offset, side_offset),
        SelectSide::Top => point(align_offset, -side_offset),
        SelectSide::Left => point(-side_offset, align_offset),
        SelectSide::Right => point(side_offset, align_offset),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_resolution_flips_to_opposite_side_with_more_space() {
        let side = resolve_collision_side(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(320.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(100.0), px(350.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(gpui::size(px(400.0), px(400.0))),
            px(8.0),
        );

        assert_eq!(side, SelectSide::Top);
    }

    #[test]
    fn collision_resolution_uses_fallback_axis_when_vertical_sides_do_not_fit() {
        let side = resolve_collision_side(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(20.0), px(100.0)),
                gpui::size(px(30.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(20.0), px(130.0)),
                gpui::size(px(160.0), px(200.0)),
            )),
            Some(gpui::size(px(400.0), px(250.0))),
            px(8.0),
        );

        assert_eq!(side, SelectSide::Right);
    }

    #[test]
    fn collision_resolution_keeps_side_when_preferred_space_fits() {
        let side = resolve_collision_side(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(40.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(100.0), px(70.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(gpui::size(px(400.0), px(400.0))),
            px(8.0),
        );

        assert_eq!(side, SelectSide::Bottom);
    }

    #[test]
    fn align_item_placement_aligns_inline_start_in_ltr() {
        let placement = align_item_placement(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(20.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(40.0), px(60.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(Bounds::new(
                point(px(64.0), px(92.0)),
                gpui::size(px(40.0), px(20.0)),
            )),
            None,
            Some(gpui::size(px(400.0), px(400.0))),
            TextDirection::Ltr,
        )
        .expect("align-item placement should be available");

        assert_eq!(placement.anchor, Anchor::TopLeft);
        assert_eq!(placement.position, point(px(76.0), px(50.0)));
        assert_eq!(placement.transform_origin_y_percent, 35.0);
    }

    #[test]
    fn align_item_placement_aligns_to_value_bounds_when_measured() {
        let placement = align_item_placement(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(20.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(40.0), px(60.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(Bounds::new(
                point(px(64.0), px(92.0)),
                gpui::size(px(40.0), px(20.0)),
            )),
            Some(Bounds::new(
                point(px(112.0), px(26.0)),
                gpui::size(px(40.0), px(16.0)),
            )),
            Some(gpui::size(px(400.0), px(400.0))),
            TextDirection::Ltr,
        )
        .expect("align-item placement should be available");

        assert_eq!(placement.position, point(px(88.0), px(-8.0)));
        assert_eq!(placement.transform_origin_y_percent, 35.0);
    }

    #[test]
    fn align_item_placement_aligns_inline_start_in_rtl() {
        let placement = align_item_placement(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(20.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(40.0), px(60.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(Bounds::new(
                point(px(64.0), px(92.0)),
                gpui::size(px(40.0), px(20.0)),
            )),
            None,
            Some(gpui::size(px(400.0), px(400.0))),
            TextDirection::Rtl,
        )
        .expect("align-item placement should be available");

        assert_eq!(placement.anchor, Anchor::TopLeft);
        assert_eq!(placement.position, point(px(116.0), px(50.0)));
    }

    #[test]
    fn align_item_placement_falls_back_near_viewport_edges() {
        let placement = align_item_placement(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(4.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(40.0), px(60.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(Bounds::new(
                point(px(64.0), px(92.0)),
                gpui::size(px(40.0), px(20.0)),
            )),
            None,
            Some(gpui::size(px(400.0), px(400.0))),
            TextDirection::Ltr,
        );

        assert!(placement.is_none());
    }

    #[test]
    fn align_item_placement_falls_back_without_vertical_space() {
        let placement = align_item_placement(
            SelectSide::Bottom,
            Some(Bounds::new(
                point(px(100.0), px(320.0)),
                gpui::size(px(80.0), px(30.0)),
            )),
            Some(Bounds::new(
                point(px(40.0), px(60.0)),
                gpui::size(px(160.0), px(120.0)),
            )),
            Some(Bounds::new(
                point(px(64.0), px(92.0)),
                gpui::size(px(40.0), px(20.0)),
            )),
            None,
            Some(gpui::size(px(400.0), px(400.0))),
            TextDirection::Ltr,
        );

        assert!(placement.is_none());
    }
}
