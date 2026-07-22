//! Scroll Area Scrollbar: one per-axis track strip that composes the
//! scrollbar primitive over the runtime's shared scroll handle. The
//! primitive's visibility policy is pinned to `Always`; showing/hiding is a
//! styling concern driven by this part's `style_with_state` over the Scroll
//! Area's `hovering`/`scrolling` facts, matching Base UI's CSS-driven
//! pattern. The strip reserves the corner at its end so tracks never
//! overlap the corner, and its measured cross-axis thickness feeds corner
//! sizing.

use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    div, px, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::primitives::scroll::{scrollbar, ScrollbarAxis, ScrollbarVisibility};
use crate::scroll_area::{
    child_wiring::ScrollAreaChildNode, ScrollAreaContext, ScrollAreaOrientation,
    ScrollAreaScrollbarChild, ScrollAreaScrollbarStyleState, ScrollAreaThumb,
};

#[derive(IntoElement)]
pub struct ScrollAreaScrollbar {
    id: ElementId,
    base: Div,
    orientation: ScrollAreaOrientation,
    keep_mounted: bool,
    thumb: Option<ScrollAreaThumb>,
    style_with_state: Option<Rc<dyn Fn(ScrollAreaScrollbarStyleState, Div) -> Div + 'static>>,
    context: Option<ScrollAreaContext>,
}

impl Default for ScrollAreaScrollbar {
    fn default() -> Self {
        Self {
            id: ElementId::from("scroll-area-scrollbar"),
            base: div(),
            orientation: ScrollAreaOrientation::default(),
            keep_mounted: false,
            thumb: None,
            style_with_state: None,
            context: None,
        }
    }
}

impl Styled for ScrollAreaScrollbar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ScrollAreaChildNode for ScrollAreaScrollbar {
    fn with_scroll_area_context(mut self, context: ScrollAreaContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl RenderOnce for ScrollAreaScrollbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context else {
            return gpui::Empty.into_any_element();
        };
        let orientation = self.orientation;

        let (hidden, handle, state, thumb_state, corner) = context.read(cx, |runtime, _| {
            (
                runtime.axis_hidden(orientation),
                runtime.scroll_handle(),
                runtime.scrollbar_state(orientation),
                runtime.thumb_state(orientation),
                runtime.corner_size(),
            )
        });

        // Unmounted while the axis has no overflow, unless kept mounted; a
        // kept-mounted strip stays in the tree with has_overflow() false.
        if hidden && !self.keep_mounted {
            return gpui::Empty.into_any_element();
        }

        // Corner reservation: the vertical track ends above the corner, the
        // horizontal track ends before it (Base UI's --corner-height /
        // --corner-width insets). The single-axis primitive reserves no end
        // margin of its own, so nothing is double-reserved.
        let strip = match orientation {
            ScrollAreaOrientation::Vertical => self
                .base
                .absolute()
                .top_0()
                .right_0()
                .bottom(corner.height)
                .w(px(12.0)),
            ScrollAreaOrientation::Horizontal => self
                .base
                .absolute()
                .left_0()
                .bottom_0()
                .right(corner.width)
                .h(px(12.0)),
        };
        let strip = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, strip),
            None => strip,
        };

        // The Thumb part contributes styling only; geometry, drag mapping,
        // track clicks, and wheel-over-track belong to the primitive.
        let thumb_style = self.thumb.and_then(ScrollAreaThumb::take_style);
        let primitive = scrollbar(&handle)
            .id(ElementId::NamedChild(
                Arc::new(self.id.clone()),
                SharedString::from("primitive"),
            ))
            .axis(match orientation {
                ScrollAreaOrientation::Vertical => ScrollbarAxis::Vertical,
                ScrollAreaOrientation::Horizontal => ScrollbarAxis::Horizontal,
            })
            .visibility(ScrollbarVisibility::Always)
            .style_with_state(move |_primitive_state, style| match &thumb_style {
                Some(thumb_style) => thumb_style(thumb_state, style),
                None => style,
            });

        let thickness_context = context.clone();
        strip
            .child(primitive)
            .on_children_prepainted(move |bounds, _window, cx| {
                // The primitive fills the strip, so its bounds measure the
                // strip's cross-axis thickness for corner sizing.
                let Some(strip_bounds) = bounds.first() else {
                    return;
                };
                let thickness = match orientation {
                    ScrollAreaOrientation::Vertical => strip_bounds.size.width,
                    ScrollAreaOrientation::Horizontal => strip_bounds.size.height,
                };
                thickness_context.refresh(cx, |runtime, _| {
                    runtime.set_scrollbar_thickness(orientation, thickness)
                });
            })
            .id(self.id)
            .into_any_element()
    }
}

impl ScrollAreaScrollbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Which axis this scrollbar tracks; defaults to vertical.
    pub fn orientation(mut self, orientation: ScrollAreaOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Keep the scrollbar in the tree while its axis has no overflow;
    /// defaults to false.
    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn child(mut self, child: impl Into<ScrollAreaScrollbarChild>) -> Self {
        let ScrollAreaScrollbarChild::Thumb(thumb) = child.into();
        self.thumb = Some(thumb);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ScrollAreaScrollbarStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
