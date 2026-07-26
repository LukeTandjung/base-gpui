use base_gpui::collapsible::{CollapsiblePanel, CollapsibleRoot, CollapsibleTrigger};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

pub fn render() -> impl IntoElement {
    CollapsibleRoot::new()
        .id("demo-collapsible")
        .default_open(true)
        .w(px(320.))
        .child(
            CollapsibleTrigger::new()
                .id("demo-collapsible-trigger")
                .w_full()
                .py(px(8.))
                .text_sm()
                .style_with_state(|state, trigger| {
                    if state.open {
                        trigger.text_color(theme::text())
                    } else {
                        trigger.text_color(theme::text_muted())
                    }
                })
                .child("Recovery keys"),
        )
        .child(
            CollapsiblePanel::new()
                .flex()
                .flex_col()
                .gap(px(4.))
                .pt(px(4.))
                .text_sm()
                .text_color(theme::text_muted())
                .children(
                    ["alpine-lunar-brick", "quiet-iris-cargo", "wild-ember-atlas"].map(|key| {
                        div()
                            .px(px(10.))
                            .py(px(6.))
                            .rounded(px(6.))
                            .bg(theme::surface())
                            .child(key)
                    }),
                ),
        )
}
