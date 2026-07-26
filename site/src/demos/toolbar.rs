use base_gpui::toolbar::{ToolbarButton, ToolbarGroup, ToolbarRoot, ToolbarSeparator};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn glyph_button(id: &'static str, label: &'static str, glyph: &'static str) -> ToolbarButton {
    ToolbarButton::new()
        .id(id)
        .aria_label(label)
        .w(px(28.))
        .h(px(28.))
        .rounded(px(6.))
        .flex()
        .items_center()
        .justify_center()
        .text_sm()
        .style_with_state(|state, button| {
            if state.focused {
                button.bg(theme::control()).text_color(theme::text())
            } else {
                button.text_color(theme::text_muted())
            }
        })
        .child(glyph)
}

pub fn render() -> impl IntoElement {
    ToolbarRoot::new()
        .id("demo-toolbar")
        .aria_label("Formatting")
        .flex()
        .items_center()
        .gap(px(4.))
        .p(px(4.))
        .rounded(px(8.))
        .border_1()
        .border_color(theme::border())
        .bg(theme::surface())
        .child(
            ToolbarGroup::new()
                .id("demo-toolbar-format")
                .aria_label("Text style")
                .flex()
                .gap(px(2.))
                .child(glyph_button("demo-toolbar-bold", "Bold", "B"))
                .child(glyph_button("demo-toolbar-italic", "Italic", "I"))
                .child(glyph_button("demo-toolbar-underline", "Underline", "U")),
        )
        .child(
            ToolbarSeparator::new()
                .w(px(1.))
                .h(px(16.))
                .mx(px(4.))
                .bg(theme::border_strong()),
        )
        .child(
            ToolbarButton::new()
                .id("demo-toolbar-copy")
                .px(px(10.))
                .h(px(28.))
                .rounded(px(6.))
                .flex()
                .items_center()
                .text_sm()
                .style_with_state(|state, button| {
                    if state.focused {
                        button.bg(theme::control()).text_color(theme::text())
                    } else {
                        button.text_color(theme::text_muted())
                    }
                })
                .child("Copy"),
        )
}
