use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{MenuChildNode, MenuChildWiring},
    MenuContext, MenuGroupLabelStyleState,
};

/// Group label. Label metadata is registered in the runtime for the AccessKit
/// follow-up; labels never consume item indices.
#[derive(IntoElement)]
pub struct MenuGroupLabel<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<MenuContext<P>>,
    label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(MenuGroupLabelStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuGroupLabel<P> {
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

impl<P: Clone + 'static> ParentElement for MenuGroupLabel<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for MenuGroupLabel<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuGroupLabel<P> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(MenuGroupLabelStyleState, self.base),
            None => self.base,
        };

        // The label string is surfaced through the parent group's
        // `.aria_label(...)`, so the visible text stays out of the a11y tree
        // (Base UI marks the label element `role="presentation"`). When no
        // explicit children are provided the label renders itself
        // inaccessibly; callers providing their own text child should pass it
        // as `Text::new_inaccessible(...)`.
        match (self.children.is_empty(), self.label) {
            (true, Some(label)) => {
                base.child(gpui::Text::new_inaccessible(label).into_any_element())
            }
            _ => base.children(self.children),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuGroupLabel<P> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        if let Some(label) = self.label.clone() {
            wiring.register_group_label(label);
        }
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuGroupLabel<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Registered label string, read by the parent group's wiring to source
    /// its `.aria_label(...)`.
    pub fn label_value(&self) -> Option<SharedString> {
        self.label.clone()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuGroupLabelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
