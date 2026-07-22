use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
};

use crate::select::{
    child_wiring::{SelectChildNode, SelectChildWiring},
    SelectContext, SelectGroupChild, SelectGroupStyleState,
};

#[derive(IntoElement)]
pub struct SelectGroup<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<SelectGroupChild<T>>,
    context: Option<SelectContext<T>>,
    index: Option<usize>,
    style_with_state: Option<Rc<dyn Fn(SelectGroupStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectGroup<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("select-group"),
            base: div(),
            children: Vec::new(),
            context: None,
            index: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectGroup<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectGroup<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let item_count = self
            .children
            .iter()
            .filter(|child| matches!(child, SelectGroupChild::Item(_)))
            .count();
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.group_state(self.index, item_count))
            })
            .unwrap_or_else(|| SelectGroupStyleState::new(item_count, self.index, None));
        let aria_label = state.label.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id(self.id)
            // AccessKit gap in this gpui revision: no `aria-labelledby`
            // builder, so the registered group-label text becomes a literal
            // `.aria_label(...)` instead of an id reference.
            .role(Role::Group)
            .when_some(aria_label, |this, label| this.aria_label(label))
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            )
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectGroup<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_select_context(context.clone()))
            .collect();
        self
    }

    fn wire_select_child(
        mut self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let group_index = wiring.begin_group();
        self.children = wiring.wire_group_children(self.children, window, cx);
        wiring.end_group();
        self.id = wiring.scope_child_id(&ElementId::from(gpui::SharedString::from(format!(
            "{}-{}",
            self.id, group_index
        ))));
        self.index = Some(group_index);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectGroup<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<SelectGroupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SelectGroupChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
