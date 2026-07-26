use base_gpui::field::{FieldControl, FieldLabel, FieldRoot};
use base_gpui::fieldset::{FieldsetLegend, FieldsetRoot};
use gpui::prelude::*;
use gpui::{px, SharedString, Text};

use crate::theme;

fn field(id: &'static str, label: &'static str, placeholder: &'static str) -> FieldRoot {
    FieldRoot::new()
        .id(id)
        .name(label)
        .flex()
        .flex_col()
        .gap(px(4.))
        .child(
            FieldLabel::new()
                .text(label)
                .text_xs()
                .text_color(theme::text_muted()),
        )
        .child(
            FieldControl::new()
                .id(format!("{id}-control"))
                .placeholder(placeholder)
                .px(px(10.))
                .py(px(6.))
                .rounded(px(6.))
                .text_sm()
                .text_color(theme::text())
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .style_with_state(|state, control| {
                    if state.focused {
                        control.border_color(theme::focus_ring())
                    } else {
                        control
                    }
                }),
        )
}

pub fn render() -> impl IntoElement {
    FieldsetRoot::new()
        .id("demo-fieldset")
        .aria_label("Shipping address")
        .flex()
        .flex_col()
        .gap(px(10.))
        .w(px(280.))
        .p(px(14.))
        .rounded(px(8.))
        .bg(theme::surface())
        .border_1()
        .border_color(theme::border())
        .child(
            // Root carries the accessible name, so the visible legend text is
            // kept out of the a11y tree to avoid a double announcement.
            FieldsetLegend::new()
                .text_sm()
                .text_color(theme::text())
                .child(Text::new_inaccessible(SharedString::new_static(
                    "Shipping address",
                ))),
        )
        .child_any(field("demo-fieldset-name", "Full name", "Ada Lovelace"))
        .child_any(field("demo-fieldset-city", "City", "London"))
}
