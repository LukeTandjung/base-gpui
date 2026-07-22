use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxContext, ComboboxItemIndicatorStyleState,
    ComboboxItemStyleState,
};

/// Renders only when its item is selected, unless `keep_mounted`.
#[derive(IntoElement)]
pub struct ComboboxItemIndicator<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    item_state: Option<ComboboxItemStyleState<T>>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(ComboboxItemIndicatorStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxItemIndicator<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            item_state: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxItemIndicator<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxItemIndicator<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxItemIndicator<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = match self.item_state.as_ref() {
            Some(item_state) => self
                .context
                .as_ref()
                .map(|context| {
                    context.read(cx, |runtime, _| {
                        runtime.item_indicator_state(item_state, self.keep_mounted)
                    })
                })
                .unwrap_or_else(|| {
                    ComboboxItemIndicatorStyleState::new(
                        item_state.selected,
                        item_state.selected || self.keep_mounted,
                    )
                }),
            None => ComboboxItemIndicatorStyleState::new(false, self.keep_mounted),
        };

        if !state.present {
            return div();
        }

        let has_custom_children = !self.children.is_empty();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        if has_custom_children {
            base.children(self.children)
        } else {
            base.child("✓")
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxItemIndicator<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxItemIndicator<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn with_item_state(mut self, state: ComboboxItemStyleState<T>) -> Self {
        self.item_state = Some(state);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxItemIndicatorStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
