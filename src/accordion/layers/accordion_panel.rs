use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Div, ElementId, Empty,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::accordion::{
    child_wiring::AccordionItemChildNode, AccordionItemContext, AccordionItemStyleState,
    AccordionOrientation, AccordionPanelStyleState,
};

#[derive(IntoElement)]
pub struct AccordionPanel<T: Clone + Eq + 'static> {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<AccordionItemContext<T>>,
    keep_mounted: Option<bool>,
    style_with_state: Option<Rc<dyn Fn(AccordionPanelStyleState<T>, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for AccordionPanel<T> {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for AccordionPanel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for AccordionPanel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AccordionPanel<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            keep_mounted,
            style_with_state,
        } = self;

        let effective_keep_mounted = context
            .as_ref()
            .map(|context| keep_mounted.unwrap_or_else(|| context.root_keep_mounted(cx)))
            .unwrap_or(keep_mounted.unwrap_or(false));
        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props, value, index, disabled| {
                    runtime.panel_state(value, index, disabled, effective_keep_mounted, props)
                })
            })
            .unwrap_or_else(|| {
                AccordionPanelStyleState::new(
                    AccordionItemStyleState::new(
                        panic_value(),
                        Vec::new(),
                        false,
                        false,
                        0,
                        AccordionOrientation::Vertical,
                    ),
                    effective_keep_mounted,
                )
            });
        let should_render = state.present;
        let hidden = state.item.hidden;
        let index = state.item.index;

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        if should_render {
            let id = id.unwrap_or_else(|| {
                ElementId::from(SharedString::from(format!("accordion-panel-{index}")))
            });

            // A kept-mounted but hidden panel keeps its id (stable node identity)
            // without a role, so its contents are not announced while closed.
            base.id(id)
                .when(!hidden, |this| this.role(Role::Region))
                .children(children)
                .when(hidden, |this| this.invisible())
                .into_any_element()
        } else {
            Empty.into_any_element()
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionItemChildNode<T> for AccordionPanel<T> {
    fn with_accordion_item_context(mut self, context: AccordionItemContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> AccordionPanel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = Some(keep_mounted);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AccordionPanelStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn panic_value<T: Clone + Eq + 'static>() -> T {
    panic!("AccordionPanel must be rendered inside AccordionItem")
}
