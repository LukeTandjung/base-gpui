use base_gpui::radio_group::{RadioGroupIndicator, RadioGroupRadio, RadioGroupRoot};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

fn radio(value: &'static str, label: &'static str) -> RadioGroupRadio<&'static str> {
    RadioGroupRadio::new()
        .id(format!("demo-radio-group-{value}"))
        .value(value)
        .aria_label(label)
        .w(px(16.))
        .h(px(16.))
        .rounded_full()
        .border_1()
        .flex()
        .items_center()
        .justify_center()
        .style_with_state(|state, radio| {
            let radio = if state.checked {
                radio.border_color(theme::accent())
            } else {
                radio.border_color(theme::border_strong())
            };
            if state.focused {
                radio.border_color(theme::focus_ring())
            } else {
                radio
            }
        })
        .child(
            RadioGroupIndicator::new().child(
                div().w(px(8.)).h(px(8.)).rounded_full().bg(theme::accent()),
            ),
        )
}

pub fn render() -> impl IntoElement {
    // Radios accept only Indicator children, so visible labels sit in a
    // parallel column with matching row metrics.
    div()
        .flex()
        .gap(px(8.))
        .child(
            RadioGroupRoot::<&'static str>::new()
                .id("demo-radio-group")
                .aria_label("Interface density")
                .default_value(Some("comfortable"))
                .flex()
                .flex_col()
                .gap(px(12.))
                .child(radio("compact", "Compact"))
                .child(radio("comfortable", "Comfortable"))
                .child(radio("spacious", "Spacious")),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(12.))
                .text_sm()
                .text_color(theme::text())
                .children(
                    ["Compact", "Comfortable", "Spacious"]
                        .map(|label| div().h(px(16.)).flex().items_center().child(label)),
                ),
        )
}
