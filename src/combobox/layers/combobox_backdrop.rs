use std::rc::Rc;

use gpui::{
    div, App, ClickEvent, Div, InteractiveElement as _, IntoElement, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxBackdropStyleState, ComboboxChangeReason,
    ComboboxChangeSource, ComboboxContext,
};

/// Combobox-local equivalent of `popover_backdrop.rs`.
#[derive(IntoElement)]
pub struct ComboboxBackdrop<T: Clone + Eq + 'static> {
    base: Div,
    context: Option<ComboboxContext<T>>,
    force_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(ComboboxBackdropStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxBackdrop<T> {
    fn default() -> Self {
        Self {
            base: div(),
            context: None,
            force_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxBackdrop<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxBackdrop<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.backdrop_state(self.force_mounted))
            })
            .unwrap_or_else(|| ComboboxBackdropStyleState::new(false, self.force_mounted));
        if !state.mounted {
            return div().into_any_element();
        }
        let context = self.context;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id("combobox-backdrop")
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }
                if let Some(context) = context.as_ref() {
                    context.set_open(
                        false,
                        ComboboxChangeReason::OutsidePress,
                        ComboboxChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .into_any_element()
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxBackdrop<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxBackdrop<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn force_mounted(mut self, force_mounted: bool) -> Self {
        self.force_mounted = force_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxBackdropStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
