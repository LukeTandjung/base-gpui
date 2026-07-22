use gpui::{App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::separator::{Separator, SeparatorOrientation, SeparatorStyleState};

/// Backed by the shared `base_gpui::separator::Separator`.
#[derive(IntoElement)]
pub struct ComboboxSeparator {
    separator: Separator,
}

impl Default for ComboboxSeparator {
    fn default() -> Self {
        Self {
            separator: Separator::new(),
        }
    }
}

impl Styled for ComboboxSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.separator.style()
    }
}

impl RenderOnce for ComboboxSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.separator
    }
}

impl ComboboxSeparator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.separator = self.separator.orientation(orientation);
        self
    }

    pub fn horizontal(self) -> Self {
        self.orientation(SeparatorOrientation::Horizontal)
    }

    pub fn vertical(self) -> Self {
        self.orientation(SeparatorOrientation::Vertical)
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SeparatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.separator = self.separator.style_with_state(style);
        self
    }
}
