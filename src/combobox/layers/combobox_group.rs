use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxContext, ComboboxGroupChild, ComboboxGroupStyleState,
};

#[derive(IntoElement)]
pub struct ComboboxGroup<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxGroupChild<T>>,
    context: Option<ComboboxContext<T>>,
    index: Option<usize>,
    style_with_state: Option<Rc<dyn Fn(ComboboxGroupStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxGroup<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            index: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxGroup<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxGroup<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let item_count = self
            .children
            .iter()
            .filter(|child| matches!(child, ComboboxGroupChild::Item(_)))
            .count();
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.group_state(self.index, item_count))
            })
            .unwrap_or_else(|| ComboboxGroupStyleState::new(item_count, self.index, None));
        // AccessKit: the literal-string label substitutes for Base UI's
        // `aria-labelledby` group-label wiring, which has no gpui builder.
        let group_label = state.label.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let mut group = base
            .id(("combobox-group", self.index.unwrap_or(0)))
            .role(Role::Group);
        if let Some(group_label) = group_label {
            group = group.aria_label(group_label);
        }

        group
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            )
            .into_any_element()
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxGroup<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_combobox_context(context.clone()))
            .collect();
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let group_index = wiring.begin_group();
        self.children = wiring.wire_group_children(self.children, window, cx);
        wiring.end_group();
        self.index = Some(group_index);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxGroup<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxGroupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxGroupChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
