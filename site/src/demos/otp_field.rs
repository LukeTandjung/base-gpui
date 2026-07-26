use base_gpui::otp_field::{OTPFieldInput, OTPFieldRoot};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn slot() -> OTPFieldInput {
    // Slot indices are wired by the root in render order.
    OTPFieldInput::new()
        .w(px(36.))
        .h(px(40.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(6.))
        .text_sm()
        .text_color(theme::text())
        .bg(theme::surface())
        .border_1()
        .border_color(theme::border_strong())
        .style_with_state(|state, slot| {
            if state.active {
                slot.border_color(theme::focus_ring())
            } else {
                slot
            }
        })
}

pub fn render() -> impl IntoElement {
    OTPFieldRoot::new()
        .id("demo-otp-field")
        .name("code")
        .aria_label("One-time code")
        .length(6)
        .flex()
        .gap(px(8.))
        .children((0..6).map(|_| slot()))
}
