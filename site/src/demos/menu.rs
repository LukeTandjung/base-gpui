use base_gpui::menu::{MenuItem, MenuPopup, MenuPortal, MenuPositioner, MenuRoot, MenuTrigger};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(slug: &'static str, label: &'static str) -> MenuItem {
    MenuItem::new()
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
    MenuRoot::<()>::new()
        .id("demo-menu")
        .child(
            MenuTrigger::new()
                .id("demo-menu-trigger")
                .aria_label("Edit")
                .px(px(14.))
                .py(px(8.))
                .rounded(px(6.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .text_sm()
                .text_color(theme::text())
                .style_with_state(|state, trigger| {
                    let trigger = if state.open {
                        trigger.bg(theme::control())
                    } else {
                        trigger.hover(|style| style.bg(theme::control()))
                    };
                    if state.focused {
                        trigger.border_color(theme::focus_ring())
                    } else {
                        trigger
                    }
                })
                .child("Edit"),
        )
        .child(
            MenuPortal::new().child(
                MenuPositioner::new().side_offset(px(6.)).child(
                    MenuPopup::new()
                        .id("demo-menu-popup")
                        .w(px(160.))
                        .p(px(4.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(item("demo-menu-cut", "Cut"))
                        .child(item("demo-menu-copy", "Copy"))
                        .child(item("demo-menu-paste", "Paste")),
                ),
            ),
        )
}
