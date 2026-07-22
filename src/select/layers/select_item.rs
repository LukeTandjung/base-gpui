use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, App, ClickEvent, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

type SelectItemStyle<T> = Rc<dyn Fn(SelectItemStyleState<T>, Div) -> Div + 'static>;

use crate::{
    field::current_field_item_disabled,
    select::{
        child_wiring::{SelectChildNode, SelectChildWiring},
        SelectContext, SelectItemChild, SelectItemStyleState, SelectValueChangeReason,
        SelectValueChangeSource, SELECT_KEY_CONTEXT,
    },
};

#[derive(IntoElement)]
pub struct SelectItem<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<SelectItemChild<T>>,
    context: Option<SelectContext<T>>,
    value: Option<T>,
    label: Option<SharedString>,
    disabled: bool,
    index: Option<usize>,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<SelectItemStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for SelectItem<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("select-item"),
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            label: None,
            disabled: false,
            index: None,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectItem<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectItem<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let aria_label = self.resolved_label();
        let disabled = self.disabled || current_field_item_disabled();
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| item_focus_handle(&self.id, window, cx));
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.item_state(
                        self.value.as_ref(),
                        disabled,
                        self.index,
                        focus_handle.is_focused(window),
                        props,
                    )
                })
            })
            .unwrap_or_else(|| {
                SelectItemStyleState::new(
                    false,
                    false,
                    disabled,
                    false,
                    false,
                    focus_handle.is_focused(window),
                    false,
                    self.index,
                    None,
                    None,
                    None,
                    self.value.clone(),
                )
            });
        let item_disabled = state.disabled || state.root_disabled;
        let root_disabled = state.root_disabled;
        let selected = state.selected;
        let size_of_set = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.list_state().item_count))
            .unwrap_or_default();
        let tab_stop = state.tab_stop;
        let click_context = self.context.clone();
        let hover_context = self.context.clone();
        let value = self.value.clone();
        let index = self.index;
        let measure_context = self.context.clone();
        let measure_index = self.index;
        let children = self
            .children
            .into_iter()
            .map(|child| match child {
                SelectItemChild::Text(text) => {
                    (*text).with_item_state(state.clone()).into_any_element()
                }
                SelectItemChild::Indicator(indicator) => (*indicator)
                    .with_item_state(state.clone())
                    .into_any_element(),
                SelectItemChild::Any(any) => any,
            })
            .collect::<Vec<_>>();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let item = base
            .id(self.id)
            // AccessKit gaps in this gpui revision: no `aria-disabled` or
            // `aria-activedescendant` builders; disabled items stay behavioral
            // no-ops and AT tracks the highlighted option through real focus.
            .role(Role::ListBoxOption)
            .aria_selected(selected)
            .when_some(index, |this, index| this.aria_position_in_set(index + 1))
            .when(size_of_set > 0, |this| this.aria_size_of_set(size_of_set))
            .when_some(aria_label, |this, label| this.aria_label(label))
            .track_focus(
                &focus_handle
                    .tab_stop(tab_stop && !root_disabled)
                    .tab_index(if tab_stop && !root_disabled { 0 } else { -1 }),
            )
            .focusable()
            .key_context(SELECT_KEY_CONTEXT)
            .on_mouse_move(move |_event, _window, cx| {
                if let Some(context) = hover_context.as_ref() {
                    let should_highlight =
                        context.read(cx, |_runtime, props| props.highlight_item_on_hover());
                    if should_highlight {
                        context.update(cx, |runtime| runtime.highlight_item(index, item_disabled));
                    }
                }
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }
                let Some(context) = click_context.as_ref() else {
                    return;
                };
                let Some(value) = value.clone() else {
                    return;
                };

                context.select_value(
                    value,
                    item_disabled,
                    SelectValueChangeReason::ItemPress,
                    SelectValueChangeSource::Pointer,
                    window,
                    cx,
                );
            })
            .children(children);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(context) = measure_context.as_ref() else {
                    return;
                };
                let Some(index) = measure_index else {
                    return;
                };
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };

                if context.record_item_bounds(index, bounds, cx) {
                    window.request_animation_frame();
                }
            })
            .child(item)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectItem<T> {
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
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = item_focus_handle(&scoped_id, window, cx);
        let disabled = self.disabled || current_field_item_disabled();
        let label = self.resolved_label();
        let index = wiring.register_item(
            self.value.clone(),
            label,
            disabled,
            focus_handle.clone(),
            focus_handle.is_focused(window),
        );
        self.children = wiring.wire_item_children(self.children, window, cx);
        self.id = scoped_id;
        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectItem<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<SelectItemChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SelectItemChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Assigns the item value that participates in selection.
    ///
    /// Values are compared with `Eq`, so callers should prefer values that are
    /// unique within a `SelectRoot`. If duplicate values are registered, the
    /// selected state is value-based for every matching item, while value-label
    /// lookup and typeahead use the first matching item in render order.
    ///
    /// An item without a value is rendered but is not registered as selectable;
    /// use `T = Option<U>` when a selectable null-like value is needed.
    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectItemStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    fn resolved_label(&self) -> Option<SharedString> {
        self.label.clone().or_else(|| {
            self.children.iter().find_map(|child| match child {
                SelectItemChild::Text(text) => text.registration_label(),
                SelectItemChild::Indicator(_) | SelectItemChild::Any(_) => None,
            })
        })
    }
}

fn item_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
