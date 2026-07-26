use base_gpui::context_menu::{
    ContextMenuItem, ContextMenuPopup, ContextMenuPortal, ContextMenuPositioner, ContextMenuRoot,
    ContextMenuTrigger,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(slug: &'static str, label: &'static str) -> ContextMenuItem {
    ContextMenuItem::new()
        .id(slug)
        .label(label)
        .px(px(10.))
        .py(px(6.))
        .rounded(px(4.))
        .text_sm()
        .text_color(theme::text())
        .style_with_state(|state, item| {
            if state.highlighted {
                item.bg(theme::control())
            } else {
                item
            }
        })
        .child(label)
}

pub fn render() -> impl IntoElement {
    ContextMenuRoot::<()>::new()
        .id("demo-context-menu")
        .child(
            ContextMenuTrigger::new()
                .id("demo-context-menu-trigger")
                .w(px(280.))
                .h(px(150.))
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(8.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .text_sm()
                .text_color(theme::text_muted())
                .style_with_state(|state, area| {
                    if state.open {
                        area.border_color(theme::focus_ring())
                    } else {
                        area
                    }
                })
                .child("Right-click here"),
        )
        .child(
            ContextMenuPortal::new().child(
                ContextMenuPositioner::new().child(
                    ContextMenuPopup::new()
                        .id("demo-context-menu-popup")
                        .w(px(150.))
                        .p(px(4.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(item("demo-context-menu-back", "Back"))
                        .child(item("demo-context-menu-forward", "Forward"))
                        .child(item("demo-context-menu-reload", "Reload")),
                ),
            ),
        )
}
