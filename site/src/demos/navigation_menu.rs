use base_gpui::navigation_menu::{
    NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList,
    NavigationMenuPopup, NavigationMenuPortal, NavigationMenuPositioner, NavigationMenuRoot,
    NavigationMenuTrigger, NavigationMenuViewport,
};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

fn content_entry(label: &'static str) -> impl IntoElement {
    div()
        .px(px(10.))
        .py(px(6.))
        .rounded(px(4.))
        .text_sm()
        .text_color(theme::text_muted())
        .hover(|style| style.bg(theme::control()).text_color(theme::text()))
        .child(label)
}

fn link(label: &'static str) -> NavigationMenuLink<&'static str> {
    NavigationMenuLink::new()
        .px(px(12.))
        .py(px(6.))
        .rounded(px(5.))
        .text_sm()
        .text_color(theme::text_muted())
        .style_with_state(|_, link| link.hover(|style| style.bg(theme::control())))
        .child(label)
}

pub fn render() -> impl IntoElement {
    NavigationMenuRoot::<&'static str>::new()
        .id("demo-navigation-menu")
        .aria_label("Main navigation")
        .child(
            NavigationMenuList::new()
                .flex()
                .gap(px(2.))
                .p(px(3.))
                .rounded(px(8.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border())
                .child(
                    NavigationMenuItem::new()
                        .value("docs")
                        .child(
                            NavigationMenuTrigger::new()
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
                                .child_any("Docs"),
                        )
                        .child(
                            NavigationMenuContent::new()
                                .flex()
                                .flex_col()
                                .gap(px(2.))
                                .w(px(180.))
                                .child(content_entry("Getting started"))
                                .child(content_entry("Components"))
                                .child(content_entry("Theming")),
                        ),
                )
                .child(link("Blog"))
                .child(link("About")),
        )
        .child(
            NavigationMenuPortal::new().child(
                NavigationMenuPositioner::new().side_offset(px(6.)).child(
                    NavigationMenuPopup::new()
                        .p(px(4.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(NavigationMenuViewport::new()),
                ),
            ),
        )
}
