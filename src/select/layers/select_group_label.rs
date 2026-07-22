use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Text, Window,
};

use crate::select::{
    child_wiring::{SelectChildNode, SelectChildWiring},
    SelectContext, SelectGroupLabelStyleState,
};

#[derive(IntoElement)]
pub struct SelectGroupLabel<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SelectContext<T>>,
    label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(SelectGroupLabelStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectGroupLabel<T> {
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

impl<T: Clone + Eq + 'static> ParentElement for SelectGroupLabel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectGroupLabel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectGroupLabel<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(SelectGroupLabelStyleState, self.base),
            None => self.base,
        };

        // The registered label text is mirrored into the parent SelectGroup's
        // `.aria_label(...)`, so the visible text stays out of the a11y tree.
        if self.children.is_empty() {
            if let Some(label) = self.label {
                base.child(Text::new_inaccessible(label))
            } else {
                base
            }
        } else {
            base.children(self.children)
        }
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectGroupLabel<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_select_child(
        self,
        wiring: &mut SelectChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        wiring.register_group_label(self.registration_label());
        self
    }
}

impl<T: Clone + Eq + 'static> SelectGroupLabel<T> {
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

    pub fn registration_label(&self) -> Option<SharedString> {
        self.label.clone()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectGroupLabelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
