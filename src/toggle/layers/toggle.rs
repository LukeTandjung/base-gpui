use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, ClickEvent, Div,
    ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Toggled, Window,
};

use crate::{
    toggle::{
        ToggleActivate, ToggleContext, TogglePressedChangeDetails, TogglePressedChangeHandler,
        TogglePressedChangeReason, TogglePressedChangeSource, ToggleProps, ToggleStyleState,
        TOGGLE_KEY_CONTEXT,
    },
    toggle_group::{
        ToggleGroupActivateFocused, ToggleGroupContext, ToggleGroupFocusDown,
        ToggleGroupFocusFirst, ToggleGroupFocusLast, ToggleGroupFocusLeft, ToggleGroupFocusRight,
        ToggleGroupFocusUp, ToggleGroupMove, ToggleGroupOrientation, TOGGLE_GROUP_KEY_CONTEXT,
    },
    utils::direction::{current_direction, HorizontalArrowKey, HorizontalDirection},
};

#[derive(IntoElement)]
pub struct Toggle<T: Clone + Eq + 'static = SharedString> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    default_pressed: bool,
    pressed: Option<bool>,
    value: Option<T>,
    disabled: bool,
    aria_label: Option<SharedString>,
    on_pressed_change: Option<TogglePressedChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(ToggleStyleState, Div) -> Div + 'static>>,
    group_context: Option<ToggleGroupContext<T>>,
    group_index: Option<usize>,
    group_focus_handle: Option<FocusHandle>,
}

impl<T: Clone + Eq + 'static> Default for Toggle<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("toggle"),
            base: div(),
            children: Vec::new(),
            default_pressed: false,
            pressed: None,
            value: None,
            disabled: false,
            aria_label: None,
            on_pressed_change: None,
            style_with_state: None,
            group_context: None,
            group_index: None,
            group_focus_handle: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for Toggle<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> ParentElement for Toggle<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for Toggle<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = match self.group_focus_handle.clone() {
            Some(focus_handle) => focus_handle,
            None => toggle_focus_handle(&self.id, window, cx),
        };

        if self.group_context.is_some() {
            return render_grouped(self, focus_handle, window, cx);
        }

        let context = ToggleContext::new(
            self.id.clone(),
            cx,
            window,
            self.pressed.map(Some),
            Some(self.default_pressed),
            ToggleProps::new(self.disabled, self.on_pressed_change),
        );

        context.update(cx, |runtime| {
            runtime.sync_focused(focus_handle.is_focused(window));
        });

        let style_state = context.read(cx, |runtime, _props| runtime.state());
        let disabled = style_state.disabled;

        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        let keyboard_context = context.clone();
        let pointer_context = context.clone();
        let a11y_context = context.clone();

        base.id(self.id)
            .role(Role::Button)
            .aria_toggled(match style_state.pressed {
                true => Toggled::True,
                false => Toggled::False,
            })
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .key_context(TOGGLE_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &ToggleActivate, window, cx| {
                keyboard_context.toggle(TogglePressedChangeSource::Keyboard, window, cx);
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                pointer_context.toggle(TogglePressedChangeSource::Pointer, window, cx);
            })
            // The `on_click` guard above only accepts `ClickEvent::Mouse`, so an
            // AT-dispatched `Action::Click` (which gpui synthesizes as a
            // keyboard-style click) would be dropped there; route it explicitly
            // into the same runtime transition instead.
            .on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
                a11y_context.toggle(TogglePressedChangeSource::Keyboard, window, cx);
            })
            .children(self.children)
            .into_any_element()
    }
}

impl<T: Clone + Eq + 'static> Toggle<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn default_pressed(mut self, default_pressed: bool) -> Self {
        self.default_pressed = default_pressed;
        self
    }

    pub fn pressed(mut self, pressed: Option<bool>) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    /// The group-membership identity, consumed by the Toggle Group wiring.
    /// Has no standalone behavior.
    pub fn group_value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// The toggle's own disabled prop, consumed by the Toggle Group wiring
    /// when resolving the effective per-item disabled fact.
    pub fn own_disabled(&self) -> bool {
        self.disabled
    }

    /// The toggle's element id, consumed by the Toggle Group wiring to key
    /// the roving focus handle.
    pub fn toggle_id(&self) -> &ElementId {
        &self.id
    }

    /// Attaches this toggle to a Toggle Group as a composite item. Called by
    /// the Toggle Group child wiring; not intended for direct use.
    pub fn with_toggle_group(
        mut self,
        context: ToggleGroupContext<T>,
        index: usize,
        focus_handle: FocusHandle,
    ) -> Self {
        self.group_context = Some(context);
        self.group_index = Some(index);
        self.group_focus_handle = Some(focus_handle);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The accessible label announced by assistive technology. Icon-only
    /// toggles must set this; there is no `aria-labelledby` id-reference
    /// wiring in this gpui revision, so the literal string is the only
    /// labelling mechanism.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_pressed_change(
        mut self,
        on_pressed_change: impl Fn(bool, &mut TogglePressedChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_pressed_change = Some(Rc::new(on_pressed_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToggleStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

/// The stable keyed focus handle for a toggle, shared between standalone
/// rendering and the Toggle Group child wiring.
pub fn toggle_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

fn render_grouped<T: Clone + Eq + 'static>(
    toggle: Toggle<T>,
    focus_handle: FocusHandle,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let group = toggle
        .group_context
        .expect("grouped toggle should carry a group context");
    let index = toggle.group_index;
    let value = toggle.value;
    let own_disabled = toggle.disabled;
    let on_pressed_change = toggle.on_pressed_change;

    let (pressed, group_disabled, orientation, loop_focus, highlighted_index) =
        group.read(cx, |runtime, props| {
            (
                runtime.toggle_pressed(value.as_ref()),
                props.disabled(),
                props.orientation(),
                props.loop_focus(),
                runtime.highlighted_index(),
            )
        });
    let disabled = own_disabled || group_disabled;
    let focused = focus_handle.is_focused(window);
    let style_state = ToggleStyleState::new(pressed, disabled, focused);
    let tab_stop = !disabled && index.is_some() && highlighted_index == index;
    let direction = current_direction();

    let base = match toggle.style_with_state {
        Some(style) => style(style_state, toggle.base),
        None => toggle.base,
    };

    let left_group = group.clone();
    let right_group = group.clone();
    let up_group = group.clone();
    let down_group = group.clone();
    let home_group = group.clone();
    let end_group = group.clone();
    let keyboard_group = group.clone();
    let a11y_group = group.clone();
    let pointer_group = group;
    let keyboard_value = value.clone();
    let a11y_value = value.clone();
    let pointer_value = value;
    let keyboard_change = on_pressed_change.clone();
    let a11y_change = on_pressed_change.clone();
    let pointer_change = on_pressed_change;

    base.id(toggle.id)
        .role(Role::Button)
        .aria_toggled(match pressed {
            true => Toggled::True,
            false => Toggled::False,
        })
        .when_some(toggle.aria_label, |this, aria_label| {
            this.aria_label(aria_label)
        })
        .track_focus(
            &focus_handle
                .tab_stop(tab_stop)
                .tab_index(if tab_stop { 0 } else { -1 }),
        )
        .key_context(TOGGLE_GROUP_KEY_CONTEXT)
        .focusable()
        .on_action(move |_: &ToggleGroupFocusLeft, window, cx| {
            if orientation != ToggleGroupOrientation::Horizontal {
                return;
            }

            grouped_move(
                &left_group,
                horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Left)),
                loop_focus,
                window,
                cx,
            );
        })
        .on_action(move |_: &ToggleGroupFocusRight, window, cx| {
            if orientation != ToggleGroupOrientation::Horizontal {
                return;
            }

            grouped_move(
                &right_group,
                horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Right)),
                loop_focus,
                window,
                cx,
            );
        })
        .on_action(move |_: &ToggleGroupFocusUp, window, cx| {
            if orientation != ToggleGroupOrientation::Vertical {
                return;
            }

            grouped_move(&up_group, ToggleGroupMove::Previous, loop_focus, window, cx);
        })
        .on_action(move |_: &ToggleGroupFocusDown, window, cx| {
            if orientation != ToggleGroupOrientation::Vertical {
                return;
            }

            grouped_move(&down_group, ToggleGroupMove::Next, loop_focus, window, cx);
        })
        .on_action(move |_: &ToggleGroupFocusFirst, window, cx| {
            grouped_move(&home_group, ToggleGroupMove::First, loop_focus, window, cx);
        })
        .on_action(move |_: &ToggleGroupFocusLast, window, cx| {
            grouped_move(&end_group, ToggleGroupMove::Last, loop_focus, window, cx);
        })
        .on_action(move |_: &ToggleGroupActivateFocused, window, cx| {
            grouped_activate(
                &keyboard_group,
                keyboard_value.as_ref(),
                own_disabled,
                keyboard_change.as_ref(),
                TogglePressedChangeSource::Keyboard,
                window,
                cx,
            );
        })
        .on_click(move |event, window, cx| {
            if !matches!(event, ClickEvent::Mouse(_)) {
                return;
            }

            grouped_activate(
                &pointer_group,
                pointer_value.as_ref(),
                own_disabled,
                pointer_change.as_ref(),
                TogglePressedChangeSource::Pointer,
                window,
                cx,
            );
        })
        // See the standalone path: the `on_click` guard drops the synthesized
        // (non-mouse) click gpui produces for an AT-dispatched `Action::Click`,
        // so route it explicitly into the same grouped activation.
        .on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
            grouped_activate(
                &a11y_group,
                a11y_value.as_ref(),
                own_disabled,
                a11y_change.as_ref(),
                TogglePressedChangeSource::Keyboard,
                window,
                cx,
            );
        })
        .children(toggle.children)
        .into_any_element()
}

fn horizontal_move(direction: HorizontalDirection) -> ToggleGroupMove {
    match direction {
        HorizontalDirection::Previous => ToggleGroupMove::Previous,
        HorizontalDirection::Next => ToggleGroupMove::Next,
    }
}

fn grouped_move<T: Clone + Eq + 'static>(
    group: &ToggleGroupContext<T>,
    direction: ToggleGroupMove,
    loop_focus: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let focus_handle = group.update(cx, |runtime| {
        runtime.move_highlight(direction, loop_focus);
        runtime.highlighted_focus_handle()
    });

    if let Some(focus_handle) = focus_handle {
        focus_handle.focus(window, cx);
    }
}

fn grouped_activate<T: Clone + Eq + 'static>(
    group: &ToggleGroupContext<T>,
    value: Option<&T>,
    own_disabled: bool,
    on_pressed_change: Option<&TogglePressedChangeHandler>,
    source: TogglePressedChangeSource,
    window: &mut Window,
    cx: &mut App,
) {
    let (pressed, group_disabled) = group.read(cx, |runtime, props| {
        (runtime.toggle_pressed(value), props.disabled())
    });

    if own_disabled || group_disabled {
        return;
    }

    let next_pressed = !pressed;
    let mut details =
        TogglePressedChangeDetails::new(TogglePressedChangeReason::None, source, true);

    if let Some(on_pressed_change) = on_pressed_change {
        on_pressed_change(next_pressed, &mut details, window, cx);
    }

    if details.is_canceled() {
        return;
    }

    let Some(value) = value else {
        return;
    };

    group.commit_toggle(value.clone(), next_pressed, &mut details, window, cx);
}
