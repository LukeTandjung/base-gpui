use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, App, ClickEvent, Div, ElementId, Entity,
    FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

type SelectTriggerStyle<T> = Rc<dyn Fn(SelectTriggerStyleState<T>, Div) -> Div + 'static>;

use crate::select::{
    child_wiring::{SelectChildNode, SelectChildWiring},
    key::typeahead_text,
    SelectActivateHighlighted, SelectClose, SelectContext, SelectMoveFirst, SelectMoveLast,
    SelectMoveNext, SelectMovePrevious, SelectOpenChangeReason, SelectOpenChangeSource,
    SelectSelectionMode, SelectSide, SelectTriggerChild, SelectTriggerStyleState,
    SelectValueChangeSource, SELECT_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct SelectTrigger<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<SelectTriggerChild<T>>,
    context: Option<SelectContext<T>>,
    focus_handle: Option<FocusHandle>,
    aria_label: Option<SharedString>,
    style_with_state: Option<SelectTriggerStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for SelectTrigger<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("select-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectTrigger<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectTrigger<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| trigger_focus_handle(&self.id, window, cx));
        let context = self.context;
        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.trigger_state(props, SelectSide::Bottom)
                })
            })
            .unwrap_or_else(|| {
                SelectTriggerStyleState::new(
                    crate::select::SelectRootStyleState::new(
                        false,
                        false,
                        false,
                        false,
                        crate::select::SelectOpenChangeSource::None,
                        focus_handle.is_focused(window),
                        false,
                        false,
                        None,
                        SelectSelectionMode::Single,
                        None,
                        Vec::new(),
                        None,
                    ),
                    SelectSide::Bottom,
                )
            });
        let disabled = state.root.disabled;
        let read_only = state.root.read_only;
        let open = state.root.open;
        let click_context = context.clone();
        let typeahead_context = context.clone();
        let next_context = context.clone();
        let previous_context = context.clone();
        let first_context = context.clone();
        let last_context = context.clone();
        let activate_context = context.clone();
        let close_context = context.clone();
        let expand_context = context.clone();
        let collapse_context = context.clone();
        let aria_label = self.aria_label;

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let measure_context = context.clone();
        let trigger = base
            .id(self.id)
            // AccessKit gaps in this gpui revision: no builders for
            // `aria-haspopup="listbox"`, `aria-controls`, `aria-labelledby`,
            // `aria-readonly`, `aria-required`, or `aria-disabled`, so
            // Role::ComboBox + aria_expanded + a literal label is the best
            // available signal; disabled/read-only stay behavioral no-ops.
            .role(Role::ComboBox)
            .aria_expanded(open)
            .when_some(aria_label, |this, label| this.aria_label(label))
            .on_a11y_action(AccessibleAction::Expand, move |_data, window, cx| {
                if disabled || read_only {
                    return;
                }
                if let Some(context) = expand_context.as_ref() {
                    context.set_open(
                        true,
                        SelectOpenChangeReason::TriggerPress,
                        SelectOpenChangeSource::Programmatic,
                        window,
                        cx,
                    );
                }
            })
            .on_a11y_action(AccessibleAction::Collapse, move |_data, window, cx| {
                if disabled || read_only {
                    return;
                }
                if let Some(context) = collapse_context.as_ref() {
                    context.set_open(
                        false,
                        SelectOpenChangeReason::TriggerPress,
                        SelectOpenChangeSource::Programmatic,
                        window,
                        cx,
                    );
                }
            })
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .focusable()
            .key_context(SELECT_KEY_CONTEXT)
            .on_key_down(move |event, window, cx| {
                let Some(text) = typeahead_text(event) else {
                    return;
                };
                if let Some(context) = typeahead_context.as_ref() {
                    if context.typeahead(&text, window, cx) {
                        cx.stop_propagation();
                    }
                }
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) || disabled || read_only {
                    return;
                }
                if let Some(context) = click_context.as_ref() {
                    context.toggle_open(
                        SelectOpenChangeReason::TriggerPress,
                        SelectOpenChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .on_action(move |_: &SelectMoveNext, window, cx| {
                if disabled || read_only {
                    return;
                }
                if let Some(context) = next_context.as_ref() {
                    if open {
                        context.move_highlight(crate::select::SelectMove::Next, true, window, cx);
                    } else {
                        context.set_open(
                            true,
                            SelectOpenChangeReason::TriggerPress,
                            SelectOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                }
            })
            .on_action(move |_: &SelectMovePrevious, window, cx| {
                if disabled || read_only {
                    return;
                }
                if let Some(context) = previous_context.as_ref() {
                    if open {
                        context.move_highlight(
                            crate::select::SelectMove::Previous,
                            true,
                            window,
                            cx,
                        );
                    } else {
                        context.set_open(
                            true,
                            SelectOpenChangeReason::TriggerPress,
                            SelectOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                }
            })
            .on_action(move |_: &SelectMoveFirst, window, cx| {
                if open {
                    if let Some(context) = first_context.as_ref() {
                        context.move_highlight(crate::select::SelectMove::First, true, window, cx);
                    }
                }
            })
            .on_action(move |_: &SelectMoveLast, window, cx| {
                if open {
                    if let Some(context) = last_context.as_ref() {
                        context.move_highlight(crate::select::SelectMove::Last, true, window, cx);
                    }
                }
            })
            .on_action(move |_: &SelectActivateHighlighted, window, cx| {
                if disabled || read_only {
                    return;
                }
                if let Some(context) = activate_context.as_ref() {
                    if open {
                        context.activate_highlighted(SelectValueChangeSource::Keyboard, window, cx);
                    } else {
                        context.set_open(
                            true,
                            SelectOpenChangeReason::TriggerPress,
                            SelectOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                }
            })
            .on_action(move |_: &SelectClose, window, cx| {
                if let Some(context) = close_context.as_ref() {
                    context.set_open(
                        false,
                        SelectOpenChangeReason::EscapeKey,
                        SelectOpenChangeSource::Keyboard,
                        window,
                        cx,
                    );
                }
            })
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| runtime.set_trigger_bounds(bounds));
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(trigger)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectTrigger<T> {
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
        let focus_handle = trigger_focus_handle(&scoped_id, window, cx);
        wiring.register_trigger(focus_handle.clone(), focus_handle.is_focused(window));
        self.children = wiring.wire_trigger_children(self.children, window, cx);
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectTrigger<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<SelectTriggerChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<SelectTriggerChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SelectTriggerChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the accessible name announced for the trigger. Base UI relies on
    /// `aria-labelledby`, which has no gpui equivalent, so the literal string
    /// is the substitute. When the visible trigger content already matches the
    /// label, render that content with `Text::new_inaccessible(...)` to avoid
    /// double-announcing.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectTriggerStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn trigger_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
