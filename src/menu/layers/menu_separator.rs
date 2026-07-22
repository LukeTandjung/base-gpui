use gpui::{App, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::separator::{Separator, SeparatorOrientation, SeparatorStyleState};

/// Thin wrapper over the shared `base_gpui::separator::Separator`. Separators
/// never consume item indices or participate in navigation.
#[derive(IntoElement)]
pub struct MenuSeparator {
    inner: Separator,
}

impl Default for MenuSeparator {
    fn default() -> Self {
        Self {
            inner: Separator::new(),
        }
    }
}

impl Styled for MenuSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl RenderOnce for MenuSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.inner
    }
}

impl MenuSeparator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.inner = self.inner.orientation(orientation);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SeparatorStyleState, gpui::Div) -> gpui::Div + 'static,
    ) -> Self {
        self.inner = self.inner.style_with_state(style);
        self
    }
}
