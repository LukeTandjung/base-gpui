use std::rc::Rc;

use gpui::{
    deferred, div, point, px, AnyElement, App, Bounds, Display, Div, Element, ElementId,
    GlobalElementId, InspectorElementId, InteractiveElement as _, IntoElement, LayoutId,
    ParentElement, Pixels, Point, Position, RenderOnce, Size, Style, StyleRefinement, Styled,
    Window,
};

use crate::{
    tooltip::{
        child_wiring::{scoped_part_id, TooltipChildNode, TooltipChildWiring},
        TooltipAlign, TooltipContext, TooltipOpenChangeReason, TooltipOpenChangeSource,
        TooltipPositionerChild, TooltipPositionerStyleState, TooltipSide,
    },
    utils::direction::{current_direction, TextDirection},
};

#[derive(IntoElement)]
pub struct TooltipPositioner<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<TooltipPositionerChild<P>>,
    context: Option<TooltipContext<P>>,
    side: TooltipSide,
    align: TooltipAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(TooltipPositionerStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for TooltipPositioner<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: TooltipSide::Top,
            align: TooltipAlign::Center,
            side_offset: px(0.0),
            align_offset: px(0.0),
            collision_padding: px(4.0),
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for TooltipPositioner<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for TooltipPositioner<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };

        let available_size = window.viewport_size();
        context.update(cx, |runtime| runtime.set_available_size(available_size));
        let mut state = context.read(cx, |runtime, _props| {
            runtime.positioner_state(self.side, self.align, self.keep_mounted)
        });
        if !state.mounted {
            return div();
        }

        let preferred_side = resolve_logical_side(self.side, current_direction());
        state.side = context
            .read(cx, |runtime, _| runtime.effective_side())
            .unwrap_or(preferred_side);
        state.align = context
            .read(cx, |runtime, _| runtime.effective_align())
            .unwrap_or(self.align);

        let open = context.read(cx, |runtime, _| runtime.open_value());
        let outside_context = context.clone();
        let positioner_id = scoped_part_id(&context.root_id(), "tooltip-positioner");
        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state.clone(), self.base),
            None => self.base,
        }
        .id(positioner_id);
        if open {
            base = base.on_mouse_down_out(move |event, window, cx| {
                let trigger_press = outside_context.read(cx, |runtime, _| {
                    runtime.active_trigger_contains(event.position)
                });
                if trigger_press {
                    return;
                }

                outside_context.close(
                    TooltipOpenChangeReason::OutsidePress,
                    TooltipOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            });
        } else {
            base = base.opacity(0.0).invisible();
        }
        let base = base.children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        if !open {
            return div().child(base);
        }

        let Some(anchor_bounds) = state.anchor_bounds else {
            return div().child(base.opacity(0.0).invisible());
        };

        let positioner = TooltipOverlayPositioner::new(
            context,
            anchor_bounds,
            preferred_side,
            self.align,
            self.side_offset,
            self.align_offset,
            self.collision_padding,
        )
        .child(base);

        div().child(deferred(positioner).priority(1))
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipPositioner<P> {
    fn with_tooltip_context(mut self, context: TooltipContext<P>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_tooltip_context(context.clone()))
            .collect();
        self
    }

    fn wire_tooltip_child(
        mut self,
        wiring: &mut TooltipChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_positioner_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> TooltipPositioner<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<TooltipPositionerChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(TooltipPositionerChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: TooltipAlign) -> Self {
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
        if keep_mounted {
            self.children = self
                .children
                .into_iter()
                .map(TooltipPositionerChild::keep_mounted_from_portal)
                .collect();
        }
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TooltipPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

struct TooltipOverlayPositioner<P: Clone + 'static> {
    context: TooltipContext<P>,
    trigger_bounds: Bounds<Pixels>,
    preferred_side: TooltipSide,
    align: TooltipAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    children: Vec<AnyElement>,
}

struct TooltipOverlayPositionerState {
    child_layout_ids: Vec<LayoutId>,
}

impl<P: Clone + 'static> TooltipOverlayPositioner<P> {
    fn new(
        context: TooltipContext<P>,
        trigger_bounds: Bounds<Pixels>,
        preferred_side: TooltipSide,
        align: TooltipAlign,
        side_offset: Pixels,
        align_offset: Pixels,
        collision_padding: Pixels,
    ) -> Self {
        Self {
            context,
            trigger_bounds,
            preferred_side,
            align,
            side_offset,
            align_offset,
            collision_padding,
            children: Vec::new(),
        }
    }
}

impl<P: Clone + 'static> ParentElement for TooltipOverlayPositioner<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Element for TooltipOverlayPositioner<P> {
    type RequestLayoutState = TooltipOverlayPositionerState;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let child_layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.request_layout(window, cx))
            .collect::<Vec<_>>();

        let layout_id = window.request_layout(
            Style {
                position: Position::Absolute,
                display: Display::Flex,
                ..Style::default()
            },
            child_layout_ids.iter().copied(),
            cx,
        );

        (
            layout_id,
            TooltipOverlayPositionerState { child_layout_ids },
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if request_layout.child_layout_ids.is_empty() {
            return;
        }

        let mut child_min: Point<Pixels> = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();
        for child_layout_id in &request_layout.child_layout_ids {
            let child_bounds = window.layout_bounds(*child_layout_id);
            child_min = child_min.min(&child_bounds.origin);
            child_max = child_max.max(&child_bounds.bottom_right());
        }

        let tooltip_size: Size<Pixels> = (child_max - child_min).into();
        let viewport_size = window.viewport_size();
        let client_inset = window.client_inset().unwrap_or(px(0.0));
        let tooltip_position = tooltip_overlay_position(
            self.preferred_side,
            self.align,
            self.trigger_bounds,
            tooltip_size,
            viewport_size,
            self.collision_padding + client_inset,
            self.side_offset,
            self.align_offset,
        );

        let changed = self.context.update(cx, |runtime| {
            runtime.set_available_size(viewport_size)
                | runtime.set_effective_placement(tooltip_position.side, tooltip_position.align)
        });
        if changed {
            window.request_animation_frame();
        }

        let offset = tooltip_position.bounds.origin - bounds.origin;
        let offset = point(offset.x.round(), offset.y.round());
        window.with_element_offset(offset, |window| {
            for child in &mut self.children {
                child.prepaint(window, cx);
            }
        });
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut self.children {
            child.paint(window, cx);
        }
    }
}

impl<P: Clone + 'static> IntoElement for TooltipOverlayPositioner<P> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TooltipOverlayPosition {
    bounds: Bounds<Pixels>,
    side: TooltipSide,
    align: TooltipAlign,
}

#[allow(clippy::too_many_arguments)]
fn tooltip_overlay_position(
    preferred_side: TooltipSide,
    align: TooltipAlign,
    trigger_bounds: Bounds<Pixels>,
    tooltip_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
    side_offset: Pixels,
    align_offset: Pixels,
) -> TooltipOverlayPosition {
    let side = resolve_overlay_side(
        preferred_side,
        trigger_bounds,
        tooltip_size,
        viewport_size,
        margin,
        side_offset,
    );
    let bounds = overlay_bounds_for_side(
        side,
        align,
        trigger_bounds,
        tooltip_size,
        side_offset,
        align_offset,
    );

    TooltipOverlayPosition {
        bounds: clamp_tooltip_bounds(bounds, viewport_size, margin),
        side,
        align,
    }
}

fn resolve_overlay_side(
    preferred_side: TooltipSide,
    trigger_bounds: Bounds<Pixels>,
    tooltip_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
    side_offset: Pixels,
) -> TooltipSide {
    let right_limit = (viewport_size.width - margin).max(margin);
    let bottom_limit = (viewport_size.height - margin).max(margin);
    let available_above = (trigger_bounds.top() - margin - side_offset).max(px(0.0));
    let available_below = (bottom_limit - trigger_bounds.bottom() - side_offset).max(px(0.0));
    let available_left = (trigger_bounds.left() - margin - side_offset).max(px(0.0));
    let available_right = (right_limit - trigger_bounds.right() - side_offset).max(px(0.0));

    let top_fits = tooltip_size.height <= available_above;
    let bottom_fits = tooltip_size.height <= available_below;
    let left_fits = tooltip_size.width <= available_left;
    let right_fits = tooltip_size.width <= available_right;

    match preferred_side {
        TooltipSide::Top => choose_vertical_side(
            TooltipSide::Top,
            top_fits,
            bottom_fits,
            available_above,
            available_below,
        ),
        TooltipSide::Bottom => choose_vertical_side(
            TooltipSide::Bottom,
            top_fits,
            bottom_fits,
            available_above,
            available_below,
        ),
        TooltipSide::Left => choose_horizontal_side(
            TooltipSide::Left,
            left_fits,
            right_fits,
            top_fits,
            bottom_fits,
            available_left,
            available_right,
            available_above,
            available_below,
        ),
        TooltipSide::Right => choose_horizontal_side(
            TooltipSide::Right,
            left_fits,
            right_fits,
            top_fits,
            bottom_fits,
            available_left,
            available_right,
            available_above,
            available_below,
        ),
        TooltipSide::InlineStart | TooltipSide::InlineEnd => preferred_side,
    }
}

fn choose_vertical_side(
    preferred_side: TooltipSide,
    top_fits: bool,
    bottom_fits: bool,
    available_above: Pixels,
    available_below: Pixels,
) -> TooltipSide {
    match preferred_side {
        TooltipSide::Top if top_fits => TooltipSide::Top,
        TooltipSide::Top if bottom_fits => TooltipSide::Bottom,
        TooltipSide::Bottom if bottom_fits => TooltipSide::Bottom,
        TooltipSide::Bottom if top_fits => TooltipSide::Top,
        _ if available_below >= available_above => TooltipSide::Bottom,
        _ => TooltipSide::Top,
    }
}

#[allow(clippy::too_many_arguments)]
fn choose_horizontal_side(
    preferred_side: TooltipSide,
    left_fits: bool,
    right_fits: bool,
    top_fits: bool,
    bottom_fits: bool,
    available_left: Pixels,
    available_right: Pixels,
    available_above: Pixels,
    available_below: Pixels,
) -> TooltipSide {
    match preferred_side {
        TooltipSide::Left if left_fits => TooltipSide::Left,
        TooltipSide::Left if right_fits => TooltipSide::Right,
        TooltipSide::Right if right_fits => TooltipSide::Right,
        TooltipSide::Right if left_fits => TooltipSide::Left,
        _ if bottom_fits && available_below >= available_above => TooltipSide::Bottom,
        _ if top_fits => TooltipSide::Top,
        _ if available_right >= available_left => TooltipSide::Right,
        _ => TooltipSide::Left,
    }
}

fn overlay_bounds_for_side(
    side: TooltipSide,
    align: TooltipAlign,
    trigger_bounds: Bounds<Pixels>,
    tooltip_size: Size<Pixels>,
    side_offset: Pixels,
    align_offset: Pixels,
) -> Bounds<Pixels> {
    let origin = match side {
        TooltipSide::Top => point(
            aligned_start(
                align,
                trigger_bounds.left(),
                trigger_bounds.right(),
                tooltip_size.width,
                align_offset,
            ),
            trigger_bounds.top() - tooltip_size.height - side_offset,
        ),
        TooltipSide::Bottom | TooltipSide::InlineStart | TooltipSide::InlineEnd => point(
            aligned_start(
                align,
                trigger_bounds.left(),
                trigger_bounds.right(),
                tooltip_size.width,
                align_offset,
            ),
            trigger_bounds.bottom() + side_offset,
        ),
        TooltipSide::Left => point(
            trigger_bounds.left() - tooltip_size.width - side_offset,
            aligned_start(
                align,
                trigger_bounds.top(),
                trigger_bounds.bottom(),
                tooltip_size.height,
                align_offset,
            ),
        ),
        TooltipSide::Right => point(
            trigger_bounds.right() + side_offset,
            aligned_start(
                align,
                trigger_bounds.top(),
                trigger_bounds.bottom(),
                tooltip_size.height,
                align_offset,
            ),
        ),
    };

    Bounds::new(origin, tooltip_size)
}

fn aligned_start(
    align: TooltipAlign,
    trigger_start: Pixels,
    trigger_end: Pixels,
    tooltip_length: Pixels,
    align_offset: Pixels,
) -> Pixels {
    let start = match align {
        TooltipAlign::Start => trigger_start,
        TooltipAlign::Center => (trigger_start + trigger_end) * 0.5 - tooltip_length * 0.5,
        TooltipAlign::End => trigger_end - tooltip_length,
    };
    start + align_offset
}

fn clamp_tooltip_bounds(
    mut bounds: Bounds<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
) -> Bounds<Pixels> {
    let right_limit = (viewport_size.width - margin).max(margin);
    let bottom_limit = (viewport_size.height - margin).max(margin);

    if bounds.right() > right_limit {
        bounds.origin.x -= bounds.right() - right_limit;
    }
    if bounds.left() < margin {
        bounds.origin.x = margin;
    }

    if bounds.bottom() > bottom_limit {
        bounds.origin.y -= bounds.bottom() - bottom_limit;
    }
    if bounds.top() < margin {
        bounds.origin.y = margin;
    }

    bounds
}

fn resolve_logical_side(side: TooltipSide, direction: TextDirection) -> TooltipSide {
    match (side, direction) {
        (TooltipSide::InlineStart, TextDirection::Ltr) => TooltipSide::Left,
        (TooltipSide::InlineStart, TextDirection::Rtl) => TooltipSide::Right,
        (TooltipSide::InlineEnd, TextDirection::Ltr) => TooltipSide::Right,
        (TooltipSide::InlineEnd, TextDirection::Rtl) => TooltipSide::Left,
        _ => side,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_position_prefers_top_when_it_fits() {
        let position = tooltip_overlay_position(
            TooltipSide::Top,
            TooltipAlign::Center,
            Bounds::new(point(px(100.0), px(80.0)), gpui::size(px(80.0), px(24.0))),
            gpui::size(px(120.0), px(30.0)),
            gpui::size(px(320.0), px(240.0)),
            px(4.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.side, TooltipSide::Top);
        assert_eq!(position.bounds.bottom(), px(80.0));
        assert_eq!(position.bounds.origin.x, px(80.0));
    }

    #[test]
    fn overlay_position_flips_to_bottom_near_top_edge() {
        let trigger_bounds = Bounds::new(point(px(24.0), px(4.0)), gpui::size(px(120.0), px(32.0)));
        let position = tooltip_overlay_position(
            TooltipSide::Top,
            TooltipAlign::Center,
            trigger_bounds,
            gpui::size(px(140.0), px(48.0)),
            gpui::size(px(320.0), px(240.0)),
            px(4.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.side, TooltipSide::Bottom);
        assert_eq!(position.bounds.top(), trigger_bounds.bottom());
    }

    #[test]
    fn overlay_position_clamps_to_viewport_margin() {
        let position = tooltip_overlay_position(
            TooltipSide::Top,
            TooltipAlign::Center,
            Bounds::new(point(px(4.0), px(80.0)), gpui::size(px(24.0), px(24.0))),
            gpui::size(px(120.0), px(32.0)),
            gpui::size(px(320.0), px(240.0)),
            px(4.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.bounds.left(), px(4.0));
    }

    #[test]
    fn left_and_right_placements_follow_trigger_edges() {
        let bounds = Bounds::new(point(px(100.0), px(120.0)), gpui::size(px(40.0), px(30.0)));
        let left = overlay_bounds_for_side(
            TooltipSide::Left,
            TooltipAlign::Center,
            bounds,
            gpui::size(px(80.0), px(20.0)),
            px(0.0),
            px(0.0),
        );
        let right = overlay_bounds_for_side(
            TooltipSide::Right,
            TooltipAlign::Center,
            bounds,
            gpui::size(px(80.0), px(20.0)),
            px(0.0),
            px(0.0),
        );

        assert_eq!(left.right(), bounds.left());
        assert_eq!(right.left(), bounds.right());
    }
}
