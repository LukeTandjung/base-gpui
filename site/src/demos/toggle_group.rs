use base_gpui::toggle::Toggle;
use base_gpui::toggle_group::ToggleGroup;
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(value: &'static str, label: &'static str, glyph: &'static str) -> Toggle<&'static str> {
    Toggle::new()
        .id(format!("demo-toggle-group-{value}"))
        .value(value)
        .aria_label(label)
        .w(px(36.))
        .h(px(28.))
        .rounded(px(6.))
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .style_with_state(|state, toggle| {
            if state.pressed {
                toggle.bg(theme::background()).text_color(theme::text())
            } else {
                toggle.text_color(theme::text_muted())
            }
        })
        .child(glyph)
}

pub fn render() -> impl IntoElement {
    ToggleGroup::<&'static str>::new()
        .id("demo-toggle-group")
        .aria_label("Text alignment")
        .default_value(vec!["left"])
        .flex()
        .gap(px(4.))
        .p(px(4.))
        .rounded(px(8.))
        .bg(theme::control())
        .child(item("left", "Align left", "L"))
        .child(item("center", "Align center", "C"))
        .child(item("right", "Align right", "R"))
}
