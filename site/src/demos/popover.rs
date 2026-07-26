use base_gpui::popover::{
    PopoverClose, PopoverDescription, PopoverPopup, PopoverPortal, PopoverPositioner, PopoverRoot,
    PopoverTitle, PopoverTrigger,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    PopoverRoot::<()>::new()
        .id("demo-popover")
        .child(
            PopoverTrigger::new()
                .id("demo-popover-trigger")
                .aria_label("Notifications")
                .px(px(14.))
                .py(px(8.))
                .rounded(px(6.))
                .bg(theme::accent())
                .text_color(theme::text_inverted())
                .text_sm()
                .style_with_state(|state, trigger| {
                    let trigger = if state.focused {
                        trigger.border_2().border_color(theme::focus_ring())
                    } else {
                        trigger
                    };
                    let trigger = if state.open {
                        trigger.bg(theme::accent_hover())
                    } else {
                        trigger
                    };
                    trigger.hover(|style| style.bg(theme::accent_hover()))
                })
                .child("Notifications"),
        )
        .child(
            PopoverPortal::new().child(
                PopoverPositioner::new().side_offset(px(8.)).child(
                    PopoverPopup::new()
                        .id("demo-popover-popup")
                        .aria_label("Notifications")
                        .w(px(280.))
                        .p(px(14.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .flex()
                        .flex_col()
                        .gap(px(6.))
                        .child(
                            PopoverTitle::new()
                                .text_sm()
                                .text_color(theme::text())
                                .child("Notifications"),
                        )
                        .child(
                            PopoverDescription::new()
                                .text_sm()
                                .text_color(theme::text_muted())
                                .child("You are all caught up. New activity shows up here."),
                        )
                        .child(
                            PopoverClose::new()
                                .id("demo-popover-close")
                                .aria_label("Close")
                                .mt(px(4.))
                                .px(px(12.))
                                .py(px(6.))
                                .rounded(px(6.))
                                .bg(theme::control())
                                .text_sm()
                                .text_color(theme::text())
                                .style_with_state(|_, close| {
                                    close.hover(|style| style.bg(theme::border_strong()))
                                })
                                .child("Close"),
                        ),
                ),
            ),
        )
}
