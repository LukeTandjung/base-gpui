use std::rc::Rc;

use gpui::{
    anchored, deferred, div, point, px, Anchor, App, Bounds, Div, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, Point, RenderOnce, Size, StyleRefinement, Styled, Window,
};

use crate::{
    popover::{
        child_wiring::{scoped_part_id, PopoverChildNode, PopoverChildWiring},
        PopoverAlign, PopoverBoundsKind, PopoverContext, PopoverOpenChangeReason,
        PopoverOpenChangeSource, PopoverPositionerChild, PopoverPositionerStyleState, PopoverSide,
    },
    utils::{
        direction::{current_direction, TextDirection},
        modal_backdrop, OverlayDismissHandler,
    },
};

#[derive(IntoElement)]
pub struct PopoverPositioner<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<PopoverPositionerChild<P>>,
    context: Option<PopoverContext<P>>,
    side: PopoverSide,
    align: PopoverAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(PopoverPositionerStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverPositioner<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            side_offset: px(0.0),
            align_offset: px(0.0),
            collision_padding: px(8.0),
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PopoverPositioner<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverPositioner<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let available_size = window.viewport_size();
        context.update(cx, |runtime| runtime.set_available_size(available_size));
        let (mut state, modal) = context.read(cx, |runtime, props| {
            (
                runtime.positioner_state(self.side, self.align, self.keep_mounted),
                props.modal(),
            )
        });
        if !state.mounted {
            return div();
        }

        let resolved_preferred_side = resolve_logical_side(self.side, current_direction());
        let effective_side = resolve_collision_side(
            resolved_preferred_side,
            state.anchor_bounds,
            state.popup_bounds,
            state.available_size,
            self.collision_padding,
        );
        state.side = effective_side;
        let placement_changed = context.update(cx, |runtime| {
            runtime.set_effective_placement(effective_side, self.align)
        });
        if placement_changed {
            window.request_animation_frame();
        }
        let position = state
            .anchor_bounds
            .map(|bounds| resolved_position(effective_side, self.align, bounds));
        let anchor = resolved_anchor(effective_side, self.align);
        let offset = resolved_offset(effective_side, self.side_offset, self.align_offset);
        let outside_context = context.clone();
        let modal_context = context.clone();
        let dismiss_handler: OverlayDismissHandler = Rc::new(move |window, cx| {
            modal_context.close(
                PopoverOpenChangeReason::OutsidePress,
                PopoverOpenChangeSource::Pointer,
                window,
                cx,
            );
        });
        let measure_context = context.clone();
        let popup_bounds = state.popup_bounds;
        let positioner_id = scoped_part_id(&context.root_id(), "popover-positioner");
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(positioner_id)
        .on_mouse_down_out(move |event, window, cx| {
            let trigger_press = outside_context.read(cx, |runtime, _| {
                runtime.active_trigger_contains(event.position)
            });
            if trigger_press {
                outside_context.close(
                    PopoverOpenChangeReason::TriggerPress,
                    PopoverOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            } else {
                outside_context.close(
                    PopoverOpenChangeReason::OutsidePress,
                    PopoverOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            }
        })
        .children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        if !context.read(cx, |runtime, _| runtime.open_value()) {
            return div().child(base);
        }

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
                    runtime.set_bounds(PopoverBoundsKind::Popup, bounds)
                        | runtime.set_available_size(available_size)
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

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPositioner<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_popover_context(context.clone()))
            .collect();
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_positioner_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> PopoverPositioner<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<PopoverPositionerChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PopoverPositionerChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
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

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn keep_mounted_from_portal(mut self) -> Self {
        self.keep_mounted = true;
        self.children = self
            .children
            .into_iter()
            .map(PopoverPositionerChild::keep_mounted_from_portal)
            .collect();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn resolve_logical_side(side: PopoverSide, direction: TextDirection) -> PopoverSide {
    match (side, direction) {
        (PopoverSide::InlineStart, TextDirection::Ltr) => PopoverSide::Left,
        (PopoverSide::InlineStart, TextDirection::Rtl) => PopoverSide::Right,
        (PopoverSide::InlineEnd, TextDirection::Ltr) => PopoverSide::Right,
        (PopoverSide::InlineEnd, TextDirection::Rtl) => PopoverSide::Left,
        _ => side,
    }
}

fn resolve_collision_side(
    side: PopoverSide,
    trigger_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    collision_padding: Pixels,
) -> PopoverSide {
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
        PopoverSide::Bottom => {
            if popup_bounds.size.height <= bottom_space {
                PopoverSide::Bottom
            } else if top_space > bottom_space {
                PopoverSide::Top
            } else if popup_bounds.size.width > right_space && left_space > right_space {
                PopoverSide::Left
            } else if right_space > bottom_space {
                PopoverSide::Right
            } else {
                PopoverSide::Bottom
            }
        }
        PopoverSide::Top => {
            if popup_bounds.size.height <= top_space {
                PopoverSide::Top
            } else if bottom_space > top_space {
                PopoverSide::Bottom
            } else if popup_bounds.size.width > left_space && right_space > left_space {
                PopoverSide::Right
            } else if left_space > top_space {
                PopoverSide::Left
            } else {
                PopoverSide::Top
            }
        }
        PopoverSide::Right => {
            if popup_bounds.size.width <= right_space {
                PopoverSide::Right
            } else if left_space > right_space {
                PopoverSide::Left
            } else if popup_bounds.size.height > bottom_space && top_space > bottom_space {
                PopoverSide::Top
            } else if bottom_space > right_space {
                PopoverSide::Bottom
            } else {
                PopoverSide::Right
            }
        }
        PopoverSide::Left => {
            if popup_bounds.size.width <= left_space {
                PopoverSide::Left
            } else if right_space > left_space {
                PopoverSide::Right
            } else if popup_bounds.size.height > top_space && bottom_space > top_space {
                PopoverSide::Bottom
            } else if top_space > left_space {
                PopoverSide::Top
            } else {
                PopoverSide::Left
            }
        }
        PopoverSide::InlineStart | PopoverSide::InlineEnd => side,
    }
}

fn resolved_anchor(side: PopoverSide, align: PopoverAlign) -> Anchor {
    match (side, align) {
        (PopoverSide::Bottom, PopoverAlign::Start) => Anchor::TopLeft,
        (PopoverSide::Bottom, PopoverAlign::Center) => Anchor::TopCenter,
        (PopoverSide::Bottom, PopoverAlign::End) => Anchor::TopRight,
        (PopoverSide::Top, PopoverAlign::Start) => Anchor::BottomLeft,
        (PopoverSide::Top, PopoverAlign::Center) => Anchor::BottomCenter,
        (PopoverSide::Top, PopoverAlign::End) => Anchor::BottomRight,
        (PopoverSide::Left, _) => Anchor::RightCenter,
        (PopoverSide::Right, _) => Anchor::LeftCenter,
        (PopoverSide::InlineStart | PopoverSide::InlineEnd, _) => Anchor::TopCenter,
    }
}

fn resolved_position(
    side: PopoverSide,
    align: PopoverAlign,
    bounds: Bounds<Pixels>,
) -> Point<Pixels> {
    match (side, align) {
        (PopoverSide::Bottom, PopoverAlign::Start) => point(bounds.left(), bounds.bottom()),
        (PopoverSide::Bottom, PopoverAlign::Center) => bounds.bottom_center(),
        (PopoverSide::Bottom, PopoverAlign::End) => point(bounds.right(), bounds.bottom()),
        (PopoverSide::Top, PopoverAlign::Start) => bounds.origin,
        (PopoverSide::Top, PopoverAlign::Center) => bounds.top_center(),
        (PopoverSide::Top, PopoverAlign::End) => bounds.top_right(),
        (PopoverSide::Left, _) => point(bounds.left(), bounds.center().y),
        (PopoverSide::Right, _) => point(bounds.right(), bounds.center().y),
        (PopoverSide::InlineStart | PopoverSide::InlineEnd, _) => bounds.bottom_center(),
    }
}

fn resolved_offset(side: PopoverSide, side_offset: Pixels, align_offset: Pixels) -> Point<Pixels> {
    match side {
        PopoverSide::Bottom => point(align_offset, side_offset),
        PopoverSide::Top => point(align_offset, -side_offset),
        PopoverSide::Left => point(-side_offset, align_offset),
        PopoverSide::Right => point(side_offset, align_offset),
        PopoverSide::InlineStart | PopoverSide::InlineEnd => point(align_offset, side_offset),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collision_resolution_flips_to_opposite_side_with_more_space() {
        let side = resolve_collision_side(
            PopoverSide::Bottom,
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

        assert_eq!(side, PopoverSide::Top);
    }

    #[test]
    fn collision_resolution_keeps_side_when_preferred_space_fits() {
        let side = resolve_collision_side(
            PopoverSide::Bottom,
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

        assert_eq!(side, PopoverSide::Bottom);
    }
}
