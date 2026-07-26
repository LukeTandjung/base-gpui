use base_gpui::avatar::{AvatarFallback, AvatarRoot};
use gpui::prelude::*;
use gpui::{div, px};

use crate::theme;

// Remote images are blocked in the embedded canvas, so each avatar renders
// its initials fallback.
fn avatar(id: &'static str, initials: &'static str, name: &'static str) -> AvatarRoot {
    AvatarRoot::new()
        .id(id)
        .aria_label(name)
        .w(px(40.))
        .h(px(40.))
        .rounded_full()
        .border_1()
        .border_color(theme::border_strong())
        .bg(theme::control())
        .flex()
        .items_center()
        .justify_center()
        .child(
            AvatarFallback::new()
                .text_sm()
                .text_color(theme::text())
                .child(initials),
        )
}

pub fn render() -> impl IntoElement {
    div()
        .flex()
        .gap(px(12.))
        .child(avatar("demo-avatar-lt", "LT", "Luke Tandjung"))
        .child(avatar("demo-avatar-af", "AF", "Ada Fairbanks"))
        .child(avatar("demo-avatar-nk", "NK", "Noor Khalid"))
}
