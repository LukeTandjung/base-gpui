use base_gpui::button::ButtonRoot;
use base_gpui::field::{FieldControl, FieldLabel, FieldRoot};
use base_gpui::form::{Form, FormSubmitAction};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    Form::new()
        .id("demo-form")
        .aria_label("Subscribe")
        .flex()
        .flex_col()
        .gap(px(10.))
        .w(px(260.))
        .child(
            FieldRoot::new()
                .id("demo-form-field")
                .name("email")
                .flex()
                .flex_col()
                .gap(px(4.))
                .child(
                    FieldLabel::new()
                        .text("Email")
                        .text_sm()
                        .text_color(theme::text()),
                )
                .child(
                    FieldControl::new()
                        .id("demo-form-control")
                        .required(true)
                        .placeholder("you@example.com")
                        .px(px(10.))
                        .py(px(8.))
                        .rounded(px(6.))
                        .text_sm()
                        .text_color(theme::text())
                        .bg(theme::surface())
                        .border_1()
                        .border_color(theme::border_strong())
                        .style_with_state(|state, control| {
                            if state.focused {
                                control.border_color(theme::focus_ring())
                            } else {
                                control
                            }
                        }),
                ),
        )
        .child(
            ButtonRoot::new()
                .id("demo-form-submit")
                .aria_label("Subscribe")
                .px(px(14.))
                .py(px(8.))
                .rounded(px(6.))
                .bg(theme::accent())
                .text_sm()
                .text_color(theme::text_inverted())
                .style_with_state(|state, button| {
                    let button = if state.focused {
                        button.border_2().border_color(theme::focus_ring())
                    } else {
                        button
                    };
                    button.hover(|style| style.bg(theme::accent_hover()))
                })
                // Submission is action-driven: the click dispatches the
                // action that the Form's key context handles.
                .on_click(|_, window, cx| window.dispatch_action(Box::new(FormSubmitAction), cx))
                .child("Subscribe"),
        )
}
