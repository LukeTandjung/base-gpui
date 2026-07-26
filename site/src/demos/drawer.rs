use base_gpui::drawer::{
    DrawerBackdrop, DrawerContent, DrawerPopup, DrawerPortal, DrawerRoot, DrawerTrigger,
    DrawerViewport,
};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

pub fn render() -> impl IntoElement {
    DrawerRoot::<()>::new()
        .id("demo-drawer")
        .child(
            DrawerTrigger::new()
                .id("demo-drawer-trigger")
                .aria_label("Open drawer")
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
                    trigger.hover(|style| style.bg(theme::accent_hover()))
                })
                .child("Open drawer"),
        )
        .child(
            DrawerPortal::new()
                .child(DrawerBackdrop::new().absolute().inset_0().bg(theme::surface()))
                .child(
                    DrawerViewport::new()
                        .id("demo-drawer-viewport")
                        .absolute()
                        .inset_0()
                        .flex()
                        .flex_col()
                        .justify_end()
                        .child(
                            DrawerPopup::new()
                                .id("demo-drawer-popup")
                                .aria_label("Session notes")
                                .w_full()
                                .rounded_t(px(12.))
                                .bg(theme::surface_solid())
                                .border_t_1()
                                .border_color(theme::border_strong())
                                .child(
                                    DrawerContent::new()
                                        .p(px(16.))
                                        .flex()
                                        .flex_col()
                                        .gap(px(8.))
                                        // Grab handle hinting at swipe-to-dismiss.
                                        .child(
                                            div().w_full().flex().justify_center().child(
                                                div()
                                                    .w(px(36.))
                                                    .h(px(4.))
                                                    .rounded_full()
                                                    .bg(theme::control()),
                                            ),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme::text())
                                                .child("Session notes"),
                                        )
                                        .child(
                                            div().text_sm().text_color(theme::text_muted()).child(
                                                "Swipe down or press Escape to dismiss this drawer.",
                                            ),
                                        ),
                                ),
                        ),
                ),
        )
}
