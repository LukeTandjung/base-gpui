use std::{cell::RefCell, rc::Rc, sync::Arc};

use gpui::{
    div, prelude::*, px, size, App, Bounds, Div, ElementId, Entity, FocusHandle, IntoElement,
    Pixels, Render, RenderOnce, SharedString, StyleRefinement, Styled, TestAppContext,
    VisualTestContext, Window, WindowHandle,
};

use crate::field::{
    current_field_context, current_field_item_disabled, FieldContext, FieldControlRegistration,
    FieldDescription, FieldDescriptionStyleState, FieldError, FieldErrorStyleState, FieldItem,
    FieldItemStyleState, FieldLabel, FieldLabelStyleState, FieldRoot, FieldRootStyleState,
    FieldValidationMode, FieldValidationResult, FieldValidity, FieldValidityKey,
    FieldValidityStyleState, FieldValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldTestValidation {
    None,
    ErrorWhenEmpty,
    MultipleErrors,
}

impl Default for FieldTestValidation {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
pub struct FieldTestConfig {
    pub value: SharedString,
    pub control_disabled: bool,
    pub control_required: bool,
    pub root_disabled: bool,
    pub root_invalid: Option<bool>,
    pub root_dirty: Option<bool>,
    pub root_touched: Option<bool>,
    pub validation_mode: FieldValidationMode,
    pub validation: FieldTestValidation,
    pub item_disabled: bool,
    pub wrap_control_in_item: bool,
    pub error_match: Option<FieldValidityKey>,
    pub error_always: bool,
}

impl Default for FieldTestConfig {
    fn default() -> Self {
        Self {
            value: SharedString::default(),
            control_disabled: false,
            control_required: false,
            root_disabled: false,
            root_invalid: None,
            root_dirty: None,
            root_touched: None,
            validation_mode: FieldValidationMode::OnSubmit,
            validation: FieldTestValidation::None,
            item_disabled: false,
            wrap_control_in_item: false,
            error_match: None,
            error_always: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct FieldObservations {
    pub root_states: Vec<FieldRootStyleState>,
    pub item_states: Vec<FieldItemStyleState>,
    pub label_states: Vec<FieldLabelStyleState>,
    pub description_states: Vec<FieldDescriptionStyleState>,
    pub error_states: Vec<FieldErrorStyleState>,
    pub validity_states: Vec<FieldValidityStyleState>,
    pub control_disabled: Vec<bool>,
    pub control_focused: Vec<bool>,
}

impl FieldObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.item_states.clear();
        self.label_states.clear();
        self.description_states.clear();
        self.error_states.clear();
        self.validity_states.clear();
        self.control_disabled.clear();
        self.control_focused.clear();
    }

    pub fn root_state(&self) -> FieldRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("field root state should be observed")
    }

    pub fn label_state(&self) -> FieldLabelStyleState {
        self.label_states
            .last()
            .copied()
            .expect("field label state should be observed")
    }

    pub fn description_state(&self) -> FieldDescriptionStyleState {
        self.description_states
            .last()
            .copied()
            .expect("field description state should be observed")
    }

    pub fn item_state(&self) -> FieldItemStyleState {
        self.item_states
            .last()
            .copied()
            .expect("field item state should be observed")
    }

    pub fn error_state(&self) -> Option<FieldErrorStyleState> {
        self.error_states.last().cloned()
    }

    pub fn validity_state(&self) -> FieldValidityStyleState {
        self.validity_states
            .last()
            .cloned()
            .expect("field validity state should be observed")
    }

    pub fn last_control_disabled(&self) -> bool {
        *self
            .control_disabled
            .last()
            .expect("field test control should be observed")
    }

    pub fn last_control_focused(&self) -> bool {
        *self
            .control_focused
            .last()
            .expect("field test control should be observed")
    }
}

pub struct FieldTestView {
    pub config: FieldTestConfig,
    observations: Rc<RefCell<FieldObservations>>,
    validation_context: Rc<RefCell<Option<FieldContext>>>,
}

impl FieldTestView {
    pub fn new(config: FieldTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(FieldObservations::default())),
            validation_context: Rc::new(RefCell::new(None)),
        }
    }

    pub fn read_observations(&self) -> FieldObservations {
        self.observations.borrow().clone()
    }
}

impl Render for FieldTestView {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();
        self.validation_context.borrow_mut().take();

        let mut root = FieldRoot::new()
            .id("field-test")
            .disabled(self.config.root_disabled)
            .validation_mode(self.config.validation_mode)
            .flex()
            .flex_col()
            .gap_2();

        if let Some(invalid) = self.config.root_invalid {
            root = root.invalid(invalid);
        }
        if let Some(dirty) = self.config.root_dirty {
            root = root.dirty(dirty);
        }
        if let Some(touched) = self.config.root_touched {
            root = root.touched(touched);
        }

        match self.config.validation {
            FieldTestValidation::None => {}
            FieldTestValidation::ErrorWhenEmpty => {
                root = root.validate(|value, _window, _cx| match value.filled() {
                    true => FieldValidationResult::Valid,
                    false => FieldValidationResult::Error(SharedString::from("Required")),
                });
            }
            FieldTestValidation::MultipleErrors => {
                root = root.validate(|_value, _window, _cx| {
                    FieldValidationResult::Errors(vec![
                        SharedString::from("First"),
                        SharedString::from("Second"),
                    ])
                });
            }
        }

        let root_observations = Rc::clone(&self.observations);
        root = root.style_with_state(move |state, root| {
            root_observations.borrow_mut().root_states.push(state);
            root.debug_selector(|| "field-root".into())
        });

        let label_observations = Rc::clone(&self.observations);
        let label = FieldLabel::new().style_with_state(move |state, label| {
            label_observations.borrow_mut().label_states.push(state);
            label.size(px(24.0)).debug_selector(|| "field-label".into())
        });

        let description_observations = Rc::clone(&self.observations);
        let description = FieldDescription::new().style_with_state(move |state, description| {
            description_observations
                .borrow_mut()
                .description_states
                .push(state);
            description.debug_selector(|| "field-description".into())
        });

        let error_observations = Rc::clone(&self.observations);
        let mut error = FieldError::new().style_with_state(move |state, error| {
            error_observations.borrow_mut().error_states.push(state);
            error.debug_selector(|| "field-error".into())
        });
        if let Some(key) = self.config.error_match {
            error = error.match_(key);
        }
        if self.config.error_always {
            error = error.match_always(true);
        }

        let validity_observations = Rc::clone(&self.observations);
        let validity = FieldValidity::new().style_with_state(move |state, validity| {
            validity_observations
                .borrow_mut()
                .validity_states
                .push(state);
            validity.debug_selector(|| "field-validity".into())
        });

        let control = FieldTestControl::new(
            "field-control",
            self.config.value.clone(),
            self.config.control_disabled,
            self.config.control_required,
            &self.observations,
        );

        root = root.child(label);
        if self.config.wrap_control_in_item {
            let item_observations = Rc::clone(&self.observations);
            root = root.child(
                FieldItem::new()
                    .disabled(self.config.item_disabled)
                    .style_with_state(move |state, item| {
                        item_observations.borrow_mut().item_states.push(state);
                        item.debug_selector(|| "field-item".into())
                    })
                    .child_any(control),
            );
        } else {
            root = root.child_any(control);
        }
        root = root
            .child(description)
            .child(error)
            .child(validity)
            .child_any(FieldValidationProbe::new(&self.validation_context));

        div().size_full().p_4().child(root)
    }
}

#[derive(IntoElement)]
struct FieldTestControl {
    id: ElementId,
    value: SharedString,
    disabled: bool,
    required: bool,
    observations: Rc<RefCell<FieldObservations>>,
    base: Div,
}

impl FieldTestControl {
    fn new(
        id: impl Into<ElementId>,
        value: SharedString,
        disabled: bool,
        required: bool,
        observations: &Rc<RefCell<FieldObservations>>,
    ) -> Self {
        Self {
            id: id.into(),
            value,
            disabled,
            required,
            observations: Rc::clone(observations),
            base: div().size(px(24.0)),
        }
    }
}

impl Styled for FieldTestControl {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldTestControl {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_disabled = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props).disabled))
            .unwrap_or(false);
        let disabled = self.disabled || field_disabled || current_field_item_disabled();
        let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
            ElementId::NamedChild(Arc::new(self.id.clone()), SharedString::from("focus")),
            cx,
            |_, cx| cx.focus_handle(),
        );
        let focus_handle = focus_handle_entity.read(cx).clone();
        let focused = focus_handle.is_focused(window);

        if let Some(context) = field_context.as_ref() {
            context.register_control(
                FieldControlRegistration::new(self.id.to_string())
                    .value(FieldValue::Text(self.value.clone()))
                    .disabled(disabled)
                    .focused(focused)
                    .required(self.required)
                    .focus_handle(focus_handle.clone()),
                cx,
            );
        }

        {
            let mut observations = self.observations.borrow_mut();
            observations.control_disabled.push(disabled);
            observations.control_focused.push(focused);
        }

        self.base
            .id(self.id)
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .focusable()
            .debug_selector(|| "field-control".into())
    }
}

#[derive(IntoElement)]
struct FieldValidationProbe {
    context: Rc<RefCell<Option<FieldContext>>>,
}

impl FieldValidationProbe {
    fn new(context: &Rc<RefCell<Option<FieldContext>>>) -> Self {
        Self {
            context: Rc::clone(context),
        }
    }
}

impl RenderOnce for FieldValidationProbe {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        *self.context.borrow_mut() = current_field_context();

        div().size(px(0.0))
    }
}

pub fn open_field(cx: &mut TestAppContext, config: FieldTestConfig) -> WindowHandle<FieldTestView> {
    let window = cx.open_window(size(px(360.0), px(220.0)), move |_, _| {
        FieldTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<FieldTestView>,
) -> FieldObservations {
    window
        .update(cx, |_view, _window, cx| cx.notify())
        .expect("field test window should be open");
    cx.run_until_parked();

    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("field test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<FieldTestView>,
    update: impl FnOnce(&mut FieldTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_next(cx: &mut TestAppContext, window: WindowHandle<FieldTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn validate_manually(cx: &mut TestAppContext, window: WindowHandle<FieldTestView>) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .validation_context
                .borrow()
                .clone()
                .expect("field validation context should be captured");
            context.validate(window, cx);
        })
        .expect("field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn blur(cx: &mut TestAppContext, window: WindowHandle<FieldTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<FieldTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
