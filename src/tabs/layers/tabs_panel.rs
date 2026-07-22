use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Div, ElementId, Empty,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::tabs::{child_wiring::TabsChildNode, TabsContext, TabsOrientation, TabsPanelStyleState};

#[derive(IntoElement)]
pub struct TabsPanel<T: Clone + Eq + 'static> {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TabsContext<T>>,
    value: Option<T>,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(TabsPanelStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsPanel<T> {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::from([]),
            context: None,
            value: None,
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for TabsPanel<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsPanel<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsPanel<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            value,
            keep_mounted,
            style_with_state,
        } = self;

        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.panel_state(value.as_ref(), props.orientation())
                })
            })
            .unwrap_or_else(|| {
                TabsPanelStyleState::new(true, TabsOrientation::Horizontal, Default::default())
            });
        let active = !state.hidden;
        let hidden = state.hidden;
        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        if active || keep_mounted {
            let base = base
                .children(children)
                .when(hidden, |this| this.invisible());

            // Only the active panel enters the a11y tree; withholding the
            // id/role from `keep_mounted` inactive panels is this revision's
            // fallback for `hidden`/`inert` (no aria_hidden builder exists).
            match (active, id) {
                (true, Some(id)) => base.id(id).role(Role::TabPanel).into_any_element(),
                _ => base.into_any_element(),
            }
        } else {
            Empty.into_any_element()
        }
    }
}

impl<T: Clone + Eq + 'static> TabsChildNode<T> for TabsPanel<T> {
    fn with_tabs_context(mut self, context: TabsContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsPanel<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    /// Stable element id for the panel. Required for the active panel to
    /// appear in the accessibility tree with `Role::TabPanel`.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsPanelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
