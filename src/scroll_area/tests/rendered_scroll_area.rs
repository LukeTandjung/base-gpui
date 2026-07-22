//! Rendered behavior: mounting Root > Viewport > Content with overflowing
//! content derives overflow state on the first frames without any user
//! scrolling, with both scrollbars, thumbs, and a corner composed over the
//! primitive.

use std::cell::Cell;
use std::rc::Rc;

use gpui::{
    div, prelude::*, px, size, Context, IntoElement, Render, TestAppContext, VisualTestContext,
    Window,
};

use crate::scroll_area::{
    ScrollAreaContent, ScrollAreaCorner, ScrollAreaOrientation, ScrollAreaRoot,
    ScrollAreaRootStyleState, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport,
};

struct ScrollAreaHarness {
    observed: Rc<Cell<ScrollAreaRootStyleState>>,
}

impl Render for ScrollAreaHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let observed = self.observed.clone();
        ScrollAreaRoot::new()
            .id("rendered-scroll-area")
            .w(px(200.0))
            .h(px(200.0))
            .style_with_state(move |state, root| {
                observed.set(state);
                root
            })
            .child(
                ScrollAreaViewport::new()
                    .id("rendered-scroll-area-viewport")
                    .size_full()
                    .child(ScrollAreaContent::new().child(div().w(px(600.0)).h(px(800.0)))),
            )
            .child(
                ScrollAreaScrollbar::new()
                    .id("rendered-scroll-area-vertical")
                    .orientation(ScrollAreaOrientation::Vertical)
                    .keep_mounted(true)
                    .child(ScrollAreaThumb::new()),
            )
            .child(
                ScrollAreaScrollbar::new()
                    .id("rendered-scroll-area-horizontal")
                    .orientation(ScrollAreaOrientation::Horizontal)
                    .child(ScrollAreaThumb::new()),
            )
            .child(ScrollAreaCorner::new())
    }
}

#[gpui::test]
fn overflowing_content_derives_overflow_state_without_scrolling(cx: &mut TestAppContext) {
    let observed = Rc::new(Cell::new(ScrollAreaRootStyleState::default()));
    let window = cx.open_window(size(px(200.0), px(200.0)), |_, _| ScrollAreaHarness {
        observed: observed.clone(),
    });
    let visual = VisualTestContext::from_window(window.into(), cx);
    visual.run_until_parked();
    // Extra frames let post-layout observation feed back into style state.
    visual.simulate_resize(size(px(200.0), px(200.0)));
    visual.run_until_parked();

    let state = observed.get();
    assert!(
        state.has_overflow_x,
        "600px content overflows a 200px viewport"
    );
    assert!(
        state.has_overflow_y,
        "800px content overflows a 200px viewport"
    );
    assert!(state.overflow_x_end && state.overflow_y_end);
    assert!(!state.overflow_x_start && !state.overflow_y_start);
    assert!(!state.corner_hidden);
    assert!(!state.scrolling);
}
