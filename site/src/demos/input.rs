use base_gpui::input::Input;
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

pub fn render() -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(6.))
        .w(px(260.))
        .child(
            div()
                .text_sm()
                .text_color(theme::text_muted())
                .child("Email"),
        )
        .child(
            Input::new()
                .id("demo-input")
                .name("email")
                .aria_label("Email")
                .placeholder("you@example.com")
                .w_full()
                .px(px(10.))
                .py(px(8.))
                .rounded(px(6.))
                .text_sm()
                .text_color(theme::text())
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .style_with_state(|state, input| {
                    if state.focused {
                        input.border_color(theme::focus_ring())
                    } else {
                        input
                    }
                }),
        )
}
