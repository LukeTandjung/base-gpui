use gpui::{
    div, point, prelude::*, px, size, Context, IntoElement, Modifiers, MouseButton, Render,
    ScrollHandle, TestAppContext, VisualTestContext, Window,
};

use crate::primitives::scroll::{scrollbar, ScrollbarAxis, ScrollbarVisibility};

struct ScrollbarHarness {
    handle: ScrollHandle,
}

impl Render for ScrollbarHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .w(px(200.0))
            .h(px(200.0))
            .child(
                div()
                    .id("scrollbar-harness-content")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.handle)
                    .child(div().w(px(200.0)).h(px(800.0))),
            )
            .child(
                scrollbar(&self.handle)
                    .id("scrollbar-harness-bar")
                    .axis(ScrollbarAxis::Vertical)
                    .visibility(ScrollbarVisibility::Always),
            )
    }
}

#[gpui::test]
fn dragging_the_thumb_scrolls_the_tracked_container(cx: &mut TestAppContext) {
    let window = cx.open_window(size(px(200.0), px(200.0)), |_, _| ScrollbarHarness {
        handle: ScrollHandle::new(),
    });
    let handle = window
        .update(cx, |harness, _, _| harness.handle.clone())
        .expect("harness window should be open");
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.run_until_parked();

    assert_eq!(handle.offset().y, px(0.0));

    // Content 800 in a 200 viewport: thumb is 50 long at the track top.
    // Grab the thumb and drag it 100px down.
    visual.simulate_mouse_down(
        point(px(194.0), px(25.0)),
        MouseButton::Left,
        Modifiers::none(),
    );
    visual.simulate_mouse_move(
        point(px(194.0), px(125.0)),
        MouseButton::Left,
        Modifiers::none(),
    );
    visual.run_until_parked();

    // Draggable range 150 maps onto scroll range 600: 100px → -400px.
    assert_eq!(handle.offset().y, px(-400.0));

    // Releasing outside the scrollbar bounds still ends the drag: further
    // non-dragging moves change nothing.
    visual.simulate_mouse_up(
        point(px(20.0), px(300.0)),
        MouseButton::Left,
        Modifiers::none(),
    );
    visual.simulate_mouse_move(point(px(194.0), px(60.0)), None, Modifiers::none());
    visual.run_until_parked();
    assert_eq!(handle.offset().y, px(-400.0));
}
