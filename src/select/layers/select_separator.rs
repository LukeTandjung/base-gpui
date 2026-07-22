use gpui::{App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::separator::{Separator, SeparatorOrientation, SeparatorStyleState};

#[derive(IntoElement)]
pub struct SelectSeparator {
    separator: Separator,
}

impl Default for SelectSeparator {
    fn default() -> Self {
        Self {
            separator: Separator::new(),
        }
    }
}

impl Styled for SelectSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.separator.style()
    }
}

impl RenderOnce for SelectSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.separator
    }
}

impl SelectSeparator {
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
