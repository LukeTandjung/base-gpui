use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::accordion::{
    child_wiring::{AccordionHeaderChildNode, AccordionItemChildNode, AccordionItemChildWiring},
    AccordionHeaderChild, AccordionHeaderStyleState, AccordionItemContext, AccordionItemStyleState,
    AccordionOrientation,
};

#[derive(IntoElement)]
pub struct AccordionHeader<T: Clone + Eq + 'static> {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AccordionHeaderChild<T>>,
    context: Option<AccordionItemContext<T>>,
    heading_level: usize,
    style_with_state: Option<Rc<dyn Fn(AccordionHeaderStyleState<T>, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for AccordionHeader<T> {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::new(),
            context: None,
            heading_level: 3,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for AccordionHeader<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AccordionHeader<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props, value, index, disabled| {
                    runtime.header_state(value, index, disabled, props)
                })
            })
            .unwrap_or_else(|| {
                AccordionHeaderStyleState::new(AccordionItemStyleState::new(
                    panic_value(),
                    Vec::new(),
                    false,
                    false,
                    0,
                    AccordionOrientation::Vertical,
                ))
            });

        let index = state.item.index;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let id = self.id.unwrap_or_else(|| {
            ElementId::from(SharedString::from(format!("accordion-header-{index}")))
        });
        let base = base
            .id(id)
            .role(Role::Heading)
            .aria_level(self.heading_level);

        match self.context {
            Some(context) => base.children(
                self.children
                    .into_iter()
                    .map(|child| child.with_accordion_item_context(context.clone())),
            ),
            None => base.children(self.children),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionItemChildNode<T> for AccordionHeader<T> {
    fn with_accordion_item_context(mut self, context: AccordionItemContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_accordion_item_child(
        mut self,
        wiring: &mut AccordionItemChildWiring,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_accordion_header_child(wiring, window, cx))
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> AccordionHeader<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn heading_level(mut self, heading_level: usize) -> Self {
        self.heading_level = heading_level;
        self
    }

    pub fn child(mut self, child: impl Into<AccordionHeaderChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<AccordionHeaderChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AccordionHeaderStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn panic_value<T: Clone + Eq + 'static>() -> T {
    panic!("AccordionHeader must be rendered inside AccordionItem")
}
