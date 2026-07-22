use std::rc::Rc;

use gpui::{
    div, AnyElement, App, ClickEvent, Div, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::select::{child_wiring::SelectChildNode, SelectContext};

#[derive(IntoElement)]
pub struct SelectLabel<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SelectContext<T>>,
    style_with_state: Option<Rc<dyn Fn((), Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectLabel<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for SelectLabel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectLabel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectLabel<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self.context;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state((), self.base),
            None => self.base,
        };

        base.id("select-label")
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }
                if let Some(context) = context.as_ref() {
                    context.focus_trigger(window, cx);
                }
            })
            .children(self.children)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectLabel<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectLabel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(mut self, style: impl Fn((), Div) -> Div + 'static) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
