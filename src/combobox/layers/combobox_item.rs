use std::rc::Rc;

use gpui::{
    div, App, ClickEvent, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
};

type ComboboxItemStyle<T> = Rc<dyn Fn(ComboboxItemStyleState<T>, Div) -> Div + 'static>;

use crate::{
    combobox::{
        child_wiring::{ComboboxChildNode, ComboboxChildWiring},
        ComboboxChangeSource, ComboboxContext, ComboboxItemChild, ComboboxItemStyleState,
        ComboboxSelectionMode,
    },
    field::current_field_item_disabled,
};

/// A selectable option. Focus never moves to items — the highlight is
/// virtual runtime state while focus stays on the input.
///
/// Values are compared with `Eq`; if duplicate values are registered, the
/// selected state is value-based for every matching item, while value-label
/// lookup uses the first matching item in render order.
#[derive(IntoElement)]
pub struct ComboboxItem<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<ComboboxItemChild<T>>,
    context: Option<ComboboxContext<T>>,
    value: Option<T>,
    label: Option<SharedString>,
    disabled: bool,
    index: Option<usize>,
    style_with_state: Option<ComboboxItemStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxItem<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("combobox-item"),
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            label: None,
            disabled: false,
            index: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxItem<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxItem<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled || current_field_item_disabled();
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.item_state(self.value.as_ref(), disabled, self.index, props)
                })
            })
            .unwrap_or_else(|| {
                ComboboxItemStyleState::new(
                    false,
                    false,
                    disabled,
                    false,
                    true,
                    self.index,
                    self.value.clone(),
                )
            });

        // Layers never filter; the runtime owns the filtered set and items
        // simply consult their visibility fact.
        if !state.visible {
            return div().into_any_element();
        }

        let item_disabled = state.disabled || state.root_disabled;
        // AccessKit facts, gathered before `state` moves into the style
        // closure. `aria-selected` is gated off in `None` selection mode,
        // matching Base UI's `selectable`-gated emission; position/size are
        // computed over the filtered (visible) set only.
        let aria_selected =
            self.context
                .as_ref()
                .and_then(|context| match context.props().selection_mode {
                    ComboboxSelectionMode::None => None,
                    _ => Some(state.selected),
                });
        let aria_label =
            self.label
                .clone()
                .or_else(|| match (self.context.as_ref(), self.value.as_ref()) {
                    (Some(context), Some(value)) => {
                        context.read(cx, |runtime, _| runtime.label_for_value(value))
                    }
                    _ => None,
                });
        let set_position = self
            .context
            .as_ref()
            .zip(self.index)
            .and_then(|(context, index)| {
                context.read(cx, |runtime, _| {
                    let filtered = runtime.filtered_indices();
                    let position = filtered.iter().position(|&candidate| candidate == index)?;
                    Some((position + 1, filtered.len()))
                })
            });
        let click_context = self.context.clone();
        let hover_context = self.context.clone();
        let value = self.value.clone();
        let index = self.index;
        let children = self
            .children
            .into_iter()
            .map(|child| match child {
                ComboboxItemChild::Indicator(indicator) => (*indicator)
                    .with_item_state(state.clone())
                    .into_any_element(),
                ComboboxItemChild::Any(any) => any,
            })
            .collect::<Vec<_>>();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // `.on_click` below auto-registers `Action::Click`; highlight-following
        // announcement (`aria-activedescendant`) is blocked pending gpui
        // upstream (documented in `combobox/mod.rs`).
        let mut item = base.id(self.id).role(Role::ListBoxOption);
        if let Some(selected) = aria_selected {
            item = item.aria_selected(selected);
        }
        if let Some(label) = aria_label {
            item = item.aria_label(label);
        }
        if let Some((position, size)) = set_position {
            item = item.aria_position_in_set(position).aria_size_of_set(size);
        }

        item.on_mouse_move(move |_event, window, cx| {
            if let Some(context) = hover_context.as_ref() {
                context.highlight_item_from_pointer(index, item_disabled, window, cx);
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

            context.select_item(
                value,
                item_disabled,
                ComboboxChangeSource::Pointer,
                window,
                cx,
            );
        })
        .children(children)
        .into_any_element()
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxItem<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_combobox_context(context.clone()))
            .collect();
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        let scoped_id = wiring.scope_child_id(&self.id);
        let disabled = self.disabled || current_field_item_disabled();
        let index = wiring.register_item(self.value.clone(), self.label.clone(), disabled);
        self.id = scoped_id;
        self.index = Some(index);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxItem<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxItemChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxItemChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    /// Label used for display and filtering.
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
        style: impl Fn(ComboboxItemStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
