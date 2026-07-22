use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, Entity,
    FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::tabs::{
    child_wiring::{TabsChildNode, TabsChildWiring},
    TabsContext, TabsOrientation, TabsTabStyleState,
};

#[derive(IntoElement)]
pub struct TabsTab<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TabsContext<T>>,
    value: Option<T>,
    disabled: bool,
    index: Option<usize>,
    focus_handle: Option<FocusHandle>,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(TabsTabStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsTab<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tabs-tab"),
            base: div(),
            children: Vec::from([]),
            context: None,
            value: None,
            disabled: false,
            index: None,
            focus_handle: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for TabsTab<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsTab<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsTab<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            value,
            disabled,
            index,
            focus_handle,
            aria_label,
            style_with_state,
        } = self;

        let focus_handle = focus_handle.unwrap_or_else(|| tab_focus_handle(&id, window, cx));

        let (state, tab_count) = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    (
                        runtime.tab_state(value.as_ref(), disabled, index, props.orientation()),
                        runtime.tab_count(),
                    )
                })
            })
            .unwrap_or_else(|| {
                (
                    TabsTabStyleState::new(false, disabled, false, TabsOrientation::Horizontal),
                    0,
                )
            });
        let active = state.active;
        let highlighted = state.highlighted;

        let selectable = match !disabled && !active {
            true => context.zip(value),
            false => None,
        };

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        base.id(id)
            .track_focus(
                &focus_handle
                    .tab_stop(highlighted && !disabled)
                    .tab_index(if highlighted { 0 } else { -1 }),
            )
            .role(Role::Tab)
            .aria_selected(active)
            .when_some(aria_label, |this, aria_label| this.aria_label(aria_label))
            .when_some(index.filter(|_| tab_count > 0), |this, index| {
                this.aria_position_in_set(index + 1)
                    .aria_size_of_set(tab_count)
            })
            .children(children)
            .when_some(selectable, |this, (context, value)| {
                this.on_click(move |event, window, cx| {
                    if !matches!(event, ClickEvent::Mouse(_)) {
                        return;
                    }

                    context.select(Some(value.clone()), window, cx);
                })
            })
    }
}

impl<T: Clone + Eq + 'static> TabsTab<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Accessible name for the tab, for icon-only tabs or when the name
    /// should differ from the visible child text. When set, create the
    /// visible label with `Text::new_inaccessible(...)` instead of
    /// `text!(...)` so the name is not announced twice.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsTabStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

impl<T: Clone + Eq + 'static> TabsChildNode<T> for TabsTab<T> {
    fn with_tabs_context(mut self, context: TabsContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_tabs_child(
        mut self,
        wiring: &mut TabsChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let focus_handle = tab_focus_handle(&self.id, window, cx);
        let index = wiring.register_tab(self.value.clone(), self.disabled, focus_handle.clone());

        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self
    }

    fn tab_index(&self) -> Option<usize> {
        self.index
    }
}

fn tab_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
