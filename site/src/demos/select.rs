use base_gpui::select::{
    SelectIcon, SelectItem, SelectItemIndicator, SelectItemText, SelectList, SelectPopup,
    SelectPortal, SelectPositioner, SelectRoot, SelectTrigger, SelectValue,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(value: &'static str, label: &'static str) -> SelectItem<&'static str> {
    SelectItem::new()
        .id(format!("demo-select-{value}"))
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
            SelectItemIndicator::new()
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
        .child(SelectItemText::new().text(label))
}

pub fn render() -> impl IntoElement {
    SelectRoot::<&'static str>::new()
        .id("demo-select")
        .default_value(Some("system"))
        .item_to_string_value(|value| (*value).into())
        .w(px(180.))
        .child(
            SelectTrigger::new()
                .id("demo-select-trigger")
                .aria_label("Theme")
                .w_full()
                .px(px(12.))
                .py(px(8.))
                .rounded(px(6.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .flex()
                .items_center()
                .justify_between()
                .text_sm()
                .text_color(theme::text())
                .style_with_state(|state, trigger| {
                    let trigger = if state.root.open {
                        trigger.bg(theme::control())
                    } else {
                        trigger.hover(|style| style.bg(theme::control()))
                    };
                    if state.root.focused {
                        trigger.border_color(theme::focus_ring())
                    } else {
                        trigger
                    }
                })
                .child(SelectValue::new().placeholder("Theme"))
                .child(SelectIcon::new().text_color(theme::text_muted()).child("↓")),
        )
        .child(
            SelectPortal::<&'static str>::new().child(
                SelectPositioner::new().side_offset(px(6.)).child(
                    SelectPopup::new()
                        .w(px(180.))
                        .p(px(4.))
                        .rounded(px(8.))
                        .bg(theme::surface_solid())
                        .border_1()
                        .border_color(theme::border_strong())
                        .child(
                            SelectList::new()
                                .flex()
                                .flex_col()
                                .gap(px(2.))
                                .child(item("system", "System"))
                                .child(item("light", "Light"))
                                .child(item("dark", "Dark")),
                        ),
                ),
            ),
        )
}
