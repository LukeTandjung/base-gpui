use std::{rc::Rc, sync::Arc};

use gpui::{App, ElementId, Entity, FocusHandle, SharedString, Window};

use crate::number_field::{
    NumberFieldCommitReason, NumberFieldProps, NumberFieldRuntime, NumberFieldScrubDirection,
    NumberFieldStepAmount, NumberFieldStepDirection, NumberFieldUpdateOutcome,
};

pub struct NumberFieldContext {
    id: ElementId,
    runtime: Entity<NumberFieldRuntime>,
    props: Rc<NumberFieldProps>,
    controlled: Rc<Option<Option<f64>>>,
    focus_handle: FocusHandle,
}

impl Clone for NumberFieldContext {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
            focus_handle: self.focus_handle.clone(),
        }
    }
}

impl NumberFieldContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<f64>>,
        default: Option<f64>,
        props: NumberFieldProps,
        focus_handle: FocusHandle,
    ) -> Self {
        let id = id.into();
        let initial_value = controlled.unwrap_or(default);
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            NumberFieldRuntime::new(initial_value)
        });
        let context = Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
            focus_handle,
        };
        context.reconcile(cx);
        context
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&NumberFieldRuntime, &NumberFieldProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut NumberFieldRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            self.reconcile_runtime(runtime);
            let output = update(runtime);
            cx.notify();
            output
        })
    }

    pub fn reconcile(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            self.reconcile_runtime(runtime);
            cx.notify();
        });
    }

    pub fn id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn child_id(&self, child: impl Into<SharedString>) -> ElementId {
        ElementId::NamedChild(Arc::new(self.id.clone()), child.into())
    }

    pub fn focus_handle(&self) -> FocusHandle {
        self.focus_handle.clone()
    }

    pub fn input_changed(&self, value: SharedString, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| {
            runtime.input_changed(value, self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn sync_focus(&self, focused: bool, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| {
            runtime.sync_focused(focused, self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn commit(&self, reason: NumberFieldCommitReason, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| {
            runtime.commit_input(reason, self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn step(
        &self,
        direction: NumberFieldStepDirection,
        amount: NumberFieldStepAmount,
        change_reason: crate::number_field::NumberFieldChangeReason,
        commit_reason: NumberFieldCommitReason,
        window: &mut Window,
        cx: &mut App,
    ) {
        let outcome = self.update(cx, |runtime| {
            runtime.step_by(
                direction,
                amount,
                change_reason,
                commit_reason,
                self.props.as_ref(),
            )
        });
        self.emit(outcome, window, cx);
    }

    pub fn move_to_boundary(
        &self,
        direction: NumberFieldStepDirection,
        window: &mut Window,
        cx: &mut App,
    ) {
        let outcome = self.update(cx, |runtime| {
            runtime.move_to_boundary(direction, self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn set_scrubbing(&self, scrubbing: bool, window: &mut Window, cx: &mut App) {
        let outcome = self.update(cx, |runtime| {
            runtime.set_scrubbing(scrubbing, self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    pub fn scrub_by_pixels(
        &self,
        pixels: f64,
        pixel_sensitivity: f64,
        _direction: NumberFieldScrubDirection,
        window: &mut Window,
        cx: &mut App,
    ) {
        let outcome = self.update(cx, |runtime| {
            runtime.scrub_by_pixels(pixels, pixel_sensitivity, self.props.as_ref())
        });
        self.emit(outcome, window, cx);
    }

    fn reconcile_runtime(&self, runtime: &mut NumberFieldRuntime) {
        match *self.controlled {
            Some(value) => runtime.reconcile(value, true),
            None => runtime.reconcile(runtime.value(), false),
        }
    }

    fn emit(&self, outcome: NumberFieldUpdateOutcome, window: &mut Window, cx: &mut App) {
        if let Some(change) = outcome.change {
            if let Some(on_value_change) = self.props.on_value_change() {
                on_value_change(change.value, change.details, window, cx);
            }
        }

        if let Some(commit) = outcome.commit {
            if let Some(on_value_committed) = self.props.on_value_committed() {
                on_value_committed(commit.value, commit.details, window, cx);
            }
        }
    }
}
