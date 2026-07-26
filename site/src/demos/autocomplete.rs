use base_gpui::autocomplete::{
    AutocompleteEmpty, AutocompleteInput, AutocompleteItem, AutocompleteList, AutocompletePopup,
    AutocompletePortal, AutocompletePositioner, AutocompleteRoot,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(value: &'static str, label: &'static str) -> AutocompleteItem<&'static str> {
    AutocompleteItem::new()
        .id(format!("demo-autocomplete-{value}"))
        .value(value)
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
        .child_any(label)
}

pub fn render() -> impl IntoElement {
    AutocompleteRoot::<&'static str>::new()
        .id("demo-autocomplete")
        .w(px(220.))
        .child(
            AutocompleteInput::new()
                .id("demo-autocomplete-input")
                .placeholder("Search components…")
                .aria_label("Search components")
                .w_full()
                .px(px(10.))
                .py(px(7.))
                .rounded(px(6.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .text_sm()
                .text_color(theme::text()),
        )
        .child(
            AutocompletePortal::new().child(
                AutocompletePositioner::new().side_offset(px(6.)).child(
                    AutocompletePopup::new()
                        .w(px(220.))
                        .p(px(4.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(
                            AutocompleteList::new()
                                .flex()
                                .flex_col()
                                .gap(px(2.))
                                .child(item("accordion", "Accordion"))
                                .child(item("button", "Button"))
                                .child(item("menu", "Menu"))
                                .child(item("select", "Select"))
                                .child(item("tabs", "Tabs")),
                        )
                        .child(
                            AutocompleteEmpty::new()
                                .px(px(10.))
                                .py(px(6.))
                                .text_sm()
                                .text_color(theme::text_muted())
                                .child("No matches"),
                        ),
                ),
            ),
        )
}
