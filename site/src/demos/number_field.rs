use base_gpui::number_field::{
    NumberFieldDecrement, NumberFieldGroup, NumberFieldIncrement, NumberFieldInput, NumberFieldRoot,
};
use gpui::prelude::*;
use gpui::{px, SharedString, Text};

use crate::theme;

pub fn render() -> impl IntoElement {
    NumberFieldRoot::new()
        .id("demo-number-field")
        .name("quantity")
        .default_value(Some(5.))
        .min(0.)
        .max(10.)
        .step(1.)
        .child(
            NumberFieldGroup::new()
                .id("demo-number-field-group")
                .flex()
                .items_center()
                .rounded(px(6.))
                .bg(theme::surface())
                .border_1()
                .border_color(theme::border_strong())
                .child(
                    NumberFieldDecrement::new()
                        .id("demo-number-field-decrement")
                        .w(px(32.))
                        .h(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .text_color(theme::text())
                        .style_with_state(|state, button| {
                            if state.can_decrement {
                                button.hover(|style| style.bg(theme::control()))
                            } else {
                                button.text_color(theme::text_muted())
                            }
                        })
                        // The part supplies the "Decrease" accessible label;
                        // keep the glyph out of the a11y tree.
                        .child(Text::new_inaccessible(SharedString::new_static("−"))),
                )
                .child(
                    NumberFieldInput::new()
                        .id("demo-number-field-input")
                        .w(px(56.))
                        .h(px(32.))
                        // The inner text element is line-height tall and sits
                        // at the top of the box; center it against the 32px
                        // input height.
                        .flex()
                        .items_center()
                        .text_sm()
                        .text_center()
                        .text_color(theme::text())
                        .border_l_1()
                        .border_r_1()
                        .border_color(theme::border())
                        .style_with_state(|state, input| {
                            if state.root.focused {
                                input.border_color(theme::focus_ring())
                            } else {
                                input
                            }
                        }),
                )
                .child(
                    NumberFieldIncrement::new()
                        .id("demo-number-field-increment")
                        .w(px(32.))
                        .h(px(32.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .text_color(theme::text())
                        .style_with_state(|state, button| {
                            if state.can_increment {
                                button.hover(|style| style.bg(theme::control()))
                            } else {
                                button.text_color(theme::text_muted())
                            }
                        })
                        .child(Text::new_inaccessible(SharedString::new_static("+"))),
                ),
        )
}
