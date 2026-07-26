use base_gpui::primitives::scroll::ScrollbarStyle;
use base_gpui::scroll_area::{
    ScrollAreaContent, ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport,
};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

pub fn render() -> impl IntoElement {
    ScrollAreaRoot::new()
        .id("demo-scroll-area")
        .w(px(280.))
        .h(px(160.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme::border())
        .bg(theme::surface())
        .child(
            ScrollAreaViewport::new()
                .id("demo-scroll-area-viewport")
                .aria_label("Changelog entries")
                .size_full()
                .child(
                    ScrollAreaContent::new()
                        .flex()
                        .flex_col()
                        .children((1..=12).map(|n| {
                            div()
                                .px(px(12.))
                                .py(px(6.))
                                .text_sm()
                                .text_color(theme::text_muted())
                                .child(format!("v0.{n:02} — patch notes"))
                        })),
                ),
        )
        .child(
            ScrollAreaScrollbar::new()
                .id("demo-scroll-area-scrollbar")
                .child(ScrollAreaThumb::new().style_with_state(|state, style| {
                    ScrollbarStyle {
                        thumb_color: if state.scrolling {
                            theme::text_muted().into()
                        } else {
                            theme::border_strong().into()
                        },
                        ..style
                    }
                })),
        )
}
