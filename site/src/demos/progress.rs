use base_gpui::progress::{ProgressIndicator, ProgressLabel, ProgressRoot, ProgressTrack, ProgressValue};
use gpui::prelude::*;
use gpui::{px, relative};

use crate::theme;

pub fn render() -> impl IntoElement {
    ProgressRoot::new()
        .id("demo-progress")
        .value(Some(65.))
        .label("Export data")
        .w(px(320.))
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            ProgressLabel::new()
                .text_sm()
                .text_color(theme::text())
                .child("Export data"),
        )
        .child(
            ProgressTrack::new()
                .w_full()
                .h(px(8.))
                .rounded_full()
                .bg(theme::control())
                .child(
                    ProgressIndicator::new()
                        .h_full()
                        .rounded_full()
                        .bg(theme::accent())
                        .style_with_state(|state, indicator| {
                            let fraction = state.percentage.unwrap_or(0.) / 100.;
                            indicator.w(relative(fraction as f32))
                        }),
                ),
        )
        .child(
            ProgressValue::new()
                .text_sm()
                .text_color(theme::text_muted()),
        )
}
