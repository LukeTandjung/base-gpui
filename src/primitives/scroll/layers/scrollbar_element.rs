//! The scrollbar custom element: absolute-positioned overlay that paints a
//! track and draggable thumb per axis over any [`ScrollTarget`].
//!
//! The element is thin: it computes per-axis geometry in `prepaint` through
//! the runtime's pure functions, paints from runtime queries, and translates
//! mouse events into runtime commands during `paint`. Overlay composition:
//! wrap the scrollable content in a `relative()` container and add
//! `scrollbar(&handle)` as a sibling layer — it fills the container and
//! paints along its right/bottom edges.

use std::panic::Location;
use std::rc::Rc;
use std::time::Instant;

use gpui::{
    fill, point, px, relative, size, App, Axis, Bounds, ContentMask, Element, ElementId, Entity,
    GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId, IntoElement, LayoutId,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Position, ScrollWheelEvent, Size, Style,
    Window,
};

use crate::primitives::scroll::{
    axis_geometry, corner_size, drag_scroll_position, horizontal_margin_end,
    scroll_offset_for_axis, track_click_scroll_position, ScrollTarget, ScrollbarAxis,
    ScrollbarAxisGeometry, ScrollbarFadePhase, ScrollbarRuntime, ScrollbarStyle,
    ScrollbarStyleState, ScrollbarVisibility,
};

/// Create a scrollbar over any scroll target. Defaults to both axes and
/// [`ScrollbarVisibility::Scrolling`], identified by the call site.
#[track_caller]
pub fn scrollbar<H: ScrollTarget + Clone>(target: &H) -> Scrollbar {
    Scrollbar {
        id: ElementId::CodeLocation(*Location::caller()),
        target: Rc::new(target.clone()),
        axis: ScrollbarAxis::Both,
        visibility: ScrollbarVisibility::default(),
        content_size: None,
        style_with_state: None,
    }
}

/// Create a vertical-only scrollbar. See [`scrollbar`].
#[track_caller]
pub fn scrollbar_vertical<H: ScrollTarget + Clone>(target: &H) -> Scrollbar {
    scrollbar(target).axis(ScrollbarAxis::Vertical)
}

/// Create a horizontal-only scrollbar. See [`scrollbar`].
#[track_caller]
pub fn scrollbar_horizontal<H: ScrollTarget + Clone>(target: &H) -> Scrollbar {
    scrollbar(target).axis(ScrollbarAxis::Horizontal)
}

/// Overlay scrollbar element. Build with [`scrollbar`].
pub struct Scrollbar {
    id: ElementId,
    target: Rc<dyn ScrollTarget>,
    axis: ScrollbarAxis,
    visibility: ScrollbarVisibility,
    content_size: Option<Size<Pixels>>,
    style_with_state: Option<Rc<dyn Fn(ScrollbarStyleState, ScrollbarStyle) -> ScrollbarStyle>>,
}

impl Scrollbar {
    /// Stable identity for the keyed interaction state (hover, drag, fade).
    /// Defaults to the constructor call site.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Which axes to render.
    pub fn axis(mut self, axis: ScrollbarAxis) -> Self {
        self.axis = axis;
        self
    }

    /// Visibility policy; defaults to [`ScrollbarVisibility::Scrolling`].
    pub fn visibility(mut self, visibility: ScrollbarVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Explicit content-size override; defaults to the target's
    /// `content_size()`.
    pub fn content_size(mut self, content_size: Size<Pixels>) -> Self {
        self.content_size = Some(content_size);
        self
    }

    /// Adjust track/thumb appearance from the current typed style state.
    /// Receives [`ScrollbarStyle::default`] and returns the style to use.
    pub fn style_with_state(
        mut self,
        f: impl Fn(ScrollbarStyleState, ScrollbarStyle) -> ScrollbarStyle + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(f));
        self
    }

    fn resolve_style(&self, state: ScrollbarStyleState) -> ScrollbarStyle {
        let default = ScrollbarStyle::default();
        match &self.style_with_state {
            Some(f) => f(state, default),
            None => default,
        }
    }
}

/// Per-axis facts carried from prepaint to paint.
pub struct ScrollbarAxisPrepaint {
    axis: Axis,
    geometry: ScrollbarAxisGeometry,
    bar_bounds: Bounds<Pixels>,
    thumb_bounds: Bounds<Pixels>,
    thumb_fill_bounds: Bounds<Pixels>,
    style: ScrollbarStyle,
    opacity: f32,
    content_len: Pixels,
    viewport_len: Pixels,
    _bar_hitbox: Hitbox,
}

/// Prepaint output: full-element hitbox plus per-axis geometry and the
/// corner square when both axes render.
pub struct ScrollbarPrepaint {
    hitbox: Hitbox,
    runtime: Entity<ScrollbarRuntime>,
    axes: Vec<ScrollbarAxisPrepaint>,
    corner: Option<(Bounds<Pixels>, ScrollbarStyle, f32)>,
}

impl IntoElement for Scrollbar {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Scrollbar {
    type RequestLayoutState = ();
    type PrepaintState = ScrollbarPrepaint;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.position = Position::Absolute;
        style.inset.top = px(0.0).into();
        style.inset.left = px(0.0).into();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let hitbox = window.with_content_mask(Some(ContentMask { bounds }), |window| {
            window.insert_hitbox(bounds, HitboxBehavior::Normal)
        });
        let runtime = window.use_keyed_state(self.id.clone(), cx, |_, _| ScrollbarRuntime::new());
        let now = Instant::now();
        let visibility = self.visibility;

        // Observe the target's current offset and keep the idle clock fresh
        // while hovered; notify only when the offset changed so repaints
        // settle once activity stops.
        let offset = self.target.offset();
        runtime.update(cx, |runtime, cx| {
            let offset_changed = runtime.observe_offset(offset, now);
            runtime.refresh_hover_activity(visibility, now);
            if offset_changed {
                cx.notify();
            }
        });

        let content = self
            .content_size
            .unwrap_or_else(|| self.target.content_size());
        let mut viewport = self.target.viewport_size();
        if viewport.width <= px(0.0) || viewport.height <= px(0.0) {
            viewport = bounds.size;
        }

        // Fade scheduling: one idle timer during the solid window, animation
        // frames during the fade window, nothing while hidden.
        match runtime.read(cx).fade_phase(visibility, now) {
            ScrollbarFadePhase::Solid { remaining_delay }
                if remaining_delay < std::time::Duration::MAX =>
            {
                if !runtime.read(cx).idle_timer_scheduled() {
                    runtime.update(cx, |runtime, _| runtime.set_idle_timer_scheduled(true));
                    let runtime = runtime.clone();
                    window
                        .spawn(cx, async move |cx| {
                            cx.background_executor().timer(remaining_delay).await;
                            cx.update(|_, cx| {
                                runtime.update(cx, |runtime, cx| {
                                    runtime.set_idle_timer_scheduled(false);
                                    cx.notify();
                                });
                            })
                            .ok();
                        })
                        .detach();
                }
            }
            ScrollbarFadePhase::Fading => window.request_animation_frame(),
            _ => {}
        }

        let default_thickness = ScrollbarStyle::default().thickness;
        let vertical_visible = self.axis.has_vertical() && content.height > viewport.height;
        let horizontal_visible = self.axis.has_horizontal() && content.width > viewport.width;

        let mut axes = Vec::new();
        let mut vertical_style: Option<ScrollbarStyle> = None;
        let mut horizontal_style: Option<ScrollbarStyle> = None;

        for axis in [Axis::Vertical, Axis::Horizontal] {
            let visible = match axis {
                Axis::Vertical => vertical_visible,
                Axis::Horizontal => horizontal_visible,
            };
            if !visible {
                continue;
            }
            let (content_len, viewport_len, scroll_offset, track_len) = match axis {
                Axis::Vertical => (
                    content.height,
                    viewport.height,
                    offset.y,
                    bounds.size.height,
                ),
                Axis::Horizontal => (content.width, viewport.width, offset.x, bounds.size.width),
            };
            let has_overflow = content_len > viewport_len;
            let max_scroll = content_len - viewport_len;
            let at_start = -scroll_offset <= px(0.5);
            let at_end = -scroll_offset >= max_scroll - px(0.5);
            let state =
                runtime
                    .read(cx)
                    .style_state(axis, visibility, now, has_overflow, at_start, at_end);
            let opacity = state.opacity;
            let style = self.resolve_style(state);
            match axis {
                Axis::Vertical => vertical_style = Some(style),
                Axis::Horizontal => horizontal_style = Some(style),
            }

            let margin_end = match axis {
                Axis::Vertical => px(0.0),
                Axis::Horizontal => horizontal_margin_end(
                    vertical_visible,
                    vertical_style.map_or(default_thickness, |s| s.thickness),
                ),
            };
            let Some(geometry) = axis_geometry(
                content_len,
                viewport_len,
                scroll_offset,
                track_len,
                margin_end,
            ) else {
                continue;
            };

            let bar_bounds = match axis {
                Axis::Vertical => Bounds {
                    origin: point(bounds.right() - style.thickness, bounds.top()),
                    size: size(style.thickness, bounds.size.height),
                },
                Axis::Horizontal => Bounds {
                    origin: point(bounds.left(), bounds.bottom() - style.thickness),
                    size: size(bounds.size.width - margin_end, style.thickness),
                },
            };
            let thumb_bounds = match axis {
                Axis::Vertical => Bounds {
                    origin: point(bar_bounds.left(), bar_bounds.top() + geometry.thumb_offset),
                    size: size(style.thickness, geometry.thumb_len),
                },
                Axis::Horizontal => Bounds {
                    origin: point(bar_bounds.left() + geometry.thumb_offset, bar_bounds.top()),
                    size: size(geometry.thumb_len, style.thickness),
                },
            };
            let inset = style.inset;
            let thumb_fill_bounds = Bounds {
                origin: thumb_bounds.origin + point(inset, inset),
                size: size(
                    (thumb_bounds.size.width - inset * 2.0).max(px(0.0)),
                    (thumb_bounds.size.height - inset * 2.0).max(px(0.0)),
                ),
            };
            let bar_hitbox = window
                .with_content_mask(Some(ContentMask { bounds: bar_bounds }), |window| {
                    window.insert_hitbox(bar_bounds, HitboxBehavior::Normal)
                });

            axes.push(ScrollbarAxisPrepaint {
                axis,
                geometry,
                bar_bounds,
                thumb_bounds,
                thumb_fill_bounds,
                style,
                opacity,
                content_len,
                viewport_len,
                _bar_hitbox: bar_hitbox,
            });
        }

        let corner = match (vertical_style, horizontal_style) {
            (Some(v), Some(h)) => {
                let corner = corner_size(v.thickness, h.thickness);
                let corner_bounds = Bounds {
                    origin: point(
                        bounds.right() - corner.width,
                        bounds.bottom() - corner.height,
                    ),
                    size: corner,
                };
                let opacity = axes.iter().map(|a| a.opacity).fold(0.0, f32::max);
                Some((corner_bounds, v, opacity))
            }
            _ => None,
        };

        ScrollbarPrepaint {
            hitbox,
            runtime,
            axes,
            corner,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let runtime = prepaint.runtime.clone();
        let target = self.target.clone();
        let visibility = self.visibility;
        let view_id = window.current_view();
        let hitbox_bounds = prepaint.hitbox.bounds;
        let now = Instant::now();
        let interactable = runtime.read(cx).is_interactable(visibility, now);

        window.with_content_mask(
            Some(ContentMask {
                bounds: hitbox_bounds,
            }),
            |window| {
                for axis_state in prepaint.axes.iter() {
                    let axis = axis_state.axis;
                    let style = axis_state.style;
                    let opacity = axis_state.opacity;
                    let bar_bounds = axis_state.bar_bounds;
                    let thumb_bounds = axis_state.thumb_bounds;
                    let geometry = axis_state.geometry;
                    let content_len = axis_state.content_len;
                    let viewport_len = axis_state.viewport_len;

                    if opacity > 0.0 {
                        window.paint_quad(fill(bar_bounds, style.track_color.opacity(opacity)));
                        window.paint_quad(
                            fill(
                                axis_state.thumb_fill_bounds,
                                style.thumb_color.opacity(opacity),
                            )
                            .corner_radii(style.corner_radius),
                        );
                    }

                    // Wheel over the track: the scroll container handles the
                    // wheel; we only refresh visibility facts.
                    window.on_mouse_event({
                        let runtime = runtime.clone();
                        let target = target.clone();
                        move |event: &ScrollWheelEvent, phase, _, cx| {
                            if phase.bubble() && hitbox_bounds.contains(&event.position) {
                                let offset = target.offset();
                                runtime.update(cx, |runtime, cx| {
                                    if runtime.observe_offset(offset, Instant::now()) {
                                        cx.notify();
                                    }
                                });
                            }
                        }
                    });

                    if interactable {
                        window.on_mouse_event({
                            let runtime = runtime.clone();
                            let target = target.clone();
                            move |event: &MouseDownEvent, phase, _, cx| {
                                if !phase.bubble() || !bar_bounds.contains(&event.position) {
                                    return;
                                }
                                cx.stop_propagation();
                                let now = Instant::now();
                                if thumb_bounds.contains(&event.position) {
                                    let grab = match axis {
                                        Axis::Vertical => event.position.y - thumb_bounds.top(),
                                        Axis::Horizontal => event.position.x - thumb_bounds.left(),
                                    };
                                    target.drag_started();
                                    runtime.update(cx, |runtime, cx| {
                                        runtime.begin_drag(axis, grab, now);
                                        cx.notify();
                                    });
                                } else {
                                    let (click, track_origin) = match axis {
                                        Axis::Vertical => (event.position.y, bar_bounds.top()),
                                        Axis::Horizontal => (event.position.x, bar_bounds.left()),
                                    };
                                    let position = track_click_scroll_position(
                                        &geometry,
                                        click,
                                        track_origin,
                                        content_len,
                                        viewport_len,
                                    );
                                    target.set_offset(scroll_offset_for_axis(
                                        axis,
                                        position,
                                        target.offset(),
                                    ));
                                    runtime.update(cx, |runtime, cx| {
                                        runtime.observe_offset(target.offset(), now);
                                        cx.notify();
                                    });
                                }
                            }
                        });
                    }

                    window.on_mouse_event({
                        let runtime = runtime.clone();
                        let target = target.clone();
                        move |event: &MouseMoveEvent, _, _, cx| {
                            let now = Instant::now();
                            let over_track = bar_bounds.contains(&event.position);
                            let over_thumb = thumb_bounds.contains(&event.position);
                            let dragging_this_axis = runtime
                                .read(cx)
                                .drag()
                                .is_some_and(|(drag_axis, _)| drag_axis == axis);

                            let hover_changed = runtime.update(cx, |runtime, _| {
                                let track_changed =
                                    runtime.set_track_hovered(axis, over_track, visibility, now);
                                let thumb_changed =
                                    runtime.set_thumb_hovered(axis, over_thumb, visibility, now);
                                track_changed || thumb_changed
                            });

                            let mut moved = false;
                            if dragging_this_axis && event.dragging() {
                                cx.stop_propagation();
                                let claimed = runtime
                                    .update(cx, |runtime, _| runtime.try_claim_drag_update(now));
                                if claimed {
                                    if let Some((_, grab)) = runtime.read(cx).drag() {
                                        let (pointer, track_origin) = match axis {
                                            Axis::Vertical => (event.position.y, bar_bounds.top()),
                                            Axis::Horizontal => {
                                                (event.position.x, bar_bounds.left())
                                            }
                                        };
                                        let position = drag_scroll_position(
                                            &geometry,
                                            pointer,
                                            grab,
                                            track_origin,
                                            content_len,
                                            viewport_len,
                                        );
                                        target.set_offset(scroll_offset_for_axis(
                                            axis,
                                            position,
                                            target.offset(),
                                        ));
                                        runtime.update(cx, |runtime, _| {
                                            runtime.observe_offset(target.offset(), now);
                                        });
                                        moved = true;
                                    }
                                }
                            }

                            if hover_changed || moved {
                                cx.notify(view_id);
                            }
                        }
                    });
                }

                if let Some((corner_bounds, style, opacity)) = prepaint.corner {
                    if opacity > 0.0 {
                        window.paint_quad(fill(corner_bounds, style.track_color.opacity(opacity)));
                    }
                }

                // Mouse up anywhere ends the drag, including outside bounds.
                window.on_mouse_event({
                    let runtime = runtime.clone();
                    let target = target.clone();
                    move |_event: &MouseUpEvent, phase, _, cx| {
                        if phase.bubble() {
                            let ended = runtime.update(cx, |runtime, cx| {
                                let ended = runtime.end_drag(Instant::now());
                                if ended {
                                    cx.notify();
                                }
                                ended
                            });
                            if ended {
                                target.drag_ended();
                            }
                        }
                    }
                });
            },
        );
    }
}
