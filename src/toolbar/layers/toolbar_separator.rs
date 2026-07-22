use std::rc::Rc;

use gpui::{App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::{
    separator::{Separator, SeparatorOrientation, SeparatorStyleState},
    toolbar::{ToolbarContext, ToolbarOrientation},
};

/// A toolbar separator wrapping the ported `Separator`. It is not a
/// composite item and is never focused by roving navigation. Without an
/// explicit orientation it renders perpendicular to the toolbar.
#[derive(IntoElement)]
pub struct ToolbarSeparator {
    inner: Separator,
    orientation: Option<SeparatorOrientation>,
    style_with_state: Option<Rc<dyn Fn(SeparatorStyleState, Div) -> Div + 'static>>,
    toolbar: Option<ToolbarContext>,
}

impl Default for ToolbarSeparator {
    fn default() -> Self {
        Self {
            inner: Separator::new(),
            orientation: None,
            style_with_state: None,
            toolbar: None,
        }
    }
}

impl Styled for ToolbarSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl RenderOnce for ToolbarSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let toolbar_orientation = self
            .toolbar
            .as_ref()
            .map(|context| context.read(cx, |_runtime, props| props.orientation()))
            .unwrap_or(ToolbarOrientation::Horizontal);
        let orientation = self.orientation.unwrap_or(match toolbar_orientation {
            ToolbarOrientation::Horizontal => SeparatorOrientation::Vertical,
            ToolbarOrientation::Vertical => SeparatorOrientation::Horizontal,
        });

        let mut inner = self.inner.orientation(orientation);

        if let Some(style_with_state) = self.style_with_state {
            inner = inner.style_with_state(move |state, base| style_with_state(state, base));
        }

        inner
    }
}

impl ToolbarSeparator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the derived perpendicular orientation.
    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SeparatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// Attaches the toolbar context so the default orientation can be
    /// derived. Called by the toolbar child wiring; not intended for direct
    /// use.
    pub fn with_toolbar(mut self, context: ToolbarContext) -> Self {
        self.toolbar = Some(context);
        self
    }
}
