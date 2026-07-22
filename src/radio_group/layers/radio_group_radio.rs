use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, App, ClickEvent, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Toggled, Window,
};

use crate::{
    field::current_field_item_disabled,
    radio_group::{
        child_wiring::{RadioGroupChildNode, RadioGroupChildWiring, RadioGroupRadioChildNode},
        Move, RadioGroupActivateFocused, RadioGroupContext, RadioGroupRadioChild,
        RadioGroupRadioStyleState, RadioGroupSelectDown, RadioGroupSelectLeft,
        RadioGroupSelectRight, RadioGroupSelectUp, RadioGroupValueChangeSource,
        RADIO_GROUP_KEY_CONTEXT,
    },
    utils::direction::{current_direction, HorizontalArrowKey, HorizontalDirection},
};

#[derive(IntoElement)]
pub struct RadioGroupRadio<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<RadioGroupRadioChild>,
    context: Option<RadioGroupContext<T>>,
    value: Option<T>,
    disabled: bool,
    read_only: bool,
    required: bool,
    index: Option<usize>,
    focus_handle: Option<FocusHandle>,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(RadioGroupRadioStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for RadioGroupRadio<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("radio-group-radio"),
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            disabled: false,
            read_only: false,
            required: false,
            index: None,
            focus_handle: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for RadioGroupRadio<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for RadioGroupRadio<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            value,
            disabled,
            read_only,
            required,
            index,
            focus_handle,
            aria_label,
            style_with_state,
        } = self;

        let disabled = disabled || current_field_item_disabled();
        let focus_handle = focus_handle.unwrap_or_else(|| radio_focus_handle(&id, window, cx));
        let direction = current_direction();
        let state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props| {
                    runtime.radio_state(value.as_ref(), disabled, read_only, required, index, props)
                })
            })
            .unwrap_or_else(|| {
                RadioGroupRadioStyleState::new(
                    false,
                    disabled,
                    read_only,
                    required,
                    focus_handle.is_focused(window),
                    false,
                    false,
                )
            });

        let radio_disabled = state.disabled;
        let radio_read_only = state.read_only;
        let tab_stop = state.tab_stop;
        let click_context = context.clone();
        let click_value = value.clone();
        let left_context = context.clone();
        let right_context = context.clone();
        let up_context = context.clone();
        let down_context = context.clone();
        let activate_context = context.clone();
        let activate_value = value.clone();
        let size_of_set = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.radio_count()))
            .filter(|count| *count > 0);
        let children = children
            .into_iter()
            .map(|child| child.with_radio_state(state).into_element())
            .collect::<Vec<_>>();

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        base.id(id)
            .track_focus(
                &focus_handle
                    .tab_stop(tab_stop && !radio_disabled)
                    .tab_index(if tab_stop && !radio_disabled { 0 } else { -1 }),
            )
            .key_context(RADIO_GROUP_KEY_CONTEXT)
            .focusable()
            .role(Role::RadioButton)
            .aria_toggled(match state.checked {
                true => Toggled::True,
                false => Toggled::False,
            })
            .when_some(aria_label, |this, aria_label| this.aria_label(aria_label))
            .when_some(index, |this, index| this.aria_position_in_set(index + 1))
            .when_some(size_of_set, |this, count| this.aria_size_of_set(count))
            .on_action(move |_: &RadioGroupSelectLeft, window, cx| {
                let Some(context) = left_context.as_ref() else {
                    return;
                };

                move_and_select(
                    context,
                    horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Left)),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &RadioGroupSelectRight, window, cx| {
                let Some(context) = right_context.as_ref() else {
                    return;
                };

                move_and_select(
                    context,
                    horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Right)),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &RadioGroupSelectUp, window, cx| {
                let Some(context) = up_context.as_ref() else {
                    return;
                };

                move_and_select(context, Move::Previous, window, cx);
            })
            .on_action(move |_: &RadioGroupSelectDown, window, cx| {
                let Some(context) = down_context.as_ref() else {
                    return;
                };

                move_and_select(context, Move::Next, window, cx);
            })
            .on_action(move |_: &RadioGroupActivateFocused, window, cx| {
                let Some(context) = activate_context.as_ref() else {
                    return;
                };
                let Some(value) = activate_value.clone() else {
                    return;
                };

                context.select(
                    value,
                    RadioGroupValueChangeSource::Keyboard,
                    radio_disabled,
                    radio_read_only,
                    window,
                    cx,
                );
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }
                let Some(context) = click_context.as_ref() else {
                    return;
                };
                let Some(value) = click_value.clone() else {
                    return;
                };

                context.select(
                    value,
                    RadioGroupValueChangeSource::Pointer,
                    radio_disabled,
                    radio_read_only,
                    window,
                    cx,
                );
            })
            .children(children)
    }
}

impl<T: Clone + Eq + 'static> RadioGroupChildNode<T> for RadioGroupRadio<T> {
    fn with_radio_group_context(mut self, context: RadioGroupContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_radio_group_child(
        mut self,
        wiring: &mut RadioGroupChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let focus_handle = radio_focus_handle(&self.id, window, cx);
        let disabled = self.disabled || current_field_item_disabled();
        let index = wiring.register_radio(
            self.value.clone(),
            disabled,
            self.read_only,
            self.required,
            focus_handle.clone(),
            focus_handle.is_focused(window),
        );

        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<T: Clone + Eq + 'static> RadioGroupRadio<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<RadioGroupRadioChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<RadioGroupRadioChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Sets the accessible label announced by screen readers. This is the
    /// literal-string substitute for Base UI's `aria-labelledby` id wiring,
    /// which has no gpui builder. When set, render the radio's visible label
    /// text with `Text::new_inaccessible(...)` instead of `text!(...)` so the
    /// label is not announced twice.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(RadioGroupRadioStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn horizontal_move(direction: HorizontalDirection) -> Move {
    match direction {
        HorizontalDirection::Previous => Move::Previous,
        HorizontalDirection::Next => Move::Next,
    }
}

fn move_and_select<T: Clone + Eq + 'static>(
    context: &RadioGroupContext<T>,
    direction: Move,
    window: &mut Window,
    cx: &mut App,
) {
    let focus_handle = context.update(cx, |runtime| {
        runtime.move_highlight(direction);
        runtime.highlighted_focus_handle()
    });
    if let Some(focus_handle) = focus_handle {
        focus_handle.focus(window, cx);
    }

    let target = context.read(cx, |runtime, _| runtime.highlighted_selection_target());
    if let Some(target) = target {
        let disabled = target.disabled();
        let read_only = target.read_only();
        context.select(
            target.into_value(),
            RadioGroupValueChangeSource::Keyboard,
            disabled,
            read_only,
            window,
            cx,
        );
    }
}

fn radio_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
