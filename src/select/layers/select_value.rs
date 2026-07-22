use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Text, Window,
};

type SelectValueStyle<T> = Rc<dyn Fn(SelectValueStyleState<T>, Div) -> Div + 'static>;

use crate::select::{
    child_wiring::SelectChildNode, SelectContext, SelectSelectionMode, SelectValueStyleState,
};

#[derive(IntoElement)]
pub struct SelectValue<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SelectContext<T>>,
    placeholder: Option<SharedString>,
    style_with_state: Option<SelectValueStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for SelectValue<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            placeholder: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for SelectValue<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectValue<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectValue<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.value_state(props, self.placeholder.clone())
                })
            })
            .unwrap_or_else(|| {
                SelectValueStyleState::new(
                    SelectSelectionMode::Single,
                    None,
                    Vec::new(),
                    Vec::new(),
                    true,
                    self.placeholder.clone().unwrap_or_default(),
                )
            });
        let display_text = state.display_text.clone();
        let has_custom_children = !self.children.is_empty();
        let measure_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // The trigger carries the accessible name and expanded state, so the
        // visible value text stays out of the a11y tree to avoid
        // double-announcing. Value announcement on change has no live-region
        // API in this gpui revision.
        let value = if has_custom_children {
            base.children(self.children)
        } else {
            base.child(Text::new_inaccessible(display_text))
        };

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(context) = measure_context.as_ref() else {
                    return;
                };
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if context.record_value_bounds(bounds, cx) {
                    window.request_animation_frame();
                }
            })
            .child(value)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectValue<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectValue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectValueStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
