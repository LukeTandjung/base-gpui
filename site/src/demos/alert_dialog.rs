use base_gpui::alert_dialog::{
    AlertDialogBackdrop, AlertDialogClose, AlertDialogDescription, AlertDialogPopup,
    AlertDialogPortal, AlertDialogRoot, AlertDialogTitle, AlertDialogTrigger, AlertDialogViewport,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    AlertDialogRoot::<()>::new()
        .id("demo-alert-dialog")
        .child(
            AlertDialogTrigger::new()
                .id("demo-alert-dialog-trigger")
                .aria_label("Discard draft")
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
                    trigger.hover(|style| style.bg(theme::border_strong()))
                })
                .child("Discard draft"),
        )
        .child(
            AlertDialogPortal::new()
                .child(AlertDialogBackdrop::new().absolute().inset_0().bg(theme::surface()))
                .child(
                    AlertDialogViewport::new()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            AlertDialogPopup::new()
                                .id("demo-alert-dialog-popup")
                                .aria_label("Discard draft?")
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
                                    AlertDialogTitle::new()
                                        .text_sm()
                                        .text_color(theme::text())
                                        .child("Discard draft?"),
                                )
                                .child(
                                    AlertDialogDescription::new()
                                        .text_sm()
                                        .text_color(theme::text_muted())
                                        .child("The draft is deleted permanently. This cannot be undone."),
                                )
                                .child(
                                    AlertDialogClose::new()
                                        .id("demo-alert-dialog-cancel")
                                        .aria_label("Cancel")
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
                                        .child("Cancel"),
                                )
                                .child(
                                    AlertDialogClose::new()
                                        .id("demo-alert-dialog-discard")
                                        .aria_label("Discard")
                                        .px(px(12.))
                                        .py(px(6.))
                                        .rounded(px(6.))
                                        .bg(theme::accent())
                                        .text_sm()
                                        .text_color(theme::text_inverted())
                                        .style_with_state(|state, close| {
                                            let close = if state.focused {
                                                close.border_2().border_color(theme::focus_ring())
                                            } else {
                                                close
                                            };
                                            close.hover(|style| style.bg(theme::accent_hover()))
                                        })
                                        .child("Discard"),
                                ),
                        ),
                ),
        )
}
