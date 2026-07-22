use std::rc::Rc;

use gpui::{
    deferred, div, point, px, AnyElement, App, Bounds, Display, Div, Element, ElementId,
    GlobalElementId, InspectorElementId, InteractiveElement as _, IntoElement, LayoutId,
    ParentElement, Pixels, Point, Position, RenderOnce, Size, Style, StyleRefinement, Styled,
    Window,
};

use crate::{
    navigation_menu::{
        child_wiring::{scoped_part_id, NavigationMenuChildNode, NavigationMenuChildWiring},
        NavigationMenuAlign, NavigationMenuContext, NavigationMenuPositionerChild,
        NavigationMenuPositionerStyleState, NavigationMenuSide, NavigationMenuValueChangeReason,
        NavigationMenuValueChangeSource,
    },
    utils::direction::{current_direction, TextDirection},
};

type NavigationMenuPositionerStyle =
    Rc<dyn Fn(NavigationMenuPositionerStyleState, Div) -> Div + 'static>;

/// Anchors the shared popup to the **active** trigger's measured bounds via
/// a deferred overlay, retargeting (without unmount) when the active value
/// changes, with a last-known-anchor fallback for the close transition after
/// the active trigger's item unmounts.
#[derive(IntoElement)]
pub struct NavigationMenuPositioner<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<NavigationMenuPositionerChild<T>>,
    context: Option<NavigationMenuContext<T>>,
    side: NavigationMenuSide,
    align: NavigationMenuAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    arrow_padding: Pixels,
    keep_mounted: bool,
    style_with_state: Option<NavigationMenuPositionerStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuPositioner<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: NavigationMenuSide::Bottom,
            align: NavigationMenuAlign::Center,
            side_offset: px(0.0),
            align_offset: px(0.0),
            collision_padding: px(5.0),
            arrow_padding: px(5.0),
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuPositioner<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuPositioner<T> {
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
        let positioner_id = scoped_part_id(&context.root_id(), "navigation-menu-positioner");
        let anchor_bounds = state.anchor_bounds;
        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(positioner_id);
        if open {
            base = base.on_mouse_down_out(move |event, window, cx| {
                let dismiss = outside_context.read(cx, |runtime, _| {
                    runtime.outside_press_should_dismiss(event.position)
                });
                if !dismiss {
                    return;
                }
                outside_context.close(
                    NavigationMenuValueChangeReason::OutsidePress,
                    NavigationMenuValueChangeSource::Pointer,
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

        let Some(anchor_bounds) = anchor_bounds else {
            return div().child(base.opacity(0.0).invisible());
        };

        let positioner = NavigationMenuOverlayPositioner::new(
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

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuPositioner<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuPositionerChild::Popup(popup) => {
                    NavigationMenuPositionerChild::Popup(Box::new(
                        popup.with_navigation_menu_context(context.clone()),
                    ))
                }
                NavigationMenuPositionerChild::Arrow(arrow) => {
                    NavigationMenuPositionerChild::Arrow(Box::new(
                        arrow.with_navigation_menu_context(context.clone()),
                    ))
                }
                NavigationMenuPositionerChild::Any(any) => NavigationMenuPositionerChild::Any(any),
            })
            .collect();
        self
    }

    fn wire_navigation_menu_child(
        mut self,
        wiring: &mut NavigationMenuChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuPositionerChild::Popup(popup) => {
                    NavigationMenuPositionerChild::Popup(Box::new(
                        popup.wire_navigation_menu_child(wiring, window, cx),
                    ))
                }
                other => other,
            })
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuPositioner<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<NavigationMenuPositionerChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuPositionerChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: NavigationMenuSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: NavigationMenuAlign) -> Self {
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
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

struct NavigationMenuOverlayPositioner<T: Clone + Eq + 'static> {
    context: NavigationMenuContext<T>,
    anchor_bounds: Bounds<Pixels>,
    preferred_side: NavigationMenuSide,
    align: NavigationMenuAlign,
    side_offset: Pixels,
    align_offset: Pixels,
    collision_padding: Pixels,
    children: Vec<AnyElement>,
}

struct NavigationMenuOverlayPositionerState {
    child_layout_ids: Vec<LayoutId>,
}

impl<T: Clone + Eq + 'static> NavigationMenuOverlayPositioner<T> {
    fn new(
        context: NavigationMenuContext<T>,
        anchor_bounds: Bounds<Pixels>,
        preferred_side: NavigationMenuSide,
        align: NavigationMenuAlign,
        side_offset: Pixels,
        align_offset: Pixels,
        collision_padding: Pixels,
    ) -> Self {
        Self {
            context,
            anchor_bounds,
            preferred_side,
            align,
            side_offset,
            align_offset,
            collision_padding,
            children: Vec::new(),
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for NavigationMenuOverlayPositioner<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Element for NavigationMenuOverlayPositioner<T> {
    type RequestLayoutState = NavigationMenuOverlayPositionerState;
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
            NavigationMenuOverlayPositionerState { child_layout_ids },
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
        let popup_position = navigation_menu_overlay_position(
            self.preferred_side,
            self.align,
            self.anchor_bounds,
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

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuOverlayPositioner<T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct NavigationMenuOverlayPosition {
    bounds: Bounds<Pixels>,
    side: NavigationMenuSide,
    align: NavigationMenuAlign,
}

/// Practical flip/shift subset: flip to the opposite side when the preferred
/// side does not fit, then clamp into the viewport. For side `Top` (and
/// physical-left placements) the anchored edge stays fixed so size morphs
/// grow away from the anchor.
#[allow(clippy::too_many_arguments)]
fn navigation_menu_overlay_position(
    preferred_side: NavigationMenuSide,
    align: NavigationMenuAlign,
    anchor_bounds: Bounds<Pixels>,
    popup_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
    side_offset: Pixels,
    align_offset: Pixels,
) -> NavigationMenuOverlayPosition {
    let side = resolve_overlay_side(
        preferred_side,
        anchor_bounds,
        popup_size,
        viewport_size,
        margin,
        side_offset,
    );
    let bounds = overlay_bounds_for_side(
        side,
        align,
        anchor_bounds,
        popup_size,
        side_offset,
        align_offset,
    );

    NavigationMenuOverlayPosition {
        bounds: clamp_popup_bounds(bounds, viewport_size, margin),
        side,
        align,
    }
}

fn resolve_overlay_side(
    preferred_side: NavigationMenuSide,
    anchor_bounds: Bounds<Pixels>,
    popup_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
    side_offset: Pixels,
) -> NavigationMenuSide {
    let right_limit = (viewport_size.width - margin).max(margin);
    let bottom_limit = (viewport_size.height - margin).max(margin);
    let available_above = (anchor_bounds.top() - margin - side_offset).max(px(0.0));
    let available_below = (bottom_limit - anchor_bounds.bottom() - side_offset).max(px(0.0));
    let available_left = (anchor_bounds.left() - margin - side_offset).max(px(0.0));
    let available_right = (right_limit - anchor_bounds.right() - side_offset).max(px(0.0));

    let top_fits = popup_size.height <= available_above;
    let bottom_fits = popup_size.height <= available_below;
    let left_fits = popup_size.width <= available_left;
    let right_fits = popup_size.width <= available_right;

    match preferred_side {
        NavigationMenuSide::Top if top_fits => NavigationMenuSide::Top,
        NavigationMenuSide::Top if bottom_fits => NavigationMenuSide::Bottom,
        NavigationMenuSide::Bottom if bottom_fits => NavigationMenuSide::Bottom,
        NavigationMenuSide::Bottom if top_fits => NavigationMenuSide::Top,
        NavigationMenuSide::Top | NavigationMenuSide::Bottom => {
            match available_below >= available_above {
                true => NavigationMenuSide::Bottom,
                false => NavigationMenuSide::Top,
            }
        }
        NavigationMenuSide::Left if left_fits => NavigationMenuSide::Left,
        NavigationMenuSide::Left if right_fits => NavigationMenuSide::Right,
        NavigationMenuSide::Right if right_fits => NavigationMenuSide::Right,
        NavigationMenuSide::Right if left_fits => NavigationMenuSide::Left,
        NavigationMenuSide::Left | NavigationMenuSide::Right => {
            match available_right >= available_left {
                true => NavigationMenuSide::Right,
                false => NavigationMenuSide::Left,
            }
        }
        NavigationMenuSide::InlineStart | NavigationMenuSide::InlineEnd => preferred_side,
    }
}

fn overlay_bounds_for_side(
    side: NavigationMenuSide,
    align: NavigationMenuAlign,
    anchor_bounds: Bounds<Pixels>,
    popup_size: Size<Pixels>,
    side_offset: Pixels,
    align_offset: Pixels,
) -> Bounds<Pixels> {
    let origin = match side {
        NavigationMenuSide::Top => point(
            aligned_start(
                align,
                anchor_bounds.left(),
                anchor_bounds.right(),
                popup_size.width,
                align_offset,
            ),
            anchor_bounds.top() - popup_size.height - side_offset,
        ),
        NavigationMenuSide::Bottom
        | NavigationMenuSide::InlineStart
        | NavigationMenuSide::InlineEnd => point(
            aligned_start(
                align,
                anchor_bounds.left(),
                anchor_bounds.right(),
                popup_size.width,
                align_offset,
            ),
            anchor_bounds.bottom() + side_offset,
        ),
        NavigationMenuSide::Left => point(
            anchor_bounds.left() - popup_size.width - side_offset,
            aligned_start(
                align,
                anchor_bounds.top(),
                anchor_bounds.bottom(),
                popup_size.height,
                align_offset,
            ),
        ),
        NavigationMenuSide::Right => point(
            anchor_bounds.right() + side_offset,
            aligned_start(
                align,
                anchor_bounds.top(),
                anchor_bounds.bottom(),
                popup_size.height,
                align_offset,
            ),
        ),
    };

    Bounds::new(origin, popup_size)
}

fn aligned_start(
    align: NavigationMenuAlign,
    anchor_start: Pixels,
    anchor_end: Pixels,
    popup_length: Pixels,
    align_offset: Pixels,
) -> Pixels {
    let start = match align {
        NavigationMenuAlign::Start => anchor_start,
        NavigationMenuAlign::Center => (anchor_start + anchor_end) * 0.5 - popup_length * 0.5,
        NavigationMenuAlign::End => anchor_end - popup_length,
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

fn resolve_logical_side(side: NavigationMenuSide, direction: TextDirection) -> NavigationMenuSide {
    match (side, direction) {
        (NavigationMenuSide::InlineStart, TextDirection::Ltr) => NavigationMenuSide::Left,
        (NavigationMenuSide::InlineStart, TextDirection::Rtl) => NavigationMenuSide::Right,
        (NavigationMenuSide::InlineEnd, TextDirection::Ltr) => NavigationMenuSide::Right,
        (NavigationMenuSide::InlineEnd, TextDirection::Rtl) => NavigationMenuSide::Left,
        _ => side,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlay_position_prefers_bottom_when_it_fits() {
        let position = navigation_menu_overlay_position(
            NavigationMenuSide::Bottom,
            NavigationMenuAlign::Center,
            Bounds::new(point(px(100.0), px(80.0)), gpui::size(px(80.0), px(24.0))),
            gpui::size(px(120.0), px(30.0)),
            gpui::size(px(320.0), px(240.0)),
            px(5.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.side, NavigationMenuSide::Bottom);
        assert_eq!(position.bounds.top(), px(104.0));
    }

    #[test]
    fn overlay_position_flips_to_top_near_bottom_edge() {
        let anchor_bounds =
            Bounds::new(point(px(24.0), px(200.0)), gpui::size(px(120.0), px(32.0)));
        let position = navigation_menu_overlay_position(
            NavigationMenuSide::Bottom,
            NavigationMenuAlign::Center,
            anchor_bounds,
            gpui::size(px(140.0), px(48.0)),
            gpui::size(px(320.0), px(240.0)),
            px(5.0),
            px(0.0),
            px(0.0),
        );

        assert_eq!(position.side, NavigationMenuSide::Top);
        assert_eq!(position.bounds.bottom(), anchor_bounds.top());
    }
}
