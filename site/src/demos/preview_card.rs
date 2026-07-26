use std::time::Duration;

use base_gpui::preview_card::{
    PreviewCardPopup, PreviewCardPortal, PreviewCardPositioner, PreviewCardRoot, PreviewCardTrigger,
};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

pub fn render() -> impl IntoElement {
    PreviewCardRoot::<()>::new()
        .id("demo-preview-card")
        .delay(Duration::from_millis(300))
        .flex()
        .gap(px(4.))
        .text_sm()
        .text_color(theme::text_muted())
        .child_any("Built with")
        .child(
            PreviewCardTrigger::new()
                .id("demo-preview-card-trigger")
                .aria_label("base-gpui project")
                .text_color(theme::text())
                .border_b_1()
                .border_color(theme::border_strong())
                .style_with_state(|state, trigger| {
                    let trigger = if state.focused {
                        trigger.border_color(theme::focus_ring())
                    } else {
                        trigger
                    };
                    if state.open || state.hovered {
                        trigger.border_color(theme::accent())
                    } else {
                        trigger
                    }
                })
                .child("base-gpui"),
        )
        .child(
            PreviewCardPortal::new().child(
                PreviewCardPositioner::new().side_offset(px(8.)).child(
                    PreviewCardPopup::new()
                        .id("demo-preview-card-popup")
                        .w(px(280.))
                        .p(px(14.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child_any(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.))
                                .child(div().text_sm().text_color(theme::text()).child("base-gpui"))
                                .child(
                                    div().text_sm().text_color(theme::text_muted()).child(
                                        "A GPUI-native port of Base UI's headless component \
                                         APIs for Rust.",
                                    ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme::text_muted())
                                        .child("Rust · headless · pre-1.0"),
                                ),
                        ),
                ),
            ),
        )
}
