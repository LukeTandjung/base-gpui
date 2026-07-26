use base_gpui::menu::{MenuItem, MenuPopup, MenuPortal, MenuPositioner, MenuRoot, MenuTrigger};
use base_gpui::menubar::Menubar;
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

fn menu(slug: &'static str, label: &'static str, items: Vec<MenuItem>) -> MenuRoot<()> {
    let mut popup = MenuPopup::new()
        .id(format!("{slug}-popup"))
        .w(px(150.))
        .p(px(4.))
        .rounded(px(8.))
        .bg(theme::surface_solid())
        .border_1()
        .border_color(theme::border_strong());
    for entry in items {
        popup = popup.child(entry);
    }

    MenuRoot::<()>::new()
        .id(slug)
        .child(
            MenuTrigger::new()
                .id(format!("{slug}-trigger"))
                .aria_label(label)
                .px(px(12.))
                .py(px(6.))
                .rounded(px(5.))
                .text_sm()
                .text_color(theme::text_muted())
                .style_with_state(|state, trigger| {
                    if state.open {
                        trigger.bg(theme::control()).text_color(theme::text())
                    } else {
                        trigger.hover(|style| style.bg(theme::control()))
                    }
                })
                .child(label),
        )
        .child(MenuPortal::new().child(MenuPositioner::new().side_offset(px(6.)).child(popup)))
}

pub fn render() -> impl IntoElement {
    Menubar::new()
        .id("demo-menubar")
        .aria_label("Application menu")
        .flex()
        .gap(px(2.))
        .p(px(3.))
        .rounded(px(8.))
        .bg(theme::surface())
        .border_1()
        .border_color(theme::border())
        .child(menu(
            "demo-menubar-file",
            "File",
            vec![
                item("demo-menubar-new", "New File"),
                item("demo-menubar-open", "Open…"),
                item("demo-menubar-save", "Save"),
            ],
        ))
        .child(menu(
            "demo-menubar-edit",
            "Edit",
            vec![
                item("demo-menubar-undo", "Undo"),
                item("demo-menubar-redo", "Redo"),
            ],
        ))
}
