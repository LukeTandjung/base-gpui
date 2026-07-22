use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::collapsible::{
    child_wiring::wire_children, CollapsibleChild, CollapsibleContext,
    CollapsibleOpenChangeDetails, CollapsibleOpenChangeHandler, CollapsibleProps,
    CollapsibleRootStyleState,
};

#[derive(IntoElement)]
pub struct CollapsibleRoot {
    id: ElementId,
    base: Div,
    children: Vec<CollapsibleChild>,
    default_open: bool,
    open: Option<bool>,
    disabled: bool,
    on_open_change: Option<CollapsibleOpenChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(CollapsibleRootStyleState, Div) -> Div + 'static>>,
}

impl Default for CollapsibleRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("collapsible"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            disabled: false,
            on_open_change: None,
            style_with_state: None,
        }
    }
}

impl Styled for CollapsibleRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for CollapsibleRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = CollapsibleContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            CollapsibleProps::new(self.disabled, self.on_open_change),
        );
        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let children = wired_children.children;

        context.update(cx, |runtime| {
            runtime.sync_trigger_focused(wired_children.trigger_focused);
        });

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(self.id).children(children)
    }
}

impl CollapsibleRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<CollapsibleChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<CollapsibleChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn open(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut CollapsibleOpenChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(CollapsibleRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
