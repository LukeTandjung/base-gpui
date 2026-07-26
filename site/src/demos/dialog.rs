use base_gpui::dialog::{
    DialogBackdrop, DialogClose, DialogDescription, DialogPopup, DialogPortal, DialogRoot,
    DialogTitle, DialogTrigger, DialogViewport,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    DialogRoot::<()>::new()
        .id("demo-dialog")
        .child(
            DialogTrigger::new()
                .id("demo-dialog-trigger")
                .aria_label("Open dialog")
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
                .child("Open dialog"),
        )
        .child(
            DialogPortal::new()
                .child(DialogBackdrop::new().absolute().inset_0().bg(theme::surface()))
                .child(
                    DialogViewport::new()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            DialogPopup::new()
                                .id("demo-dialog-popup")
                                .aria_label("Share workspace")
                                .w(px(300.))
                                .p(px(16.))
                                .rounded(px(8.))
                                .bg(theme::surface_solid())
                                .border_1()
                                .border_color(theme::border_strong())
                                .flex()
                                .flex_col()
                                .gap(px(8.))
                                .child(
                                    DialogTitle::new()
                                        .text_sm()
                                        .text_color(theme::text())
                                        .child("Share workspace"),
                                )
                                .child(
                                    DialogDescription::new()
                                        .text_sm()
                                        .text_color(theme::text_muted())
                                        .child(
                                            "Invite teammates by email. They can view every \
                                             project in this workspace.",
                                        ),
                                )
                                .child(
                                    DialogClose::new()
                                        .id("demo-dialog-close")
                                        .aria_label("Close")
                                        .mt(px(4.))
                                        .px(px(12.))
                                        .py(px(6.))
                                        .rounded(px(6.))
                                        .bg(theme::control())
                                        .text_sm()
                                        .text_color(theme::text())
                                        .style_with_state(|state, close| {
                                            let close = if state.focused {
                                                close.border_2().border_color(theme::focus_ring())
                                            } else {
                                                close
                                            };
                                            close.hover(|style| style.bg(theme::border_strong()))
                                        })
                                        .child("Close"),
                                ),
                        ),
                ),
        )
}
