use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement, IntoElement, Orientation, RenderOnce, Role,
    StatefulInteractiveElement, StyleRefinement, Styled, Window,
};

use crate::separator::{SeparatorOrientation, SeparatorStyleState};

#[derive(IntoElement)]
pub struct Separator {
    id: ElementId,
    base: Div,
    orientation: SeparatorOrientation,
    style_with_state: Option<Rc<dyn Fn(SeparatorStyleState, Div) -> Div + 'static>>,
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            id: ElementId::from("separator"),
            base: div(),
            orientation: SeparatorOrientation::default(),
            style_with_state: None,
        }
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let state = SeparatorStyleState::new(self.orientation);

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // Applied after `style_with_state` so the styling closure cannot drop
        // the accessibility wiring. The stable id plus `Role::Separator` put
        // the divider in the a11y tree; `aria_orientation` mirrors Base UI's
        // `aria-orientation`. Separator stays non-focusable and inert.
        // accesskit 0.24 names the separator role `Role::Splitter` (there is
        // no `Role::Separator` variant).
        base.id(self.id.clone())
            .role(Role::Splitter)
            .aria_orientation(match self.orientation {
                SeparatorOrientation::Horizontal => Orientation::Horizontal,
                SeparatorOrientation::Vertical => Orientation::Vertical,
            })
    }
}

impl Separator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the default `"separator"` element id. Give each separator in
    /// a window a distinct, stable id so assistive technology sees stable
    /// accessibility nodes across frames.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
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
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
