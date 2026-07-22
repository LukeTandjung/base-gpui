use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, Entity,
    FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::button::{ButtonActivate, ButtonRootStyleState, BUTTON_ROOT_KEY_CONTEXT};

type ButtonClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ButtonRoot {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    disabled: bool,
    focusable_when_disabled: bool,
    aria_label: Option<SharedString>,
    on_click: Option<ButtonClickHandler>,
    style_with_state: Option<Rc<dyn Fn(ButtonRootStyleState, Div) -> Div + 'static>>,
}

impl Default for ButtonRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("button-root"),
            base: div(),
            children: Vec::new(),
            disabled: false,
            focusable_when_disabled: false,
            aria_label: None,
            on_click: None,
            style_with_state: None,
        }
    }
}

impl Styled for ButtonRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for ButtonRoot {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = button_focus_handle(&self.id, window, cx);

        let disabled = self.disabled;
        let tab_stop = !disabled || self.focusable_when_disabled;
        let style_state = ButtonRootStyleState::new(disabled, focus_handle.is_focused(window));

        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        let keyboard_handler = self.on_click.clone();
        let pointer_handler = self.on_click;

        base.id(self.id)
            .role(Role::Button)
            // Note: gpui has no `.aria_disabled(...)` builder in this revision,
            // so neither plain `disabled` nor `focusable_when_disabled` is
            // announced to AT; activation stays inert via `activate(...)`.
            .when_some(self.aria_label, |base, label| base.aria_label(label))
            .key_context(BUTTON_ROOT_KEY_CONTEXT)
            // A non-focusable disabled button must not track focus at all:
            // GPUI focuses any tracked element on click, which would paint
            // the focus ring even though the button is inert.
            .when(tab_stop, |base| {
                base.track_focus(&focus_handle.tab_stop(true).tab_index(0))
                    .focusable()
            })
            .on_action(move |_: &ButtonActivate, window, cx| {
                activate(
                    disabled,
                    keyboard_handler.as_ref(),
                    &ClickEvent::default(),
                    window,
                    cx,
                );
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                activate(disabled, pointer_handler.as_ref(), event, window, cx);
            })
            .children(self.children)
            .into_any_element()
    }
}

impl ButtonRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focusable_when_disabled(mut self, focusable_when_disabled: bool) -> Self {
        self.focusable_when_disabled = focusable_when_disabled;
        self
    }

    /// Accessible name for the button. Required for icon-only buttons; when
    /// set alongside a visible text child, render that child with
    /// `Text::new_inaccessible(...)` to avoid double-announcing.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ButtonRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

/// The single activation path shared by pointer and keyboard: the disabled
/// guard lives here and nowhere else.
fn activate(
    disabled: bool,
    on_click: Option<&ButtonClickHandler>,
    event: &ClickEvent,
    window: &mut Window,
    cx: &mut App,
) {
    if disabled {
        return;
    }

    if let Some(on_click) = on_click {
        on_click(event, window, cx);
    }
}

fn button_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
