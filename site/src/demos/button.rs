use base_gpui::button::ButtonRoot;
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    ButtonRoot::new()
        .id("demo-button")
        .aria_label("Say hello")
        .px(px(14.))
        .py(px(8.))
        .rounded(px(6.))
        .bg(theme::accent())
        .text_color(theme::text_inverted())
        .text_sm()
        .style_with_state(|state, root| {
            let root = if state.focused {
                root.border_2().border_color(theme::focus_ring())
            } else {
                root
            };
            root.hover(|style| style.bg(theme::accent_hover()))
        })
        .on_click(|_, _, _| {})
        .child("Say hello")
}
