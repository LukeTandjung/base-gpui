use base_gpui::accordion::{
    AccordionHeader, AccordionItem, AccordionPanel, AccordionRoot, AccordionTrigger,
};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn item(value: &'static str, question: &'static str, answer: &'static str) -> AccordionItem<&'static str> {
    AccordionItem::new(value)
        .id(value)
        .child(
            AccordionHeader::new().child(
                AccordionTrigger::new()
                    .w_full()
                    .flex()
                    .justify_between()
                    .py(px(10.))
                    .text_sm()
                    .text_color(theme::text())
                    .style_with_state(|state, trigger| {
                        if state.panel_open {
                            trigger.text_color(theme::accent())
                        } else {
                            trigger
                        }
                    })
                    .child(question),
            ),
        )
        .child(
            AccordionPanel::new()
                .pb(px(12.))
                .text_sm()
                .text_color(theme::text_muted())
                .child(answer),
        )
}

pub fn render() -> impl IntoElement {
    AccordionRoot::<&'static str>::new()
        .id("demo-accordion")
        .default_value(vec!["what"])
        .w(px(360.))
        .child(item(
            "what",
            "What is base-gpui?",
            "A GPUI-native port of Base UI's headless component APIs for Rust.",
        ))
        .child(item(
            "styling",
            "How do I style parts?",
            "Every part accepts standard GPUI styling plus style_with_state for state-driven styles.",
        ))
        .child(item(
            "a11y",
            "Is it accessible?",
            "Parts expose AccessKit roles and keyboard interactions out of the box.",
        ))
}
