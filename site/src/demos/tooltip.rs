use std::time::Duration;

use base_gpui::tooltip::{
    TooltipPopup, TooltipPortal, TooltipPositioner, TooltipRoot, TooltipTrigger,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    TooltipRoot::<()>::new()
        .id("demo-tooltip")
        .child(
            TooltipTrigger::new()
                .id("demo-tooltip-trigger")
                .aria_label("Save document")
                .delay(Duration::from_millis(200))
                .px(px(14.))
                .py(px(8.))
                .rounded(px(6.))
                .bg(theme::control())
                .text_color(theme::text())
                .text_sm()
                .style_with_state(|state, trigger| {
                    let trigger = if state.focused {
                        trigger.border_2().border_color(theme::focus_ring())
                    } else {
                        trigger
                    };
                    if state.open {
                        trigger.bg(theme::border_strong())
                    } else {
                        trigger
                    }
                })
                .child("Hover me"),
        )
        .child(
            TooltipPortal::new().child(
                TooltipPositioner::new().side_offset(px(6.)).child(
                    TooltipPopup::new()
                        .id("demo-tooltip-popup")
                        .px(px(10.))
                        .py(px(6.))
                        .rounded(px(6.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .text_sm()
                        .text_color(theme::text())
                        .child_any("Saves the current document"),
                ),
            ),
        )
}
