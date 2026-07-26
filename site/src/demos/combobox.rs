use base_gpui::combobox::{
    ComboboxEmpty, ComboboxIcon, ComboboxInput, ComboboxInputGroup, ComboboxItem,
    ComboboxItemIndicator, ComboboxList, ComboboxPopup, ComboboxPortal, ComboboxPositioner,
    ComboboxRoot, ComboboxTrigger,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(value: &'static str, label: &'static str) -> ComboboxItem<&'static str> {
    ComboboxItem::new()
        .id(format!("demo-combobox-{value}"))
        .value(value)
        .label(label)
        .px(px(8.))
        .py(px(6.))
        .rounded(px(4.))
        .flex()
        .items_center()
        .gap(px(6.))
        .text_sm()
        .text_color(theme::text())
        .style_with_state(|state, item| {
            if state.highlighted {
                item.bg(theme::control())
            } else {
                item
            }
        })
        .child(
            ComboboxItemIndicator::new()
                .keep_mounted(true)
                .w(px(14.))
                .style_with_state(|state, indicator| {
                    if state.selected {
                        indicator
                    } else {
                        indicator.invisible()
                    }
                }),
        )
        .child_any(label)
}

pub fn render() -> impl IntoElement {
    ComboboxRoot::<&'static str>::new()
        .id("demo-combobox")
        .item_to_string_value(|value| (*value).into())
        .w(px(220.))
        .child(
            ComboboxInputGroup::new()
                .w_full()
                .px(px(10.))
                .py(px(7.))
                .rounded(px(6.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .flex()
                .items_center()
                .gap(px(6.))
                .child(
                    ComboboxInput::new()
                        .id("demo-combobox-input")
                        .placeholder("Search fruits…")
                        .aria_label("Fruits")
                        .flex_1()
                        .text_sm()
                        .text_color(theme::text()),
                )
                .child(
                    ComboboxTrigger::new().id("demo-combobox-trigger").child(
                        ComboboxIcon::<&'static str>::new()
                            .text_sm()
                            .text_color(theme::text_muted())
                            .child("↓"),
                    ),
                ),
        )
        .child(
            ComboboxPortal::new().child(
                ComboboxPositioner::new().side_offset(px(6.)).child(
                    ComboboxPopup::new()
                        .w(px(220.))
                        .p(px(4.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(
                            ComboboxList::new()
                                .flex()
                                .flex_col()
                                .gap(px(2.))
                                .child(item("apple", "Apple"))
                                .child(item("banana", "Banana"))
                                .child(item("cherry", "Cherry"))
                                .child(item("orange", "Orange")),
                        )
                        .child(
                            ComboboxEmpty::new()
                                .px(px(8.))
                                .py(px(6.))
                                .text_sm()
                                .text_color(theme::text_muted())
                                .child("No fruits found"),
                        ),
                ),
            ),
        )
}
