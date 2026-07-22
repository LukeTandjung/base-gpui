use gpui::{
    div, prelude::*, px, size, Context, IntoElement, Render, ScrollHandle, TestAppContext, Window,
};

use crate::primitives::scroll::ScrollTarget;

struct ScrollContainerHarness {
    handle: ScrollHandle,
}

impl Render for ScrollContainerHarness {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("scroll-target-harness")
            .w(px(200.0))
            .h(px(100.0))
            .overflow_scroll()
            .track_scroll(&self.handle)
            .child(div().w(px(300.0)).h(px(400.0)))
    }
}

#[gpui::test]
fn scroll_handle_target_derives_content_size_from_max_offset_and_bounds(cx: &mut TestAppContext) {
    let window = cx.open_window(size(px(200.0), px(100.0)), |_, _| ScrollContainerHarness {
        handle: ScrollHandle::new(),
    });
    cx.run_until_parked();
    let handle = window
        .update(cx, |harness, _, _| harness.handle.clone())
        .expect("harness window should be open");

    let viewport = ScrollTarget::viewport_size(&handle);
    let content = ScrollTarget::content_size(&handle);
    assert_eq!(viewport, size(px(200.0), px(100.0)));
    assert_eq!(content, size(px(300.0), px(400.0)));

    // content_size == max_offset + bounds().size, per the trait contract.
    let max_offset = handle.max_offset();
    assert_eq!(
        content,
        size(
            max_offset.x + viewport.width,
            max_offset.y + viewport.height
        )
    );
}
