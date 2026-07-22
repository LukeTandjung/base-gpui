use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::tabs::{
    child_wiring::wire_children, TabsChild, TabsContext, TabsOrientation, TabsProps,
    TabsRootStyleState, TabsValueChangeHandler,
};

#[derive(IntoElement)]
pub struct TabsRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<TabsChild<T>>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    on_value_change: Option<TabsValueChangeHandler<T>>,
    orientation: TabsOrientation,
    style_with_state: Option<Rc<dyn Fn(TabsRootStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tabs"),
            base: div(),
            children: Vec::from([]),
            default_value: None,
            value: None,
            on_value_change: None,
            orientation: TabsOrientation::Horizontal,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let controlled = self.value.clone();
        let is_controlled = controlled.is_some();
        let orientation = self.orientation;
        let context = TabsContext::new(
            self.id,
            cx,
            window,
            self.value,
            self.default_value,
            TabsProps::new(orientation, self.on_value_change),
        );
        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let tabs = wired_children.tabs;
        let tab_focus_handles = wired_children.tab_focus_handles;
        let children = wired_children.children;

        // Scoped tab key bindings only dispatch once focus is inside the tabs list.
        // Seed focus once from the synced highlighted tab so initial keyboard use works.
        let focus_handle = context.update(cx, |runtime| {
            runtime.sync_children(tabs, tab_focus_handles);

            let observed_selected = controlled.unwrap_or_else(|| runtime.selected_value());
            runtime.reconcile(observed_selected, !is_controlled, orientation);
            runtime.take_initial_focus_handle()
        });
        if let Some(focus_handle) = focus_handle {
            focus_handle.focus(window, cx);
        }

        let style_state =
            context.read(cx, |runtime, props| runtime.root_state(props.orientation()));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };
        base.children(children)
    }
}

impl<T: Clone + Eq + 'static> TabsRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<TabsChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<TabsChild<T>>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: Option<T>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, default_value: Option<T>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(Option<&T>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
