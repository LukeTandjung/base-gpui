use std::{rc::Rc, sync::Arc};

use gpui::{
    accesskit::ActionData, div, prelude::FluentBuilder as _, AccessibleAction, App, Div, ElementId,
    Entity, FocusHandle, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::{
    field::{
        current_field_context, current_field_item_disabled, FieldContext, FieldControlRegistration,
        FieldValue,
    },
    fieldset::current_fieldset_disabled,
    form::{current_form_context, FormContext},
    otp_field::{
        child_wiring::wire_children, OTPFieldBackspace, OTPFieldChangeDetails, OTPFieldChild,
        OTPFieldClear, OTPFieldContext, OTPFieldDelete, OTPFieldFirst, OTPFieldLast, OTPFieldMove,
        OTPFieldNext, OTPFieldNormalizeValueHandler, OTPFieldPrevious, OTPFieldProps,
        OTPFieldRootStyleState, OTPFieldValidationType, OTPFieldValueChangeHandler,
        OTPFieldValueCompleteHandler, OTPFieldValueInvalidHandler, OTP_FIELD_KEY_CONTEXT,
    },
    primitives::input::InputRuntime,
};

#[derive(IntoElement)]
pub struct OTPFieldRoot {
    id: ElementId,
    base: Div,
    children: Vec<OTPFieldChild>,
    context: Option<FieldContext>,
    name: Option<SharedString>,
    aria_label: Option<SharedString>,
    length: usize,
    default_value: SharedString,
    value: Option<SharedString>,
    validation_type: OTPFieldValidationType,
    normalize_value: Option<OTPFieldNormalizeValueHandler>,
    mask: bool,
    auto_submit: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_value_change: Option<OTPFieldValueChangeHandler>,
    on_value_complete: Option<OTPFieldValueCompleteHandler>,
    on_value_invalid: Option<OTPFieldValueInvalidHandler>,
    style_with_state: Option<Rc<dyn Fn(OTPFieldRootStyleState, Div) -> Div + 'static>>,
}

impl Default for OTPFieldRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("otp-field"),
            base: div(),
            children: Vec::new(),
            context: None,
            name: None,
            aria_label: None,
            length: 0,
            default_value: SharedString::default(),
            value: None,
            validation_type: OTPFieldValidationType::default(),
            normalize_value: None,
            mask: false,
            auto_submit: false,
            disabled: false,
            read_only: false,
            required: false,
            on_value_change: None,
            on_value_complete: None,
            on_value_invalid: None,
            style_with_state: None,
        }
    }
}

impl Styled for OTPFieldRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for OTPFieldRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = self.context.or_else(current_field_context);
        let form_context: Option<FormContext> = current_form_context();
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state
            .as_ref()
            .map(|state| state.disabled)
            .unwrap_or(false);
        let field_valid = field_state.as_ref().and_then(|state| state.valid);
        let item_disabled = current_field_item_disabled();
        let fieldset_disabled = current_fieldset_disabled();
        let disabled = self.disabled || field_disabled || item_disabled || fieldset_disabled;
        let id = self.id.clone();
        let name = self.name.clone();
        let aria_label = self.aria_label.clone();

        #[cfg(debug_assertions)]
        if self.length == 0 {
            eprintln!(
                "OTPFieldRoot: `.length(...)` is zero or missing; no editable slots will render"
            );
        }

        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();

        let context = OTPFieldContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            OTPFieldProps::new(
                self.name,
                self.length,
                self.validation_type,
                self.normalize_value,
                self.mask,
                self.auto_submit,
                disabled,
                self.read_only,
                self.required,
                self.on_value_change,
                self.on_value_complete,
                self.on_value_invalid,
            )
            .with_aria_label(self.aria_label),
            focus_handle.clone(),
            form_context,
            field_valid,
        );
        context.sync_focus(focus_handle.is_focused(window), window, cx);

        // Keep the platform input bridge in controlled mode: push the normalized
        // OTP value and hooks every render. The public `sync_props` selection
        // clamping is sufficient to keep the cursor at the end of the value —
        // insertions collapse the selection to the end and every other edit path
        // bypasses the bridge — so no `move_cursor_to_end` extension is needed.
        let input_runtime = context.input_runtime();
        let otp_value = context.read(cx, |runtime, _props| runtime.value());
        let change_context = context.clone();
        let paste_context = context.clone();
        input_runtime.update(cx, |input, cx| {
            input.sync_props(
                Some(otp_value),
                disabled,
                self.read_only,
                self.required,
                Some(Rc::new(move |value, window, cx| {
                    change_context.input_edited(value, window, cx);
                })),
                None,
                None,
                None,
                Some(Rc::new(move |text, window, cx| {
                    paste_context.paste(text, window, cx);
                    true
                })),
                None,
                None,
                cx,
            );
        });

        let mut style_state =
            context.read(cx, |runtime, props| runtime.root_state(props, field_valid));
        if let Some(field_valid) = field_valid {
            style_state.valid = Some(field_valid);
            style_state.invalid = !field_valid;
        }

        if let Some(field_context) = field_context.as_ref() {
            let field_value = if style_state.value.is_empty() {
                FieldValue::Empty
            } else {
                FieldValue::Text(style_state.value.clone())
            };
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(field_value)
                .disabled(style_state.disabled)
                .focused(style_state.focused)
                .required(style_state.required)
                .focus_handle(focus_handle.clone());
            if let Some(name) = name {
                registration = registration.name(name);
            }
            field_context.register_control(registration, cx);
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let wired = wire_children(self.children, context.clone());
        #[cfg(debug_assertions)]
        if self.length > 0 && wired.input_count != self.length {
            eprintln!(
                "OTPFieldRoot: rendered {} OTPFieldInput children but `.length(...)` is {}",
                wired.input_count, self.length
            );
        }

        let previous_context = context.clone();
        let next_context = context.clone();
        let first_context = context.clone();
        let last_context = context.clone();
        let backspace_context = context.clone();
        let delete_context = context.clone();
        let clear_context = context.clone();
        let a11y_set_value_context = context.clone();

        // Accessibility: Base UI's root `role="group"`. `aria-labelledby` /
        // `aria-describedby` linkage to `FieldLabel` / `FieldDescription` /
        // `FieldError` has no relationship builders in this gpui revision, so
        // the literal `.aria_label(...)` prop stands in. `disabled`,
        // `read_only`, `required`, and `aria-invalid` have no a11y builders
        // either and are blocked pending gpui upstream; `Action::Focus` is
        // auto-registered by `.track_focus`.
        base.id(self.id)
            .role(Role::Group)
            .when_some(aria_label, |this, aria_label| this.aria_label(aria_label))
            .on_a11y_action(AccessibleAction::SetValue, move |data, window, cx| {
                if let Some(ActionData::Value(value)) = data {
                    a11y_set_value_context.paste(SharedString::from(value.to_string()), window, cx);
                }
            })
            .when(!disabled, |this| {
                this.track_focus(&focus_handle.tab_stop(true))
            })
            .key_context(OTP_FIELD_KEY_CONTEXT)
            .on_action(move |_: &OTPFieldPrevious, window, cx| {
                previous_context.move_active(OTPFieldMove::Previous, window, cx);
            })
            .on_action(move |_: &OTPFieldNext, window, cx| {
                next_context.move_active(OTPFieldMove::Next, window, cx);
            })
            .on_action(move |_: &OTPFieldFirst, window, cx| {
                first_context.move_active(OTPFieldMove::First, window, cx);
            })
            .on_action(move |_: &OTPFieldLast, window, cx| {
                last_context.move_active(OTPFieldMove::EndOfValue, window, cx);
            })
            .on_action(move |_: &OTPFieldBackspace, window, cx| {
                backspace_context.backspace(window, cx);
            })
            .on_action(move |_: &OTPFieldDelete, window, cx| {
                delete_context.delete(window, cx);
            })
            .on_action(move |_: &OTPFieldClear, window, cx| {
                clear_context.clear(window, cx);
            })
            .on_action(window.listener_for(&input_runtime, InputRuntime::paste))
            .children(wired.children)
    }
}

impl OTPFieldRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn child(mut self, child: impl Into<OTPFieldChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<OTPFieldChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(OTPFieldChild::Any(child.into_any_element()));
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

    /// Accessible name for the OTP group. Mirror the visible `FieldLabel`
    /// text here manually: relationship props (`aria-labelledby`) do not
    /// exist in this gpui revision.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn default_value(mut self, default_value: impl Into<SharedString>) -> Self {
        self.default_value = default_value.into();
        self
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn validation_type(mut self, validation_type: OTPFieldValidationType) -> Self {
        self.validation_type = validation_type;
        self
    }

    pub fn normalize_value(mut self, normalize_value: impl Fn(String) -> String + 'static) -> Self {
        self.normalize_value = Some(Rc::new(normalize_value));
        self
    }

    pub fn mask(mut self, mask: bool) -> Self {
        self.mask = mask;
        self
    }

    pub fn auto_submit(mut self, auto_submit: bool) -> Self {
        self.auto_submit = auto_submit;
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

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn on_value_complete(
        mut self,
        on_value_complete: impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_complete = Some(Rc::new(on_value_complete));
        self
    }

    pub fn on_value_invalid(
        mut self,
        on_value_invalid: impl Fn(SharedString, OTPFieldChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_invalid = Some(Rc::new(on_value_invalid));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(OTPFieldRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
