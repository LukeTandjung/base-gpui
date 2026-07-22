use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, App, ClickEvent, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Toggled, Window,
};

use crate::{
    field::{
        current_field_context, current_field_item_disabled, FieldControlRegistration, FieldValue,
    },
    fieldset::current_fieldset_disabled,
    switch::{
        child_wiring::SwitchChildNode, SwitchCheckedChangeDetails, SwitchCheckedChangeHandler,
        SwitchCheckedChangeSource, SwitchChild, SwitchContext, SwitchProps, SwitchRootStyleState,
        SwitchToggle, SWITCH_ROOT_KEY_CONTEXT,
    },
};

#[derive(IntoElement)]
pub struct SwitchRoot {
    id: ElementId,
    base: Div,
    children: Vec<SwitchChild>,
    name: Option<SharedString>,
    default_checked: bool,
    checked: Option<bool>,
    value: Option<SharedString>,
    form: Option<SharedString>,
    unchecked_value: Option<SharedString>,
    disabled: bool,
    read_only: bool,
    required: bool,
    aria_label: Option<SharedString>,
    on_checked_change: Option<SwitchCheckedChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(SwitchRootStyleState, Div) -> Div + 'static>>,
}

impl Default for SwitchRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("switch"),
            base: div(),
            children: Vec::new(),
            name: None,
            default_checked: false,
            checked: None,
            value: None,
            form: None,
            unchecked_value: None,
            disabled: false,
            read_only: false,
            required: false,
            aria_label: None,
            on_checked_change: None,
            style_with_state: None,
        }
    }
}

impl Styled for SwitchRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SwitchRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_disabled = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props).disabled))
            .unwrap_or(false);
        let item_disabled = current_field_item_disabled();
        let fieldset_disabled = current_fieldset_disabled();
        let disabled = self.disabled || field_disabled || item_disabled || fieldset_disabled;
        let name = self.name.clone();
        let id = self.id.clone();
        let context = SwitchContext::new(
            self.id.clone(),
            cx,
            window,
            self.checked.map(Some),
            Some(self.default_checked),
            SwitchProps::new(
                self.name,
                self.value,
                self.form,
                self.unchecked_value,
                disabled,
                self.read_only,
                self.required,
                self.on_checked_change,
            ),
        );

        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();
        context.update(cx, |runtime| {
            runtime.sync_focused(focus_handle.is_focused(window));
        });

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let disabled = style_state.disabled;
        if let Some(field_context) = field_context.as_ref() {
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(FieldValue::Bool(style_state.checked))
                .disabled(disabled)
                .focused(focus_handle.is_focused(window))
                .required(style_state.required)
                .focus_handle(focus_handle.clone());
            if let Some(name) = name {
                registration = registration.name(name);
            }
            field_context.register_control(registration, cx);
        }

        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        let keyboard_context = context.clone();
        let pointer_context = context.clone();

        base.id(self.id)
            .role(Role::Switch)
            .aria_toggled(if style_state.checked {
                Toggled::True
            } else {
                Toggled::False
            })
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .key_context(SWITCH_ROOT_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &SwitchToggle, window, cx| {
                keyboard_context.toggle(SwitchCheckedChangeSource::Keyboard, window, cx);
            })
            .on_click(move |event, window, cx| {
                // AT-dispatched `accesskit::Action::Click` is synthesized by gpui as a
                // mouse down/up pair, so it arrives here as `ClickEvent::Mouse` and is
                // accepted. `ClickEvent::Keyboard` is filtered out because keyboard
                // activation is handled by the `SwitchToggle` action above.
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                pointer_context.toggle(SwitchCheckedChangeSource::Pointer, window, cx);
            })
            .children(
                self.children
                    .into_iter()
                    .map(|child| child.with_switch_context(context.clone())),
            )
    }
}

impl SwitchRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<SwitchChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<SwitchChild>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn checked(mut self, checked: Option<bool>) -> Self {
        self.checked = checked;
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn form(mut self, form: impl Into<SharedString>) -> Self {
        self.form = Some(form.into());
        self
    }

    pub fn unchecked_value(mut self, unchecked_value: impl Into<SharedString>) -> Self {
        self.unchecked_value = Some(unchecked_value.into());
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

    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_checked_change(
        mut self,
        on_checked_change: impl Fn(bool, &mut SwitchCheckedChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(on_checked_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SwitchRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
