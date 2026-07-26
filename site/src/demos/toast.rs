use base_gpui::button::ButtonRoot;
use base_gpui::toast::{
    create_toast_manager, ToastClose, ToastDescription, ToastOptions, ToastPortal, ToastProvider,
    ToastRoot, ToastTitle, ToastViewport,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    let manager = create_toast_manager::<()>();
    let add = manager.clone();

    ToastProvider::<()>::new()
        .id("demo-toast")
        .manager(manager)
        .child_any(
            ButtonRoot::new()
                .id("demo-toast-trigger")
                .aria_label("Show toast")
                .px(px(14.))
                .py(px(8.))
                .rounded(px(6.))
                .bg(theme::accent())
                .text_color(theme::text_inverted())
                .text_sm()
                .style_with_state(|state, root| {
                    let root = if state.focused {
                        root.border_2().border_color(theme::focus_ring())
                    } else {
                        root
                    };
                    root.hover(|style| style.bg(theme::accent_hover()))
                })
                .on_click(move |_, _, cx| {
                    add.add(
                        ToastOptions::new()
                            .title("Draft saved")
                            .description("All changes synced to your workspace."),
                        cx,
                    );
                })
                .child("Show toast"),
        )
        .child(
            ToastPortal::new().child(
                ToastViewport::new()
                    .id("demo-toast-viewport")
                    .absolute()
                    .bottom(px(16.))
                    .right(px(16.))
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .content_builder(|_facts| {
                        ToastRoot::new()
                            .w(px(260.))
                            .p(px(12.))
                            .rounded(px(8.))
                            .bg(theme::surface_solid())
                            .border_1()
                            .border_color(theme::border_strong())
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .child(ToastTitle::new().text_sm().text_color(theme::text()))
                            .child(
                                ToastDescription::new()
                                    .text_sm()
                                    .text_color(theme::text_muted()),
                            )
                            .child(
                                ToastClose::new()
                                    .aria_label("Close")
                                    .mt(px(4.))
                                    .px(px(10.))
                                    .py(px(4.))
                                    .rounded(px(6.))
                                    .bg(theme::control())
                                    .text_sm()
                                    .text_color(theme::text())
                                    .style_with_state(|_, close| {
                                        close.hover(|style| style.bg(theme::border_strong()))
                                    })
                                    .child_any("Dismiss"),
                            )
                    }),
            ),
        )
}
