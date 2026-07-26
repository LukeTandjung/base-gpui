use base_gpui::toggle::Toggle;
use gpui::prelude::*;
use gpui::{px, SharedString, Text};

use crate::theme;

pub fn render() -> impl IntoElement {
    Toggle::<SharedString>::new()
        .id("demo-toggle")
        .aria_label("Bold")
        .default_pressed(false)
        .w(px(36.))
        .h(px(36.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(6.))
        .text_sm()
        .style_with_state(|state, toggle| {
            let toggle = if state.pressed {
                toggle
                    .bg(theme::accent())
                    .text_color(theme::text_inverted())
            } else {
                toggle.bg(theme::control()).text_color(theme::text())
            };
            if state.focused {
                toggle.border_2().border_color(theme::focus_ring())
            } else {
                toggle
            }
        })
        // aria_label carries the accessible name; the glyph stays out of the
        // a11y tree.
        .child(Text::new_inaccessible(SharedString::new_static("B")))
}
