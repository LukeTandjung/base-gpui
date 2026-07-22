use std::rc::Rc;

use gpui::{
    div, AnyElement, App, ClickEvent, Div, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::combobox::{child_wiring::ComboboxChildNode, ComboboxContext, ComboboxLabelStyleState};

/// Focuses the input without opening the popup when pressed.
#[derive(IntoElement)]
pub struct ComboboxLabel<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    style_with_state: Option<Rc<dyn Fn(ComboboxLabelStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxLabel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxLabel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxLabel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxLabel<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self.context;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(ComboboxLabelStyleState, self.base),
            None => self.base,
        };

        base.id("combobox-label")
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }
                if let Some(context) = context.as_ref() {
                    context.focus_input(window, cx);
                }
            })
            .children(self.children)
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxLabel<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxLabel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxLabelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
