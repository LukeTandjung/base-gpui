use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{MenuChildNode, MenuChildWiring},
    MenuContext, MenuGroupChild, MenuGroupStyleState,
};

#[derive(IntoElement)]
pub struct MenuGroup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<MenuGroupChild<P>>,
    context: Option<MenuContext<P>>,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(MenuGroupStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuGroup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-group"),
            base: div(),
            children: Vec::new(),
            context: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuGroup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuGroup<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.group_state()))
            .unwrap_or_else(|| MenuGroupStyleState::new(false));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // `aria-labelledby` has no gpui builder (documented gap): the group
        // label registered by `MenuGroupLabel` is surfaced here as a literal
        // `.aria_label(...)` instead.
        base.id(self.id)
            .role(Role::Group)
            .when_some(self.aria_label, |this, label| this.aria_label(label))
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            )
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuGroup<P> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        if self.aria_label.is_none() {
            self.aria_label = self.children.iter().find_map(|child| match child {
                MenuGroupChild::GroupLabel(label) => label.label_value(),
                _ => None,
            });
        }
        self.id = wiring.scope_child_id(&self.id);
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_menu_child(wiring, context, window, cx))
            .collect();
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuGroup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Accessible group label. Defaults to the label registered by a
    /// `MenuGroupLabel` child.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn child(mut self, child: impl Into<MenuGroupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuGroupChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
