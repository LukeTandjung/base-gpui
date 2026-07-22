use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::accordion::{
    child_wiring::{
        AccordionChildWiring, AccordionItemChildNode, AccordionItemChildWiring,
        AccordionRootChildNode,
    },
    AccordionContext, AccordionItemChild, AccordionItemContext, AccordionItemOpenChangeDetails,
    AccordionItemOpenChangeHandler, AccordionItemStyleState, AccordionOrientation,
};

#[derive(IntoElement)]
pub struct AccordionItem<T: Clone + Eq + 'static> {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AccordionItemChild<T>>,
    context: Option<AccordionContext<T>>,
    value: T,
    disabled: bool,
    index: Option<usize>,
    on_open_change: Option<AccordionItemOpenChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(AccordionItemStyleState<T>, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Styled for AccordionItem<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AccordionItem<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            value,
            disabled,
            index,
            on_open_change,
            style_with_state,
        } = self;
        let index = index.unwrap_or(0);
        let id = id.unwrap_or_else(|| {
            ElementId::from(SharedString::from(format!("accordion-item-{index}")))
        });

        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.item_state(&value, index, disabled, props)
                })
            })
            .unwrap_or_else(|| {
                AccordionItemStyleState::new(
                    value.clone(),
                    Vec::new(),
                    false,
                    disabled,
                    index,
                    AccordionOrientation::Vertical,
                )
            });
        let item_context = context.map(|context| {
            AccordionItemContext::new(context, value, index, disabled, on_open_change)
        });

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        // Base UI's item wrapper carries no ARIA role, so the item div stays out of
        // the a11y tree; the keyed id keeps descendant node ids stable across frames.
        let base = base.id(id);

        match item_context {
            Some(item_context) => base.children(
                children
                    .into_iter()
                    .map(|child| child.with_accordion_item_context(item_context.clone())),
            ),
            None => base.children(children),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionRootChildNode<T> for AccordionItem<T> {
    fn with_accordion_context(mut self, context: AccordionContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_accordion_child(
        mut self,
        wiring: &mut AccordionChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let index = wiring.next_item_index();
        let mut item_wiring = AccordionItemChildWiring::new();
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_accordion_item_child(&mut item_wiring, window, cx))
            .collect();
        wiring.register_item(
            self.value.clone(),
            self.disabled,
            index,
            item_wiring.trigger_focus_handle(),
            item_wiring.trigger_focused(),
        );
        self.index = Some(index);
        self
    }
}

impl<T: Clone + Eq + 'static> AccordionItem<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::new(),
            context: None,
            value,
            disabled: false,
            index: None,
            on_open_change: None,
            style_with_state: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn child(mut self, child: impl Into<AccordionItemChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<AccordionItemChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut AccordionItemOpenChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AccordionItemStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
