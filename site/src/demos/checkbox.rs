use base_gpui::checkbox::{CheckboxIndicator, CheckboxRoot};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    CheckboxRoot::new()
        .id("demo-checkbox")
        .aria_label("Accept terms")
        .default_checked(true)
        .w(px(20.))
        .h(px(20.))
        .rounded(px(4.))
        .border_1()
        .flex()
        .items_center()
        .justify_center()
        .style_with_state(|state, root| {
            let root = if state.checked {
                root.bg(theme::accent()).border_color(theme::accent())
            } else {
                root.bg(theme::background()).border_color(theme::border_strong())
            };
            if state.focused {
                root.border_color(theme::focus_ring())
            } else {
                root
            }
        })
        .child(
            CheckboxIndicator::new().child(
                gpui::div()
                    .text_size(px(13.))
                    .text_color(theme::text_inverted())
                    .child("✓"),
            ),
        )
}
