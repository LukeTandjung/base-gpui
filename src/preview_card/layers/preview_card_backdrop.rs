use std::rc::Rc;

use gpui::{div, App, Div, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::preview_card::{
    child_wiring::PreviewCardChildNode, PreviewCardBackdropStyleState, PreviewCardContext,
};

/// Presentation-only backdrop: it never captures pointer events, never closes
/// the card, and never blocks content beneath it. Outside-press dismissal is
/// handled by the positioner's `on_mouse_down_out` runtime path — this is the
/// deliberate adaptation from Popover's click-capturing backdrop.
///
/// Base UI marks the backdrop `role="presentation"`. This gpui revision has
/// no presentation role builder (`Role::GenericContainer` is filtered), so
/// the equivalent is achieved by assigning no role: a role-less element never
/// enters the AccessKit tree.
#[derive(IntoElement)]
pub struct PreviewCardBackdrop<P: Clone + 'static = ()> {
    base: Div,
    context: Option<PreviewCardContext<P>>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(PreviewCardBackdropStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PreviewCardBackdrop<P> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PreviewCardBackdrop<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardBackdrop<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.backdrop_state(self.keep_mounted)))
            .unwrap_or_else(|| PreviewCardBackdropStyleState::new(false, self.keep_mounted));
        if !state.mounted {
            return div();
        }

        match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardBackdrop<P> {
    fn with_preview_card_context(mut self, context: PreviewCardContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> PreviewCardBackdrop<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PreviewCardBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
