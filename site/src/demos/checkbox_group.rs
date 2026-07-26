use base_gpui::checkbox::{CheckboxIndicator, CheckboxRoot};
use base_gpui::checkbox_group::CheckboxGroup;
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

fn row(value: &'static str, label: &'static str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .child(
            CheckboxRoot::new()
                .id(format!("demo-checkbox-group-{value}"))
                .value(value)
                .aria_label(label)
                .w(px(18.))
                .h(px(18.))
                .rounded(px(4.))
                .border_1()
                .flex()
                .items_center()
                .justify_center()
                .style_with_state(|state, root| {
                    let root = if state.checked {
                        root.bg(theme::accent()).border_color(theme::accent())
                    } else {
                        root.bg(theme::background()).border_color(theme::border_strong())
                    };
                    if state.focused {
                        root.border_color(theme::focus_ring())
                    } else {
                        root
                    }
                })
                .child(
                    CheckboxIndicator::new().child(
                        div()
                            .text_size(px(11.))
                            .text_color(theme::text_inverted())
                            .child("✓"),
                    ),
                ),
        )
        .child(div().text_sm().text_color(theme::text()).child(label))
}

pub fn render() -> impl IntoElement {
    CheckboxGroup::new()
        .id("demo-checkbox-group")
        .aria_label("Notifications")
        .default_value(["updates"])
        .flex()
        .flex_col()
        .gap(px(10.))
        .child(div().text_sm().text_color(theme::text_muted()).child("Notifications"))
        .child(row("updates", "Product updates"))
        .child(row("digest", "Weekly digest"))
        .child(row("mentions", "Mentions"))
}
