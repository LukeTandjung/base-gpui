use base_gpui::separator::Separator;
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

fn label(text: &'static str) -> gpui::Div {
    div().text_sm().text_color(theme::text()).child(text)
}

pub fn render() -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(12.))
        .child(label("Docs"))
        .child(
            Separator::new()
                .id("demo-separator")
                .vertical()
                .w(px(1.))
                .h(px(16.))
                .bg(theme::border_strong()),
        )
        .child(label("Guides"))
        .child(
            Separator::new()
                .id("demo-separator-2")
                .vertical()
                .w(px(1.))
                .h(px(16.))
                .bg(theme::border_strong()),
        )
        .child(label("Reference"))
}
