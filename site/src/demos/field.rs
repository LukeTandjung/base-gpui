use base_gpui::field::{FieldControl, FieldDescription, FieldLabel, FieldRoot};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    FieldRoot::new()
        .id("demo-field")
        .name("username")
        .flex()
        .flex_col()
        .gap(px(6.))
        .w(px(260.))
        .child(
            FieldLabel::new()
                .text("Username")
                .text_sm()
                .text_color(theme::text()),
        )
        .child(
            FieldControl::new()
                .id("demo-field-control")
                .placeholder("e.g. ada")
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
        )
        .child(
            FieldDescription::new()
                .text_xs()
                .text_color(theme::text_muted())
                .child("Visible on your public profile."),
        )
}
