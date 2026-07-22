use std::rc::Rc;

use gpui::{
    deferred, div, point, px, AnyElement, App, Bounds, Display, Div, Element, ElementId,
    GlobalElementId, InspectorElementId, InteractiveElement as _, IntoElement, LayoutId,
    ParentElement, Pixels, Point, Position, RenderOnce, Size, Style, StyleRefinement, Styled,
    Window,
};

use crate::{
    preview_card::{
        child_wiring::{scoped_part_id, PreviewCardChildNode, PreviewCardChildWiring},
        PreviewCardAlign, PreviewCardContext, PreviewCardOpenChangeReason,
        PreviewCardOpenChangeSource, PreviewCardPositionerChild, PreviewCardPositionerStyleState,
        PreviewCardSide,
    },
    utils::direction::{current_direction, TextDirection},
};

/// Anchors the popup to the active trigger's bounds via a deferred overlay.
/// Inline-rect anchoring (Base UI's hovered-line anchor for wrapped links)
/// is descoped: DOM client-rect traversal has no GPUI equivalent, so the
/// whole trigger's bounds anchor the card.
#[derive(IntoElement)]
pub struct PreviewCardPositioner<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<PreviewCardPositionerChild<P>>,
    context: Option<PreviewCardContext<P>>,
    side: PreviewCardSide,
    align: PreviewCardAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    arrow_padding: Pixels,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(PreviewCardPositionerStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PreviewCardPositioner<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: PreviewCardSide::Bottom,
            align: PreviewCardAlign::Center,
            side_offset: px(0.0),
            align_offset: px(0.0),
            collision_padding: px(5.0),
            arrow_padding: px(5.0),
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PreviewCardPositioner<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardPositioner<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };

        let available_size = window.viewport_size();
        let arrow_padding = self.arrow_padding;
        context.update(cx, |runtime| {
            runtime.set_available_size(available_size);
            runtime.set_arrow_padding(arrow_padding);
        });
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
        let positioner_id = scoped_part_id(&context.root_id(), "preview-card-positioner");
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
                    PreviewCardOpenChangeReason::OutsidePress,
                    PreviewCardOpenChangeSource::Pointer,
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

        let positioner = PreviewCardOverlayPositioner::new(
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

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardPositioner<P> {
    fn with_preview_card_context(mut self, context: PreviewCardContext<P>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_preview_card_context(context.clone()))
            .collect();
        self
    }

    fn wire_preview_card_child(
        mut self,
        wiring: &mut PreviewCardChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_positioner_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> PreviewCardPositioner<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<PreviewCardPositionerChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PreviewCardPositionerChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: PreviewCardSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PreviewCardAlign) -> Self {
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

    pub fn arrow_padding(mut self, arrow_padding: Pixels) -> Self {
        self.arrow_padding = arrow_padding;
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        if keep_mounted {
            self.children = self
                .children
                .into_iter()
                .map(PreviewCardPositionerChild::keep_mounted_from_portal)
                .collect();
        }
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PreviewCardPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

struct PreviewCardOverlayPositioner<P: Clone + 'static> {
    context: PreviewCardContext<P>,
    trigger_bounds: Bounds<Pixels>,
    preferred_side: PreviewCardSide,
    align: PreviewCardAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    children: Vec<AnyElement>,
}

struct PreviewCardOverlayPositionerState {
    child_layout_ids: Vec<LayoutId>,
}

impl<P: Clone + 'static> PreviewCardOverlayPositioner<P> {
    fn new(
        context: PreviewCardContext<P>,
        trigger_bounds: Bounds<Pixels>,
        preferred_side: PreviewCardSide,
        align: PreviewCardAlign,
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

impl<P: Clone + 'static> ParentElement for PreviewCardOverlayPositioner<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Element for PreviewCardOverlayPositioner<P> {
    type RequestLayoutState = PreviewCardOverlayPositionerState;
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
            PreviewCardOverlayPositionerState { child_layout_ids },
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

        let popup_size: Size<Pixels> = (child_max - child_min).into();
        let viewport_size = window.viewport_size();
        let client_inset = window.client_inset().unwrap_or(px(0.0));
        let popup_position = preview_card_overlay_position(
            self.preferred_side,
            self.align,
            self.trigger_bounds,
            popup_size,
            viewport_size,
            self.collision_padding + client_inset,
            self.side_offset,
            self.align_offset,
        );

        let changed = self.context.update(cx, |runtime| {
            runtime.set_available_size(viewport_size)
                | runtime.set_effective_placement(popup_position.side, popup_position.align)
        });
        if changed {
            window.request_animation_frame();
        }

        let offset = popup_position.bounds.origin - bounds.origin;
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

impl<P: Clone + 'static> IntoElement for PreviewCardOverlayPositioner<P> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PreviewCardOverlayPosition {
    bounds: Bounds<Pixels>,
    side: PreviewCardSide,
    align: PreviewCardAlign,
}

#[allow(clippy::too_many_arguments)]
fn preview_card_overlay_position(
    preferred_side: PreviewCardSide,
    align: PreviewCardAlign,
    trigger_bounds: Bounds<Pixels>,
    popup_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
    side_offset: Pixels,
    align_offset: Pixels,
) -> PreviewCardOverlayPosition {
    let side = resolve_overlay_side(
        preferred_side,
        trigger_bounds,
        popup_size,
        viewport_size,
        margin,
        side_offset,
    );
    let bounds = overlay_bounds_for_side(
        side,
        align,
        trigger_bounds,
        popup_size,
        side_offset,
        align_offset,
    );

    PreviewCardOverlayPosition {
        bounds: clamp_popup_bounds(bounds, viewport_size, margin),
        side,
        align,
    }
}

fn resolve_overlay_side(
    preferred_side: PreviewCardSide,
    trigger_bounds: Bounds<Pixels>,
    popup_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
    side_offset: Pixels,
) -> PreviewCardSide {
    let right_limit = (viewport_size.width - margin).max(margin);
    let bottom_limit = (viewport_size.height - margin).max(margin);
    let available_above = (trigger_bounds.top() - margin - side_offset).max(px(0.0));
    let available_below = (bottom_limit - trigger_bounds.bottom() - side_offset).max(px(0.0));
    let available_left = (trigger_bounds.left() - margin - side_offset).max(px(0.0));
    let available_right = (right_limit - trigger_bounds.right() - side_offset).max(px(0.0));

    let top_fits = popup_size.height <= available_above;
    let bottom_fits = popup_size.height <= available_below;
    let left_fits = popup_size.width <= available_left;
    let right_fits = popup_size.width <= available_right;

    match preferred_side {
        PreviewCardSide::Top => choose_vertical_side(
            PreviewCardSide::Top,
            top_fits,
            bottom_fits,
            available_above,
            available_below,
        ),
        PreviewCardSide::Bottom => choose_vertical_side(
            PreviewCardSide::Bottom,
            top_fits,
            bottom_fits,
            available_above,
            available_below,
        ),
        PreviewCardSide::Left => choose_horizontal_side(
            PreviewCardSide::Left,
            left_fits,
            right_fits,
            top_fits,
            bottom_fits,
            available_left,
            available_right,
            available_above,
            available_below,
        ),
        PreviewCardSide::Right => choose_horizontal_side(
            PreviewCardSide::Right,
            left_fits,
            right_fits,
            top_fits,
            bottom_fits,
            available_left,
            available_right,
            available_above,
            available_below,
        ),
        PreviewCardSide::InlineStart | PreviewCardSide::InlineEnd => preferred_side,
    }
}

fn choose_vertical_side(
    preferred_side: PreviewCardSide,
    top_fits: bool,
    bottom_fits: bool,
    available_above: Pixels,
    available_below: Pixels,
) -> PreviewCardSide {
    match preferred_side {
        PreviewCardSide::Top if top_fits => PreviewCardSide::Top,
        PreviewCardSide::Top if bottom_fits => PreviewCardSide::Bottom,
        PreviewCardSide::Bottom if bottom_fits => PreviewCardSide::Bottom,
        PreviewCardSide::Bottom if top_fits => PreviewCardSide::Top,
        _ if available_below >= available_above => PreviewCardSide::Bottom,
        _ => PreviewCardSide::Top,
    }
}

#[allow(clippy::too_many_arguments)]
fn choose_horizontal_side(
    preferred_side: PreviewCardSide,
    left_fits: bool,
    right_fits: bool,
    top_fits: bool,
    bottom_fits: bool,
    available_left: Pixels,
    available_right: Pixels,
    available_above: Pixels,
    available_below: Pixels,
) -> PreviewCardSide {
    match preferred_side {
        PreviewCardSide::Left if left_fits => PreviewCardSide::Left,
        PreviewCardSide::Left if right_fits => PreviewCardSide::Right,
        PreviewCardSide::Right if right_fits => PreviewCardSide::Right,
        PreviewCardSide::Right if left_fits => PreviewCardSide::Left,
        _ if bottom_fits && available_below >= available_above => PreviewCardSide::Bottom,
        _ if top_fits => PreviewCardSide::Top,
        _ if available_right >= available_left => PreviewCardSide::Right,
        _ => PreviewCardSide::Left,
    }
}

fn overlay_bounds_for_side(
    side: PreviewCardSide,
    align: PreviewCardAlign,
    trigger_bounds: Bounds<Pixels>,
    popup_size: Size<Pixels>,
    side_offset: Pixels,
    align_offset: Pixels,
) -> Bounds<Pixels> {
    let origin = match side {
        PreviewCardSide::Top => point(
            aligned_start(
                align,
                trigger_bounds.left(),
                trigger_bounds.right(),
                popup_size.width,
                align_offset,
            ),
            trigger_bounds.top() - popup_size.height - side_offset,
        ),
        PreviewCardSide::Bottom | PreviewCardSide::InlineStart | PreviewCardSide::InlineEnd => {
            point(
                aligned_start(
                    align,
                    trigger_bounds.left(),
                    trigger_bounds.right(),
                    popup_size.width,
                    align_offset,
                ),
                trigger_bounds.bottom() + side_offset,
            )
        }
        PreviewCardSide::Left => point(
            trigger_bounds.left() - popup_size.width - side_offset,
            aligned_start(
                align,
                trigger_bounds.top(),
                trigger_bounds.bottom(),
                popup_size.height,
                align_offset,
            ),
        ),
        PreviewCardSide::Right => point(
            trigger_bounds.right() + side_offset,
            aligned_start(
                align,
                trigger_bounds.top(),
                trigger_bounds.bottom(),
                popup_size.height,
                align_offset,
            ),
        ),
    };

    Bounds::new(origin, popup_size)
}

fn aligned_start(
    align: PreviewCardAlign,
    trigger_start: Pixels,
    trigger_end: Pixels,
    popup_length: Pixels,
    align_offset: Pixels,
) -> Pixels {
    let start = match align {
        PreviewCardAlign::Start => trigger_start,
        PreviewCardAlign::Center => (trigger_start + trigger_end) * 0.5 - popup_length * 0.5,
        PreviewCardAlign::End => trigger_end - popup_length,
    };
    start + align_offset
}

fn clamp_popup_bounds(
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

fn resolve_logical_side(side: PreviewCardSide, direction: TextDirection) -> PreviewCardSide {
    match (side, direction) {
        (PreviewCardSide::InlineStart, TextDirection::Ltr) => PreviewCardSide::Left,
        (PreviewCardSide::InlineStart, TextDirection::Rtl) => PreviewCardSide::Right,
        (PreviewCardSide::InlineEnd, TextDirection::Ltr) => PreviewCardSide::Right,
        (PreviewCardSide::InlineEnd, TextDirection::Rtl) => PreviewCardSide::Left,
        _ => side,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_position_prefers_bottom_when_it_fits() {
        let position = preview_card_overlay_position(
            PreviewCardSide::Bottom,
            PreviewCardAlign::Center,
            Bounds::new(point(px(100.0), px(80.0)), gpui::size(px(80.0), px(24.0))),
            gpui::size(px(120.0), px(30.0)),
            gpui::size(px(320.0), px(240.0)),
            px(5.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.side, PreviewCardSide::Bottom);
        assert_eq!(position.bounds.top(), px(104.0));
        assert_eq!(position.bounds.origin.x, px(80.0));
    }

    #[test]
    fn overlay_position_flips_to_top_near_bottom_edge() {
        let trigger_bounds =
            Bounds::new(point(px(24.0), px(200.0)), gpui::size(px(120.0), px(32.0)));
        let position = preview_card_overlay_position(
            PreviewCardSide::Bottom,
            PreviewCardAlign::Center,
            trigger_bounds,
            gpui::size(px(140.0), px(48.0)),
            gpui::size(px(320.0), px(240.0)),
            px(5.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.side, PreviewCardSide::Top);
        assert_eq!(position.bounds.bottom(), trigger_bounds.top());
    }

    #[test]
    fn overlay_position_clamps_to_viewport_margin() {
        let position = preview_card_overlay_position(
            PreviewCardSide::Bottom,
            PreviewCardAlign::Center,
            Bounds::new(point(px(4.0), px(80.0)), gpui::size(px(24.0), px(24.0))),
            gpui::size(px(120.0), px(32.0)),
            gpui::size(px(320.0), px(240.0)),
            px(5.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.bounds.left(), px(5.0));
    }
}
