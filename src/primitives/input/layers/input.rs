use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::primitives::input::{
    InputBoundaryHandler, InputEnterHandler, InputRuntime, InputStyleState,
    InputValueChangeHandler, INPUT_KEY_CONTEXT,
};

use super::InputTextElement;

type InputStyleStateHandler = Rc<dyn Fn(InputStyleState, &mut Window, &mut App) + 'static>;

#[derive(derive_setters::Setters, IntoElement)]
pub struct Input {
    #[setters(into)]
    id: ElementId,
    #[setters(skip)]
    base: Div,
    #[setters(into, strip_option)]
    name: Option<SharedString>,
    #[setters(into, strip_option)]
    value: Option<SharedString>,
    #[setters(into)]
    default_value: SharedString,
    #[setters(into)]
    placeholder: SharedString,
    disabled: bool,
    read_only: bool,
    required: bool,
    auto_focus: bool,
    tab_index: isize,
    /// Overrides whether the input participates in window Tab order.
    /// Composite containers such as the Toolbar use this to keep a single
    /// roving tab stop. Defaults to `!disabled`.
    #[setters(strip_option)]
    tab_stop: Option<bool>,
    #[setters(skip)]
    on_value_change: Option<InputValueChangeHandler>,
    #[setters(skip)]
    on_enter: Option<InputEnterHandler>,
    #[setters(skip)]
    on_home: Option<InputBoundaryHandler>,
    #[setters(skip)]
    on_end: Option<InputBoundaryHandler>,
    #[setters(skip)]
    on_edge_left: Option<InputBoundaryHandler>,
    #[setters(skip)]
    on_edge_right: Option<InputBoundaryHandler>,
    #[setters(skip)]
    on_backspace: Option<InputBoundaryHandler>,
    #[setters(skip)]
    on_delete: Option<InputBoundaryHandler>,
    /// Selects the whole text whenever the input gains focus, matching
    /// composite-container focus behavior.
    select_all_on_focus: bool,
    #[setters(skip)]
    on_style_state: Option<InputStyleStateHandler>,
    #[setters(strip_option)]
    focus_handle: Option<FocusHandle>,
    #[setters(skip)]
    style_with_state: Option<Rc<dyn Fn(InputStyleState, Div) -> Div + 'static>>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            id: ElementId::from("input"),
            base: div(),
            name: None,
            value: None,
            default_value: SharedString::default(),
            placeholder: SharedString::default(),
            disabled: false,
            read_only: false,
            required: false,
            auto_focus: false,
            tab_index: 0,
            tab_stop: None,
            on_value_change: None,
            on_enter: None,
            on_home: None,
            on_end: None,
            on_edge_left: None,
            on_edge_right: None,
            on_backspace: None,
            on_delete: None,
            select_all_on_focus: false,
            on_style_state: None,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl Styled for Input {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Input {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled;
        let controlled_value = self.value.clone();
        let initial_value = controlled_value
            .clone()
            .unwrap_or_else(|| self.default_value.clone());
        let state_id =
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("state"));
        let state: Entity<InputRuntime> = window.use_keyed_state(state_id, cx, {
            let initial_value = initial_value.clone();
            let focus_handle = self.focus_handle.clone();
            move |window, cx| match focus_handle {
                Some(focus_handle) => {
                    InputRuntime::new_with_focus_handle(initial_value, focus_handle, window, cx)
                }
                None => InputRuntime::new(initial_value, window, cx),
            }
        });

        state.update(cx, |runtime, cx| {
            runtime.sync_props(
                controlled_value,
                disabled,
                self.read_only,
                self.required,
                self.on_value_change.clone(),
                self.on_enter.clone(),
                self.on_home.clone(),
                self.on_end.clone(),
                None,
                self.on_backspace.clone(),
                self.on_delete.clone(),
                cx,
            );
            runtime.sync_composite(
                self.on_edge_left.clone(),
                self.on_edge_right.clone(),
                self.select_all_on_focus,
            );
        });

        let focus_handle = state.read(cx).focus_handle();
        let focused_now = focus_handle.is_focused(window);
        state.update(cx, |runtime, cx| {
            runtime.sync_focus_observed(focused_now, cx);
        });
        let auto_focus_done_id = ElementId::NamedChild(
            Arc::new(self.id.clone()),
            SharedString::from("auto-focus-done"),
        );
        let auto_focus_done: Entity<bool> =
            window.use_keyed_state(auto_focus_done_id, cx, |_, _| false);
        if self.auto_focus && !disabled && !*auto_focus_done.read(cx) {
            focus_handle.focus(window, cx);
            *auto_focus_done.as_mut(cx) = true;
        }

        let style_state = state.read(cx).style_state(window, None);
        if let Some(on_style_state) = self.on_style_state.as_ref() {
            on_style_state(style_state.clone(), window, cx);
        }

        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        base.id(self.id)
            .track_focus(
                &focus_handle
                    .tab_stop(self.tab_stop.unwrap_or(!disabled))
                    .tab_index(if disabled { -1 } else { self.tab_index }),
            )
            .key_context(INPUT_KEY_CONTEXT)
            .focusable()
            .on_action(window.listener_for(&state, InputRuntime::left))
            .on_action(window.listener_for(&state, InputRuntime::right))
            .on_action(window.listener_for(&state, InputRuntime::select_left))
            .on_action(window.listener_for(&state, InputRuntime::select_right))
            .on_action(window.listener_for(&state, InputRuntime::select_all))
            .on_action(window.listener_for(&state, InputRuntime::home))
            .on_action(window.listener_for(&state, InputRuntime::end))
            .on_action(window.listener_for(&state, InputRuntime::copy))
            .on_action(window.listener_for(&state, InputRuntime::enter))
            .when(!disabled && !self.read_only, |this| {
                this.on_action(window.listener_for(&state, InputRuntime::backspace))
                    .on_action(window.listener_for(&state, InputRuntime::delete))
                    .on_action(window.listener_for(&state, InputRuntime::paste))
                    .on_action(window.listener_for(&state, InputRuntime::cut))
            })
            .when(!disabled, |this| {
                this.on_mouse_down(
                    MouseButton::Left,
                    window.listener_for(&state, InputRuntime::on_mouse_down),
                )
                .on_mouse_up(
                    MouseButton::Left,
                    window.listener_for(&state, InputRuntime::on_mouse_up),
                )
                .on_mouse_up_out(
                    MouseButton::Left,
                    window.listener_for(&state, InputRuntime::on_mouse_up),
                )
                .on_mouse_move(window.listener_for(&state, InputRuntime::on_mouse_move))
            })
            .child(InputTextElement::new(state, self.placeholder))
    }
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_value_change(mut self, on_value_change: impl Fn(SharedString) + 'static) -> Self {
        self.on_value_change = Some(Rc::new(move |value, _window, _cx| on_value_change(value)));
        self
    }

    pub fn on_value_change_with_context(
        mut self,
        on_value_change: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn on_enter(mut self, on_enter: impl Fn(SharedString) + 'static) -> Self {
        self.on_enter = Some(Rc::new(move |value, _window, _cx| on_enter(value)));
        self
    }

    pub fn on_enter_with_context(
        mut self,
        on_enter: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) + 'static,
    ) -> Self {
        self.on_enter = Some(Rc::new(on_enter));
        self
    }

    /// Consulted before any text edit on Backspace; returning `true`
    /// consumes the press (e.g. Combobox chip removal).
    pub fn on_backspace(
        mut self,
        on_backspace: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,
    ) -> Self {
        self.on_backspace = Some(Rc::new(on_backspace));
        self
    }

    /// Consulted before any text edit on Delete; returning `true` consumes
    /// the press.
    pub fn on_delete(
        mut self,
        on_delete: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,
    ) -> Self {
        self.on_delete = Some(Rc::new(on_delete));
        self
    }

    pub fn on_home(
        mut self,
        on_home: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool + 'static,
    ) -> Self {
        self.on_home = Some(Rc::new(on_home));
        self
    }

    pub fn on_end(
        mut self,
        on_end: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool + 'static,
    ) -> Self {
        self.on_end = Some(Rc::new(on_end));
        self
    }

    /// Consulted when a plain Left arrow is pressed with the caret at
    /// position 0 and no selection; returning `true` consumes the press.
    pub fn on_edge_left(
        mut self,
        on_edge_left: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,
    ) -> Self {
        self.on_edge_left = Some(Rc::new(on_edge_left));
        self
    }

    /// Consulted when a plain Right arrow is pressed with the caret at the
    /// end of the text and no selection; returning `true` consumes the press.
    pub fn on_edge_right(
        mut self,
        on_edge_right: impl Fn(SharedString, &mut Window, &mut gpui::Context<InputRuntime>) -> bool
            + 'static,
    ) -> Self {
        self.on_edge_right = Some(Rc::new(on_edge_right));
        self
    }

    pub fn on_style_state(
        mut self,
        on_style_state: impl Fn(InputStyleState, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_style_state = Some(Rc::new(on_style_state));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(InputStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

pub fn input() -> Input {
    Input::new()
}
