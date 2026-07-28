use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxContext, ComboboxGroupLabelStyleState,
};

#[derive(derive_setters::Setters, IntoElement)]
pub struct ComboboxGroupLabel<T: Clone + Eq + 'static> {
    #[setters(skip)]
    base: Div,
    #[setters(skip)]
    children: Vec<AnyElement>,
    #[setters(skip)]
    context: Option<ComboboxContext<T>>,
    #[setters(into, strip_option)]
    label: Option<SharedString>,
    #[setters(skip)]
    style_with_state: Option<Rc<dyn Fn(ComboboxGroupLabelStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxGroupLabel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxGroupLabel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxGroupLabel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxGroupLabel<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(ComboboxGroupLabelStyleState, self.base),
            None => self.base,
        };

        if self.children.is_empty() {
            if let Some(label) = self.label {
                base.child(label)
            } else {
                base
            }
        } else {
            base.children(self.children)
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxGroupLabel<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_combobox_child(
        self,
        wiring: &mut ComboboxChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        wiring.register_group_label(self.label.clone());
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxGroupLabel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(self, label: impl Into<SharedString>) -> Self {
        self.label(label)
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxGroupLabelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
