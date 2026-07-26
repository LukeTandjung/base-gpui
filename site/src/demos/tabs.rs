use base_gpui::tabs::{TabsList, TabsPanel, TabsRoot, TabsTab};
use gpui::prelude::*;
use gpui::px;

use crate::theme;

fn tab(slug: &'static str, label: &'static str) -> TabsTab<&'static str> {
    TabsTab::new()
        .id(slug)
        .value(slug)
        .px(px(12.))
        .py(px(6.))
        .rounded(px(6.))
        .text_sm()
        .style_with_state(|state, tab| {
            if state.active {
                tab.bg(theme::background()).text_color(theme::text())
            } else {
                tab.text_color(theme::text_muted())
            }
        })
        .child(label)
}

fn panel(slug: &'static str, body: &'static str) -> TabsPanel<&'static str> {
    TabsPanel::new()
        .id(format!("{slug}-panel"))
        .value(slug)
        .pt(px(12.))
        .text_sm()
        .text_color(theme::text_muted())
        .child(body)
}

pub fn render() -> impl IntoElement {
    TabsRoot::<&'static str>::new()
        .id("demo-tabs")
        .default_value(Some("overview"))
        .w(px(360.))
        .child(
            TabsList::new()
                .aria_label("Project sections")
                .flex()
                .gap(px(4.))
                .p(px(4.))
                .rounded(px(8.))
                .bg(theme::control())
                .child(tab("overview", "Overview"))
                .child(tab("projects", "Projects"))
                .child(tab("account", "Account")),
        )
        .child(panel(
            "overview",
            "A quick summary of recent activity across the workspace.",
        ))
        .child(panel(
            "projects",
            "Browse every project you have access to, sorted by recency.",
        ))
        .child(panel(
            "account",
            "Manage profile details, security, and notification settings.",
        ))
}
