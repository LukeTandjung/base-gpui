use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Text, Window,
};

use crate::select::{
    child_wiring::SelectChildNode, SelectContext, SelectItemStyleState, SelectItemTextStyleState,
};

#[derive(IntoElement)]
pub struct SelectItemText<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SelectContext<T>>,
    item_state: Option<SelectItemStyleState<T>>,
    label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(SelectItemTextStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectItemText<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            item_state: None,
            label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for SelectItemText<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectItemText<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectItemText<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = match self.item_state.as_ref() {
            Some(item_state) => self
                .context
                .as_ref()
                .map(|context| context.read(cx, |runtime, _| runtime.item_text_state(item_state)))
                .unwrap_or_else(|| {
                    SelectItemTextStyleState::new(
                        item_state.selected,
                        item_state.highlighted,
                        item_state.item_text_bounds,
                    )
                }),
            None => SelectItemTextStyleState::new(false, false, None),
        };
        let has_custom_children = !self.children.is_empty();
        let label = self.label.clone();
        let measure_context = self.context.clone();
        let measure_index = self.item_state.as_ref().and_then(|state| state.index);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // The parent SelectItem sets `.aria_label(...)` from the registered
        // label, so the visible item text stays out of the a11y tree.
        let text = if has_custom_children {
            base.children(self.children)
        } else if let Some(label) = label {
            base.child(Text::new_inaccessible(label))
        } else {
            base
        };

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(context) = measure_context.as_ref() else {
                    return;
                };
                let Some(index) = measure_index else {
                    return;
                };
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };

                if context.record_item_text_bounds(index, bounds, cx) {
                    window.request_animation_frame();
                }
            })
            .child(text)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectItemText<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectItemText<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn text(self, label: impl Into<SharedString>) -> Self {
        self.label(label)
    }

    pub fn with_item_state(mut self, state: SelectItemStyleState<T>) -> Self {
        self.item_state = Some(state);
        self
    }

    pub fn registration_label(&self) -> Option<SharedString> {
        self.label.clone()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectItemTextStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
