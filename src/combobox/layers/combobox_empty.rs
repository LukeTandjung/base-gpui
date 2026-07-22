use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::combobox::{child_wiring::ComboboxChildNode, ComboboxContext, ComboboxEmptyStyleState};

/// Renders its children only when the filtered list is empty.
#[derive(IntoElement)]
pub struct ComboboxEmpty<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    style_with_state: Option<Rc<dyn Fn(ComboboxEmptyStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxEmpty<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxEmpty<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxEmpty<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxEmpty<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.empty_state()))
            .unwrap_or_else(|| ComboboxEmptyStyleState::new(false));

        if !state.empty {
            return div();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxEmpty<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxEmpty<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxEmptyStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
