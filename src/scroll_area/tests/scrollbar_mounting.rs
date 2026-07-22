//! Scrollbar mount policy: without overflow a `keep_mounted(false)`
//! scrollbar is absent from the tree (its style callback never runs), while
//! a `keep_mounted(true)` scrollbar stays mounted with `has_overflow()`
//! false.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    div, prelude::*, px, size, Context, IntoElement, Render, TestAppContext, VisualTestContext,
    Window,
};

use crate::scroll_area::{
    ScrollAreaOrientation, ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport,
};

struct MountHarness {
    unmounted_rendered: Rc<Cell<bool>>,
    kept_overflow: Rc<Cell<Option<bool>>>,
}

impl Render for MountHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let unmounted_rendered = self.unmounted_rendered.clone();
        let kept_overflow = self.kept_overflow.clone();
        ScrollAreaRoot::new()
            .id("mount-scroll-area")
            .w(px(200.0))
            .h(px(200.0))
            .child(
                ScrollAreaViewport::new()
                    .id("mount-scroll-area-viewport")
                    .size_full()
                    // Content fits the viewport: no overflow on either axis.
                    .child_any(div().w(px(100.0)).h(px(100.0))),
            )
            .child(
                ScrollAreaScrollbar::new()
                    .id("mount-scroll-area-unmounted")
                    .orientation(ScrollAreaOrientation::Vertical)
                    .style_with_state(move |_state, strip| {
                        unmounted_rendered.set(true);
                        strip
                    })
                    .child(ScrollAreaThumb::new()),
            )
            .child(
                ScrollAreaScrollbar::new()
                    .id("mount-scroll-area-kept")
                    .orientation(ScrollAreaOrientation::Horizontal)
                    .keep_mounted(true)
                    .style_with_state(move |state, strip| {
                        kept_overflow.set(Some(state.has_overflow()));
                        strip
                    })
                    .child(ScrollAreaThumb::new()),
            )
    }
}

#[gpui::test]
fn scrollbar_without_overflow_unmounts_unless_kept_mounted(cx: &mut TestAppContext) {
    let unmounted_rendered = Rc::new(Cell::new(false));
    let kept_overflow = Rc::new(Cell::new(None));
    let window = cx.open_window(size(px(200.0), px(200.0)), |_, _| MountHarness {
        unmounted_rendered: unmounted_rendered.clone(),
        kept_overflow: kept_overflow.clone(),
    });
    let visual = VisualTestContext::from_window(window.into(), cx);
    visual.run_until_parked();

    assert!(
        !unmounted_rendered.get(),
        "non-kept scrollbar must render nothing while its axis has no overflow"
    );
    assert_eq!(
        kept_overflow.get(),
        Some(false),
        "kept-mounted scrollbar stays in the tree with has_overflow() false"
    );
}
