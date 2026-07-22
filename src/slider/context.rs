use std::{rc::Rc, sync::Arc};

use gpui::{App, ElementId, Entity, Point, SharedString, Window};

use crate::slider::{
    SliderKeyboardStep, SliderProposal, SliderProps, SliderRuntime, SliderValueChangeDetails,
    SliderValueCommitDetails, SliderValues,
};
use crate::utils::TextDirection;

pub struct SliderContext {
    id: ElementId,
    runtime: Entity<SliderRuntime>,
    props: Rc<SliderProps>,
    controlled: Rc<Option<SliderValues>>,
}

impl Clone for SliderContext {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl SliderContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<SliderValues>,
        default: Option<SliderValues>,
        props: SliderProps,
    ) -> Self {
        let id = id.into();
        let initial = controlled.clone().or(default);
        let min = props.min();
        let max = props.max();
        let runtime =
            window.use_keyed_state(id.clone(), cx, |_, _| SliderRuntime::new(initial, min, max));
        let context = Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        };
        context.reconcile(cx);
        context
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&SliderRuntime, &SliderProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut SliderRuntime) -> Output,
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

    pub fn press_track(
        &self,
        position: Point<gpui::Pixels>,
        direction: TextDirection,
        window: &mut Window,
        cx: &mut App,
    ) {
        let proposal = self.update(cx, |runtime| {
            runtime.press_track(position, direction, self.props.as_ref())
        });
        let target = proposal.as_ref().map(|proposal| proposal.thumb_index);
        self.resolve_proposal(proposal, window, cx);
        if let Some(index) = target {
            let handle = self.read(cx, |runtime, _| runtime.thumb_focus_handle(index));
            if let Some(handle) = handle {
                handle.focus(window, cx);
            }
        }
    }

    pub fn press_thumb(
        &self,
        index: usize,
        position: Point<gpui::Pixels>,
        direction: TextDirection,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_target = self.update(cx, |runtime| {
            runtime.press_thumb(index, position, direction, self.props.as_ref())
        });
        if let Some(index) = focus_target {
            let handle = self.read(cx, |runtime, _| runtime.thumb_focus_handle(index));
            if let Some(handle) = handle {
                handle.focus(window, cx);
            }
        }
    }

    pub fn drag_to(
        &self,
        position: Point<gpui::Pixels>,
        direction: TextDirection,
        window: &mut Window,
        cx: &mut App,
    ) {
        let proposal = self.update(cx, |runtime| {
            runtime.drag_to(position, direction, self.props.as_ref())
        });
        self.resolve_proposal(proposal, window, cx);
    }

    pub fn release(&self, window: &mut Window, cx: &mut App) {
        let commit = self.update(cx, |runtime| runtime.release());
        self.emit_commit(commit, window, cx);
    }

    pub fn keyboard_step(
        &self,
        index: usize,
        step: SliderKeyboardStep,
        window: &mut Window,
        cx: &mut App,
    ) {
        let proposal = self.update(cx, |runtime| {
            runtime.keyboard_step(index, step, self.props.as_ref())
        });
        self.resolve_proposal(proposal, window, cx);
    }

    /// Routes an AccessKit `SetValue` request through the same
    /// neighbor-clamped path as keyboard steps.
    pub fn set_thumb_value(&self, index: usize, value: f64, window: &mut Window, cx: &mut App) {
        let proposal = self.update(cx, |runtime| {
            runtime.set_thumb_value(index, value, self.props.as_ref())
        });
        self.resolve_proposal(proposal, window, cx);
    }

    pub fn sync_thumb_focused(&self, index: usize, focused: bool, cx: &mut App) {
        self.update(cx, |runtime| runtime.sync_thumb_focused(index, focused));
    }

    /// Resolves controlled vs uncontrolled, fires the cancelable change
    /// callback, applies the proposal when not canceled, and commits
    /// immediately for keyboard changes.
    fn resolve_proposal(
        &self,
        proposal: Option<SliderProposal>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(proposal) = proposal else {
            return;
        };

        let single = self.read(cx, |runtime, _| runtime.is_single());
        let proposed = SliderValues::from_vec(&proposal.values, single);
        let mut details = SliderValueChangeDetails::new(proposal.reason, proposal.thumb_index);
        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(proposed, &mut details, window, cx);
        }
        if details.is_canceled() {
            return;
        }

        let controlled = self.controlled.is_some();
        let commit_immediately = proposal.commit_immediately;
        let focus_target = self.update(cx, |runtime| {
            let focus_target = runtime.apply_proposal(&proposal, !controlled);
            if commit_immediately {
                (focus_target, runtime.take_pending_commit())
            } else {
                (focus_target, None)
            }
        });
        if let Some(index) = focus_target.0 {
            let handle = self.read(cx, |runtime, _| runtime.thumb_focus_handle(index));
            if let Some(handle) = handle {
                handle.focus(window, cx);
            }
        }
        self.emit_commit(focus_target.1, window, cx);
    }

    fn emit_commit(
        &self,
        commit: Option<(Vec<f64>, crate::slider::SliderChangeReason)>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some((values, reason)) = commit else {
            return;
        };
        let single = self.read(cx, |runtime, _| runtime.is_single());
        if let Some(on_value_committed) = self.props.on_value_committed() {
            on_value_committed(
                SliderValues::from_vec(&values, single),
                SliderValueCommitDetails::new(reason),
                window,
                cx,
            );
        }
    }

    fn reconcile_runtime(&self, runtime: &mut SliderRuntime) {
        match self.controlled.as_ref() {
            Some(values) => runtime.reconcile(Some(values.clone()), true, self.props.as_ref()),
            None => runtime.reconcile(None, false, self.props.as_ref()),
        }
    }
}
