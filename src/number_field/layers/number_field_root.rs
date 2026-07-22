use std::{rc::Rc, sync::Arc};

use gpui::{
    div, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, ScrollDelta, SharedString, StyleRefinement, Styled, Window,
};

use crate::{
    field::{
        current_field_context, current_field_item_disabled, FieldContext, FieldControlRegistration,
        FieldValue,
    },
    fieldset::current_fieldset_disabled,
    number_field::{
        child_wiring::wire_children, format_number, NumberFieldChangeDetails,
        NumberFieldChangeReason, NumberFieldChild, NumberFieldCommitDetails,
        NumberFieldCommitReason, NumberFieldContext, NumberFieldMax, NumberFieldMin,
        NumberFieldProps, NumberFieldRootStyleState, NumberFieldStep, NumberFieldStepAmount,
        NumberFieldStepDirection, NumberFieldStepDown, NumberFieldStepDownLarge,
        NumberFieldStepDownSmall, NumberFieldStepUp, NumberFieldStepUpLarge,
        NumberFieldStepUpSmall, NumberFieldValueChangeHandler, NumberFieldValueCommitHandler,
        NUMBER_FIELD_KEY_CONTEXT,
    },
};

#[derive(IntoElement)]
pub struct NumberFieldRoot {
    id: ElementId,
    base: Div,
    children: Vec<NumberFieldChild>,
    context: Option<FieldContext>,
    name: Option<SharedString>,
    form: Option<SharedString>,
    default_value: Option<f64>,
    value: Option<Option<f64>>,
    min: Option<f64>,
    max: Option<f64>,
    step: NumberFieldStep,
    small_step: f64,
    large_step: f64,
    snap_on_step: bool,
    allow_out_of_range: bool,
    allow_wheel_scrub: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    on_value_change: Option<NumberFieldValueChangeHandler>,
    on_value_committed: Option<NumberFieldValueCommitHandler>,
    style_with_state: Option<Rc<dyn Fn(NumberFieldRootStyleState, Div) -> Div + 'static>>,
}

impl Default for NumberFieldRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("number-field"),
            base: div(),
            children: Vec::new(),
            context: None,
            name: None,
            form: None,
            default_value: None,
            value: None,
            min: None,
            max: None,
            step: NumberFieldStep::default(),
            small_step: 0.1,
            large_step: 10.0,
            snap_on_step: false,
            allow_out_of_range: false,
            allow_wheel_scrub: false,
            disabled: false,
            read_only: false,
            required: false,
            on_value_change: None,
            on_value_committed: None,
            style_with_state: None,
        }
    }
}

impl Styled for NumberFieldRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for NumberFieldRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = self.context.or_else(current_field_context);
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state.map(|state| state.disabled).unwrap_or(false);
        let field_valid = field_state.and_then(|state| state.valid);
        let item_disabled = current_field_item_disabled();
        let fieldset_disabled = current_fieldset_disabled();
        let disabled = self.disabled || field_disabled || item_disabled || fieldset_disabled;
        let id = self.id.clone();
        let name = self.name.clone();

        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();

        let context = NumberFieldContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            NumberFieldProps::new(
                self.name,
                self.form,
                self.min,
                self.max,
                self.step,
                self.small_step,
                self.large_step,
                self.snap_on_step,
                self.allow_out_of_range,
                self.allow_wheel_scrub,
                disabled,
                self.read_only,
                self.required,
                self.on_value_change,
                self.on_value_committed,
            ),
            focus_handle.clone(),
        );
        context.sync_focus(focus_handle.is_focused(window), window, cx);

        let mut style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        if let Some(field_valid) = field_valid {
            style_state.valid = Some(field_valid);
            style_state.invalid = !field_valid;
        }

        if let Some(field_context) = field_context.as_ref() {
            let field_value = match style_state.value {
                Some(value) => FieldValue::Text(format_number(Some(value))),
                None => FieldValue::Empty,
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

        let step_up = context.clone();
        let step_down = context.clone();
        let step_up_small = context.clone();
        let step_down_small = context.clone();
        let step_up_large = context.clone();
        let step_down_large = context.clone();
        let min_context = context.clone();
        let max_context = context.clone();
        let wheel_context = context.clone();

        // No a11y role on purpose: Base UI's root is a plain `div`. Without a role this
        // element stays out of the AccessKit tree, matching that behavior.
        base.id(self.id)
            .key_context(NUMBER_FIELD_KEY_CONTEXT)
            .on_action(move |_: &NumberFieldStepUp, window, cx| {
                step_up.step(
                    NumberFieldStepDirection::Up,
                    NumberFieldStepAmount::Normal,
                    NumberFieldChangeReason::Keyboard,
                    NumberFieldCommitReason::Keyboard,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &NumberFieldStepDown, window, cx| {
                step_down.step(
                    NumberFieldStepDirection::Down,
                    NumberFieldStepAmount::Normal,
                    NumberFieldChangeReason::Keyboard,
                    NumberFieldCommitReason::Keyboard,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &NumberFieldStepUpSmall, window, cx| {
                step_up_small.step(
                    NumberFieldStepDirection::Up,
                    NumberFieldStepAmount::Small,
                    NumberFieldChangeReason::Keyboard,
                    NumberFieldCommitReason::Keyboard,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &NumberFieldStepDownSmall, window, cx| {
                step_down_small.step(
                    NumberFieldStepDirection::Down,
                    NumberFieldStepAmount::Small,
                    NumberFieldChangeReason::Keyboard,
                    NumberFieldCommitReason::Keyboard,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &NumberFieldStepUpLarge, window, cx| {
                step_up_large.step(
                    NumberFieldStepDirection::Up,
                    NumberFieldStepAmount::Large,
                    NumberFieldChangeReason::Keyboard,
                    NumberFieldCommitReason::Keyboard,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &NumberFieldStepDownLarge, window, cx| {
                step_down_large.step(
                    NumberFieldStepDirection::Down,
                    NumberFieldStepAmount::Large,
                    NumberFieldChangeReason::Keyboard,
                    NumberFieldCommitReason::Keyboard,
                    window,
                    cx,
                );
            })
            .on_action(move |_: &NumberFieldMin, window, cx| {
                min_context.move_to_boundary(NumberFieldStepDirection::Down, window, cx);
            })
            .on_action(move |_: &NumberFieldMax, window, cx| {
                max_context.move_to_boundary(NumberFieldStepDirection::Up, window, cx);
            })
            .on_scroll_wheel(move |event, window, cx| {
                let allow_wheel_scrub =
                    wheel_context.read(cx, |_runtime, props| props.allow_wheel_scrub());
                if !allow_wheel_scrub || event.modifiers.control {
                    return;
                }

                let delta_y = match event.delta {
                    ScrollDelta::Pixels(delta) => delta.y.as_f32(),
                    ScrollDelta::Lines(delta) => delta.y,
                };
                if delta_y == 0.0 {
                    return;
                }

                let direction = if delta_y > 0.0 {
                    NumberFieldStepDirection::Down
                } else {
                    NumberFieldStepDirection::Up
                };
                wheel_context.step(
                    direction,
                    NumberFieldStepAmount::Normal,
                    NumberFieldChangeReason::Wheel,
                    NumberFieldCommitReason::Wheel,
                    window,
                    cx,
                );
            })
            .children(wire_children(self.children, context))
    }
}

impl NumberFieldRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn child(mut self, child: impl Into<NumberFieldChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<NumberFieldChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NumberFieldChild::Any(child.into_any_element()));
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

    pub fn form(mut self, form: impl Into<SharedString>) -> Self {
        self.form = Some(form.into());
        self
    }

    pub fn default_value(mut self, default_value: Option<f64>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn value(mut self, value: Option<f64>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = NumberFieldStep::amount(step);
        self
    }

    pub fn step_any(mut self) -> Self {
        self.step = NumberFieldStep::any();
        self
    }

    pub fn small_step(mut self, small_step: f64) -> Self {
        self.small_step = small_step;
        self
    }

    pub fn large_step(mut self, large_step: f64) -> Self {
        self.large_step = large_step;
        self
    }

    pub fn snap_on_step(mut self, snap_on_step: bool) -> Self {
        self.snap_on_step = snap_on_step;
        self
    }

    pub fn allow_out_of_range(mut self, allow_out_of_range: bool) -> Self {
        self.allow_out_of_range = allow_out_of_range;
        self
    }

    pub fn allow_wheel_scrub(mut self, allow_wheel_scrub: bool) -> Self {
        self.allow_wheel_scrub = allow_wheel_scrub;
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
        on_value_change: impl Fn(Option<f64>, NumberFieldChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn on_value_committed(
        mut self,
        on_value_committed: impl Fn(Option<f64>, NumberFieldCommitDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_committed = Some(Rc::new(on_value_committed));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NumberFieldRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
