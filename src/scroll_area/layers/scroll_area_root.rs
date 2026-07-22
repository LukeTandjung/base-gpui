//! Scroll Area Root: creates the shared context, reconciles scroll/overflow
//! facts at the top of render, arms the scroll-timeout timer, tracks
//! root-level hover, and hosts the wired parts in a `relative()` container
//! the scrollbars and corner overlay.

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::scroll_area::{
    child_wiring::wire_children, ScrollAreaChild, ScrollAreaContext, ScrollAreaEdgeThreshold,
    ScrollAreaProps, ScrollAreaRootStyleState,
};

#[derive(IntoElement)]
pub struct ScrollAreaRoot {
    id: ElementId,
    base: Div,
    children: Vec<ScrollAreaChild>,
    overflow_edge_threshold: ScrollAreaEdgeThreshold,
    style_with_state: Option<Rc<dyn Fn(ScrollAreaRootStyleState, Div) -> Div + 'static>>,
}

impl Default for ScrollAreaRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("scroll-area"),
            base: div(),
            children: Vec::new(),
            overflow_edge_threshold: ScrollAreaEdgeThreshold::default(),
            style_with_state: None,
        }
    }
}

impl Styled for ScrollAreaRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ScrollAreaRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = ScrollAreaContext::new(
            self.id.clone(),
            cx,
            window,
            ScrollAreaProps::new(self.overflow_edge_threshold),
        );

        // Render-top reconcile: expire stale scrolling flags and refresh
        // scroll/overflow facts from the handle. Notifies only on change.
        let handle = context.read(cx, |runtime, _| runtime.scroll_handle());
        let now = Instant::now();
        context.refresh(cx, |runtime, props| {
            let mut changed = runtime.expire_scrolling(now);
            changed |= runtime.observe_scroll(handle.offset(), now);
            changed |= runtime.refresh_overflow(
                handle.offset(),
                handle.max_offset(),
                props.overflow_edge_threshold(),
            );
            changed
        });

        // One re-armed timer clears the scrolling flags SCROLL_TIMEOUT after
        // the last activity; re-render after expiry re-arms it if the
        // deadline was extended. No per-frame polling while idle.
        let remaining = context.read(cx, |runtime, _| {
            if runtime.scroll_timer_scheduled() {
                None
            } else {
                runtime.remaining_scroll_activity(now)
            }
        });
        if let Some(remaining) = remaining {
            context.refresh(cx, |runtime, _| {
                runtime.set_scroll_timer_scheduled(true);
                false
            });
            let timer_context = context.clone();
            window
                .spawn(cx, async move |cx| {
                    cx.background_executor().timer(remaining).await;
                    cx.update(|_, cx| {
                        timer_context.update(cx, |runtime, _| {
                            runtime.set_scroll_timer_scheduled(false);
                            runtime.expire_scrolling(Instant::now());
                        });
                    })
                    .ok();
                })
                .detach();
        }

        let children = wire_children(self.children, context.clone());

        let style_state = context.read(cx, |runtime, _| runtime.root_state());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let hover_context = context.clone();
        base.id(self.id)
            .relative()
            .on_hover(move |hovered, _window, cx| {
                hover_context.refresh(cx, |runtime, _| runtime.set_hovering(*hovered));
            })
            .children(children)
    }
}

impl ScrollAreaRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<ScrollAreaChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<ScrollAreaChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    /// Distance thresholds gating the overflow-edge flags; accepts a
    /// uniform [`gpui::Pixels`] or a per-edge [`ScrollAreaEdgeThreshold`].
    /// Negative values clamp to zero; the default is zero.
    pub fn overflow_edge_threshold(
        mut self,
        threshold: impl Into<ScrollAreaEdgeThreshold>,
    ) -> Self {
        self.overflow_edge_threshold = threshold.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ScrollAreaRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
