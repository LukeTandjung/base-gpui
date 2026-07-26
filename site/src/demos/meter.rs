use base_gpui::meter::{MeterIndicator, MeterLabel, MeterRoot, MeterTrack, MeterValue};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    MeterRoot::new()
        .id("demo-meter")
        .value(24.)
        .min(0.)
        .max(100.)
        .aria_label("Storage used")
        .w(px(320.))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            MeterLabel::new()
                .text_sm()
                .text_color(theme::text())
                .child("Storage used"),
        )
        .child(
            MeterTrack::new()
                .w_full()
                .h(px(8.))
                .rounded_full()
                .bg(theme::control())
                .child(
                    MeterIndicator::new()
                        .h_full()
                        .rounded_full()
                        .bg(theme::accent()),
                ),
        )
        .child(
            MeterValue::new()
                .text_sm()
                .text_color(theme::text_muted()),
        )
}
