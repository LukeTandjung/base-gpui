use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, App, ClickEvent, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Toggled, Window,
};

use crate::{
    checkbox::{
        child_wiring::CheckboxChildNode, CheckboxCheckedChangeHandler, CheckboxCheckedChangeSource,
        CheckboxChild, CheckboxContext, CheckboxProps, CheckboxRootStyleState, CheckboxToggle,
        CHECKBOX_ROOT_KEY_CONTEXT,
    },
    checkbox_group::{current_checkbox_group_context, CheckboxGroupChildMetadata},
    field::{
        current_field_context, current_field_item_disabled, FieldControlRegistration, FieldValue,
    },
    fieldset::current_fieldset_disabled,
};

#[derive(IntoElement)]
pub struct CheckboxRoot {
    id: ElementId,
    base: Div,
    children: Vec<CheckboxChild>,
    name: Option<SharedString>,
    default_checked: bool,
    checked: Option<bool>,
    indeterminate: bool,
    value: Option<SharedString>,
    form: Option<SharedString>,
    parent: bool,
    unchecked_value: Option<SharedString>,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_checked_change: Option<CheckboxCheckedChangeHandler>,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(CheckboxRootStyleState, Div) -> Div + 'static>>,
}

impl Default for CheckboxRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("checkbox"),
            base: div(),
            children: Vec::new(),
            name: None,
            default_checked: false,
            checked: None,
            indeterminate: false,
            value: None,
            form: None,
            parent: false,
            unchecked_value: None,
            disabled: false,
            read_only: false,
            required: false,
            on_checked_change: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl Styled for CheckboxRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for CheckboxRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_disabled = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props).disabled))
            .unwrap_or(false);
        let item_disabled = current_field_item_disabled();
        let fieldset_disabled = current_fieldset_disabled();
        let group_context = current_checkbox_group_context();
        let group_disabled = group_context
            .as_ref()
            .map(|context| context.disabled())
            .unwrap_or(false);
        let disabled =
            self.disabled || field_disabled || item_disabled || fieldset_disabled || group_disabled;
        let name = self.name.clone();
        let id = self.id.clone();
        let value = self.value.clone();
        let parent = self.parent;
        let group_checked = group_context.as_ref().and_then(|context| {
            if parent {
                Some(context.parent_checked(cx))
            } else {
                value
                    .as_ref()
                    .map(|value| context.checked_for_value(value, cx))
            }
        });
        let indeterminate = if parent {
            self.indeterminate
                || group_context
                    .as_ref()
                    .map(|context| context.parent_indeterminate(cx))
                    .unwrap_or(false)
        } else {
            self.indeterminate
        };
        let controlled_checked = group_checked.or(self.checked).map(Some);
        let context = CheckboxContext::new(
            self.id.clone(),
            cx,
            window,
            controlled_checked,
            Some(self.default_checked),
            CheckboxProps::new(
                self.name,
                self.value,
                self.form,
                parent,
                self.unchecked_value,
                indeterminate,
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
        let focused = focus_handle.is_focused(window);
        if let Some(group_context) = group_context.as_ref() {
            group_context.register_checkbox(
                CheckboxGroupChildMetadata::new(id.to_string())
                    .maybe_value(value.clone())
                    .disabled(disabled)
                    .required(style_state.required)
                    .parent(parent)
                    .checked(style_state.checked)
                    .focused(focused)
                    .focus_handle(focus_handle.clone()),
                cx,
            );
        } else if let Some(field_context) = field_context.as_ref() {
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(FieldValue::Bool(style_state.checked))
                .disabled(disabled)
                .focused(focused)
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

        let action_context = context.clone();
        let action_group_context = group_context.clone();
        let action_value = value.clone();
        let toggle_context = context.clone();
        let toggle_group_context = group_context;
        let toggle_value = value;

        let aria_toggled = if style_state.indeterminate {
            Toggled::Mixed
        } else if style_state.checked {
            Toggled::True
        } else {
            Toggled::False
        };

        base.id(self.id)
            .role(Role::CheckBox)
            .aria_toggled(aria_toggled)
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .key_context(CHECKBOX_ROOT_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &CheckboxToggle, window, cx| {
                request_checkbox_toggle(
                    &action_context,
                    action_group_context.as_ref(),
                    action_value.clone(),
                    parent,
                    CheckboxCheckedChangeSource::Keyboard,
                    window,
                    cx,
                );
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                request_checkbox_toggle(
                    &toggle_context,
                    toggle_group_context.as_ref(),
                    toggle_value.clone(),
                    parent,
                    CheckboxCheckedChangeSource::Pointer,
                    window,
                    cx,
                );
            })
            .children(
                self.children
                    .into_iter()
                    .map(|child| child.with_checkbox_context(context.clone())),
            )
    }
}

fn request_checkbox_toggle(
    checkbox_context: &CheckboxContext,
    group_context: Option<&crate::checkbox_group::CheckboxGroupContext>,
    value: Option<SharedString>,
    parent: bool,
    source: CheckboxCheckedChangeSource,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(next_checked) = checkbox_context.request_toggle(source, window, cx) else {
        return;
    };

    match group_context {
        Some(group_context) if parent => group_context.toggle_parent(window, cx),
        Some(group_context) if value.is_some() => {
            group_context.toggle_child(value, next_checked, window, cx);
        }
        _ => checkbox_context.commit_checked(next_checked, cx),
    }
}

impl CheckboxRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<CheckboxChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<CheckboxChild>>,
    ) -> Self {
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

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
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

    pub fn parent(mut self, parent: bool) -> Self {
        self.parent = parent;
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

    pub fn on_checked_change(
        mut self,
        on_checked_change: impl Fn(bool, &mut crate::checkbox::CheckboxCheckedChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_checked_change = Some(Rc::new(on_checked_change));
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(CheckboxRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
