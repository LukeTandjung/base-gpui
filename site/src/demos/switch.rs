use base_gpui::switch::{SwitchRoot, SwitchThumb};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

pub fn render() -> impl IntoElement {
    SwitchRoot::new()
        .id("demo-switch")
        .aria_label("Enable notifications")
        .default_checked(false)
        .w(px(40.))
        .h(px(24.))
        .p(px(2.))
        .rounded_full()
        .style_with_state(|state, root| {
            root.bg(if state.checked {
                theme::accent()
            } else {
                theme::control()
            })
        })
        .child(
            SwitchThumb::new()
                .w(px(20.))
                .h(px(20.))
                .rounded_full()
                .bg(theme::background())
                .style_with_state(|state, thumb| {
                    if state.root.checked {
                        thumb.ml(px(16.))
                    } else {
                        thumb
                    }
                }),
        )
}
