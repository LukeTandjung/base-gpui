use gpui::{Bounds, FocusHandle, Pixels, Point, SharedString};

use crate::slider::{
    clamp, fraction_to_value, get_new_value, get_slider_value, position_to_fraction,
    resolve_thumb_collision, round_value_to_step, validate_minimum_distance, value_to_fraction,
    SliderControlStyleState, SliderIndicatorStyleState, SliderLabelStyleState, SliderProps,
    SliderRootStyleState, SliderThumbStyleState, SliderTrackStyleState, SliderValueStyleState,
};
use crate::utils::TextDirection;

/// Base UI applies `dragging` only after more than this many move events for a
/// thumb press (`INTENTIONAL_DRAG_COUNT_THRESHOLD`).
const INTENTIONAL_DRAG_COUNT_THRESHOLD: u32 = 2;

/// The public slider value: a scalar slider or an N-thumb range.
#[derive(Clone, Debug, PartialEq)]
pub enum SliderValues {
    Single(f64),
    Range(Vec<f64>),
}

impl SliderValues {
    /// Normalizes to the internal sorted ascending vector, clamped into
    /// `[min, max]` for the single case, with non-finite entries dropped.
    pub fn to_normalized_vec(&self, min: f64, max: f64) -> Vec<f64> {
        match self {
            Self::Single(value) => {
                let value = if value.is_finite() { *value } else { min };
                Vec::from([clamp(value, min, max)])
            }
            Self::Range(values) => {
                let mut values = values
                    .iter()
                    .copied()
                    .filter(|value| value.is_finite())
                    .collect::<Vec<_>>();
                values.sort_by(|left, right| {
                    left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
                });
                values
            }
        }
    }

    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    /// Rebuilds the caller-facing shape from the internal vector.
    pub fn from_vec(values: &[f64], single: bool) -> Self {
        if single {
            Self::Single(values.first().copied().unwrap_or(0.0))
        } else {
            Self::Range(values.to_vec())
        }
    }
}

/// The slider orientation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// How thumbs are aligned against the control's ends.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderThumbAlignment {
    #[default]
    Center,
    Edge,
}

/// Why a value change happened.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SliderChangeReason {
    TrackPress,
    Drag,
    Keyboard,
    #[default]
    None,
}

/// Cancelable details passed to `on_value_change`.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderValueChangeDetails {
    reason: SliderChangeReason,
    thumb_index: usize,
    canceled: bool,
}

impl SliderValueChangeDetails {
    pub fn new(reason: SliderChangeReason, thumb_index: usize) -> Self {
        Self {
            reason,
            thumb_index,
            canceled: false,
        }
    }

    pub fn reason(&self) -> SliderChangeReason {
        self.reason
    }

    pub fn thumb_index(&self) -> usize {
        self.thumb_index
    }

    pub fn cancel(&mut self) {
        self.canceled = true;
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }
}

/// Non-cancelable details passed to `on_value_committed`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SliderValueCommitDetails {
    reason: SliderChangeReason,
}

impl SliderValueCommitDetails {
    pub fn new(reason: SliderChangeReason) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> SliderChangeReason {
        self.reason
    }
}

/// Signed keyboard step resolved by the thumb layer from orientation/RTL.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SliderKeyboardStep {
    Increment,
    Decrement,
    LargeIncrement,
    LargeDecrement,
    Home,
    End,
}

/// Per-thumb metadata synced from child wiring.
#[derive(Clone, Debug)]
pub struct SliderThumbMeta {
    pub disabled: bool,
    pub focus_handle: FocusHandle,
}

/// A proposed value change awaiting callback approval; the runtime never fires
/// user callbacks itself.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderProposal {
    pub values: Vec<f64>,
    pub reason: SliderChangeReason,
    pub thumb_index: usize,
    pub new_pressed_index: Option<usize>,
    pub did_swap: bool,
    pub commit_immediately: bool,
}

/// The deep slider module: normalized sorted values, interaction state,
/// measured geometry, and per-thumb focus handles.
#[derive(Clone, Debug)]
pub struct SliderRuntime {
    values: Vec<f64>,
    single: bool,
    initial_values: Vec<f64>,
    controlled: bool,
    thumbs: Vec<SliderThumbMeta>,
    active_thumb: Option<usize>,
    last_used_thumb: Option<usize>,
    focused_thumb: Option<usize>,
    pressed_thumb: Option<usize>,
    thumb_offset: f64,
    track_pressed: bool,
    move_count: u32,
    dragging: bool,
    drag_start_values: Vec<f64>,
    last_change_reason: SliderChangeReason,
    pending_commit_values: Option<Vec<f64>>,
    touched: bool,
    control_bounds: Option<Bounds<Pixels>>,
    thumb_bounds: Vec<Option<Bounds<Pixels>>>,
}

impl SliderRuntime {
    /// Creates the runtime from the uncontrolled default (falling back to
    /// `Single(min)` when absent).
    pub fn new(default_value: Option<SliderValues>, min: f64, max: f64) -> Self {
        let default_value = default_value.unwrap_or(SliderValues::Single(min));
        let single = default_value.is_single();
        let values = default_value.to_normalized_vec(min, max);

        Self {
            initial_values: values.clone(),
            values,
            single,
            controlled: false,
            thumbs: Vec::from([]),
            active_thumb: None,
            last_used_thumb: None,
            focused_thumb: None,
            pressed_thumb: None,
            thumb_offset: 0.0,
            track_pressed: false,
            move_count: 0,
            dragging: false,
            drag_start_values: Vec::from([]),
            last_change_reason: SliderChangeReason::None,
            pending_commit_values: None,
            touched: false,
            control_bounds: None,
            thumb_bounds: Vec::from([]),
        }
    }

    /// Reconciles with the controlled value (single transition-resolution point).
    pub fn reconcile(
        &mut self,
        observed: Option<SliderValues>,
        controlled: bool,
        props: &SliderProps,
    ) {
        self.controlled = controlled;
        let Some(observed) = observed else {
            return;
        };
        if !controlled {
            return;
        }

        let single = observed.is_single();
        let values = observed.to_normalized_vec(props.min(), props.max());
        if self.single != single || !values_equal(&self.values, &values) {
            self.single = single;
            self.values = values;
        }
    }

    /// Syncs per-thumb metadata and focus handles from child wiring.
    pub fn sync_thumbs(&mut self, thumbs: Vec<SliderThumbMeta>) {
        self.thumb_bounds.resize(thumbs.len(), None);
        self.thumbs = thumbs;
        if self
            .pressed_thumb
            .map(|index| index >= self.thumbs.len())
            .unwrap_or(false)
        {
            self.clear_interaction();
        }
    }

    /// Clears transient interaction state when the slider becomes disabled.
    pub fn sync_disabled(&mut self, disabled: bool) {
        if disabled {
            self.clear_interaction();
            self.active_thumb = None;
        }
    }

    /// Records the measured control content bounds; returns whether it changed.
    pub fn set_control_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.control_bounds == Some(bounds) {
            return false;
        }
        self.control_bounds = Some(bounds);
        true
    }

    /// Records measured thumb bounds by thumb index; returns whether any changed.
    pub fn set_thumb_bounds(&mut self, bounds: Vec<(usize, Bounds<Pixels>)>) -> bool {
        let mut changed = false;
        for (index, thumb_bounds) in bounds {
            if index >= self.thumb_bounds.len() {
                self.thumb_bounds.resize(index + 1, None);
            }
            if self.thumb_bounds[index] != Some(thumb_bounds) {
                self.thumb_bounds[index] = Some(thumb_bounds);
                changed = true;
            }
        }
        changed
    }

    /// Press on the control (not a thumb): selects the nearest enabled thumb by
    /// main-axis midpoint distance (ties to the later index) and proposes an
    /// immediate `TrackPress` change. Begins dragging immediately.
    pub fn press_track(
        &mut self,
        position: Point<Pixels>,
        direction: TextDirection,
        props: &SliderProps,
    ) -> Option<SliderProposal> {
        if props.disabled() {
            return None;
        }

        let next_value = self.position_to_value(position, direction, props)?;
        let target = self.closest_enabled_thumb(position, direction, props)?;

        self.pressed_thumb = Some(target);
        self.track_pressed = true;
        self.move_count = 0;
        self.dragging = true;
        self.thumb_offset = 0.0;
        self.drag_start_values = self.values.clone();

        self.propose_pointer_value(next_value, target, SliderChangeReason::TrackPress, props)
    }

    /// Press on a thumb: records the pressed index and pointer-to-thumb-center
    /// offset without applying a value change. Applies the stacked-at-max
    /// walk-back rule. Returns the index that should receive focus.
    pub fn press_thumb(
        &mut self,
        index: usize,
        position: Point<Pixels>,
        direction: TextDirection,
        props: &SliderProps,
    ) -> Option<usize> {
        if props.disabled() || self.thumb_disabled(index) {
            self.clear_interaction();
            return None;
        }

        let mut index = index.min(self.values.len().saturating_sub(1));
        if self.values.get(index).copied() == Some(props.max()) {
            while index > 0 && self.values[index - 1] == self.values[index] {
                index -= 1;
            }
        }

        let horizontal = props.orientation() == SliderOrientation::Horizontal;
        self.thumb_offset = self
            .thumb_bounds
            .get(index)
            .copied()
            .flatten()
            .map(|bounds| {
                if horizontal {
                    f64::from((position.x - bounds.center().x).as_f32())
                } else {
                    f64::from((position.y - bounds.center().y).as_f32())
                }
            })
            .unwrap_or(0.0);
        let _ = direction;

        self.pressed_thumb = Some(index);
        self.track_pressed = false;
        self.move_count = 0;
        self.dragging = false;
        self.drag_start_values = self.values.clone();
        Some(index)
    }

    /// A pointer move during an active press: converts position to value with
    /// the recorded center offset and proposes a `Drag` change.
    pub fn drag_to(
        &mut self,
        position: Point<Pixels>,
        direction: TextDirection,
        props: &SliderProps,
    ) -> Option<SliderProposal> {
        let pressed = self.pressed_thumb?;
        if props.disabled() {
            return None;
        }

        self.move_count += 1;
        if !self.track_pressed && self.move_count > INTENTIONAL_DRAG_COUNT_THRESHOLD {
            self.dragging = true;
        }

        let horizontal = props.orientation() == SliderOrientation::Horizontal;
        let adjusted = if horizontal {
            Point::new(position.x - gpui::px(self.thumb_offset as f32), position.y)
        } else {
            Point::new(position.x, position.y - gpui::px(self.thumb_offset as f32))
        };
        let next_value = self.position_to_value(adjusted, direction, props)?;

        self.propose_pointer_value(next_value, pressed, SliderChangeReason::Drag, props)
    }

    /// Ends the interaction; returns the values to commit when a change was
    /// applied during the interaction.
    pub fn release(&mut self) -> Option<(Vec<f64>, SliderChangeReason)> {
        self.pressed_thumb?;

        self.clear_interaction();
        self.active_thumb = self.focused_thumb;
        let reason = self.last_change_reason;
        self.pending_commit_values
            .take()
            .map(|values| (values, reason))
    }

    /// A keyboard step on a thumb: neighbor-clamped path, never pushes or swaps.
    pub fn keyboard_step(
        &mut self,
        index: usize,
        step: SliderKeyboardStep,
        props: &SliderProps,
    ) -> Option<SliderProposal> {
        if props.disabled() || self.thumb_disabled(index) {
            return None;
        }
        let current = self.values.get(index).copied()?;

        let min_distance = props.step() * props.min_steps_between_values();
        let next_value = match step {
            SliderKeyboardStep::Increment => get_new_value(
                current,
                props.step(),
                props.step(),
                props.min(),
                props.max(),
            ),
            SliderKeyboardStep::Decrement => get_new_value(
                current,
                -props.step(),
                props.step(),
                props.min(),
                props.max(),
            ),
            SliderKeyboardStep::LargeIncrement => get_new_value(
                current,
                props.large_step(),
                props.step(),
                props.min(),
                props.max(),
            ),
            SliderKeyboardStep::LargeDecrement => get_new_value(
                current,
                -props.large_step(),
                props.step(),
                props.min(),
                props.max(),
            ),
            SliderKeyboardStep::Home => {
                let mut target = props.min();
                if index > 0 {
                    if let Some(previous) = self.values.get(index - 1) {
                        target = target.max(previous + min_distance);
                    }
                }
                target
            }
            SliderKeyboardStep::End => {
                let mut target = props.max();
                if let Some(next) = self.values.get(index + 1) {
                    target = target.min(next - min_distance);
                }
                target
            }
        };

        if !next_value.is_finite() {
            return None;
        }

        let next_values =
            get_slider_value(next_value, index, props.min(), props.max(), &self.values);
        if !validate_minimum_distance(&next_values, props.step(), props.min_steps_between_values())
        {
            return None;
        }
        if values_equal(&next_values, &self.values) {
            return None;
        }

        Some(SliderProposal {
            values: next_values,
            reason: SliderChangeReason::Keyboard,
            thumb_index: index,
            new_pressed_index: None,
            did_swap: false,
            commit_immediately: true,
        })
    }

    /// An AT-requested absolute value for one thumb (AccessKit `SetValue`):
    /// same neighbor-clamped path as keyboard steps, never pushes or swaps.
    pub fn set_thumb_value(
        &mut self,
        index: usize,
        value: f64,
        props: &SliderProps,
    ) -> Option<SliderProposal> {
        if props.disabled() || self.thumb_disabled(index) {
            return None;
        }
        self.values.get(index)?;
        if !value.is_finite() {
            return None;
        }

        let next_value = value.clamp(props.min(), props.max());
        let next_values =
            get_slider_value(next_value, index, props.min(), props.max(), &self.values);
        if !validate_minimum_distance(&next_values, props.step(), props.min_steps_between_values())
        {
            return None;
        }
        if values_equal(&next_values, &self.values) {
            return None;
        }

        Some(SliderProposal {
            values: next_values,
            reason: SliderChangeReason::Keyboard,
            thumb_index: index,
            new_pressed_index: None,
            did_swap: false,
            commit_immediately: true,
        })
    }

    /// Applies an approved proposal. Mutates internal values only when
    /// uncontrolled; always records interaction bookkeeping. Returns the thumb
    /// index that should receive focus after a swap.
    pub fn apply_proposal(
        &mut self,
        proposal: &SliderProposal,
        mutate_values: bool,
    ) -> Option<usize> {
        if mutate_values {
            self.values = proposal.values.clone();
        }
        self.active_thumb = Some(proposal.thumb_index);
        self.last_used_thumb = Some(proposal.thumb_index);
        self.last_change_reason = proposal.reason;
        self.pending_commit_values = Some(proposal.values.clone());
        self.touched = true;
        if let Some(new_pressed_index) = proposal.new_pressed_index {
            self.pressed_thumb = Some(new_pressed_index);
        }
        if proposal.did_swap {
            Some(proposal.thumb_index)
        } else {
            None
        }
    }

    /// Takes the pending commit values for an immediately committing change.
    pub fn take_pending_commit(&mut self) -> Option<(Vec<f64>, SliderChangeReason)> {
        let reason = self.last_change_reason;
        self.pending_commit_values
            .take()
            .map(|values| (values, reason))
    }

    /// Records a thumb focus transition.
    pub fn sync_thumb_focused(&mut self, index: usize, focused: bool) {
        if focused {
            self.focused_thumb = Some(index);
            self.active_thumb = Some(index);
            self.last_used_thumb = Some(index);
        } else if self.focused_thumb == Some(index) {
            self.focused_thumb = None;
            if self.pressed_thumb.is_none() {
                self.active_thumb = None;
            }
            self.touched = true;
        }
    }

    /// The focus handle for a thumb, when synced.
    pub fn thumb_focus_handle(&self, index: usize) -> Option<FocusHandle> {
        self.thumbs
            .get(index)
            .map(|thumb| thumb.focus_handle.clone())
    }

    /// The focus handle Field label clicks should target: only when exactly one
    /// thumb exists (Base UI falls back only with exactly one input).
    pub fn single_thumb_focus_handle(&self) -> Option<FocusHandle> {
        if self.thumbs.len() == 1 {
            self.thumb_focus_handle(0)
        } else {
            None
        }
    }

    /// The current values in the caller-facing shape.
    pub fn current_values(&self) -> SliderValues {
        SliderValues::from_vec(&self.values, self.single)
    }

    pub fn is_single(&self) -> bool {
        self.single
    }

    pub fn any_thumb_focused(&self) -> bool {
        self.focused_thumb.is_some()
    }

    pub fn touched(&self) -> bool {
        self.touched
    }

    pub fn dirty(&self) -> bool {
        !values_equal(&self.values, &self.initial_values)
    }

    pub fn thumb_count(&self) -> usize {
        self.thumbs.len()
    }

    pub fn root_state(&self, props: &SliderProps) -> SliderRootStyleState {
        SliderRootStyleState {
            values: self.current_values(),
            min: props.min(),
            max: props.max(),
            step: props.step(),
            orientation: props.orientation(),
            disabled: props.disabled(),
            dragging: self.dragging,
            active_thumb_index: self.active_thumb,
            focused: self.focused_thumb.is_some(),
            touched: self.touched,
            dirty: self.dirty(),
        }
    }

    pub fn control_state(&self, props: &SliderProps) -> SliderControlStyleState {
        SliderControlStyleState {
            root: self.root_state(props),
        }
    }

    pub fn track_state(&self, props: &SliderProps) -> SliderTrackStyleState {
        SliderTrackStyleState {
            root: self.root_state(props),
        }
    }

    pub fn indicator_state(&self, props: &SliderProps) -> SliderIndicatorStyleState {
        let (start_fraction, end_fraction) = if self.values.len() > 1 {
            (
                self.alignment_fraction(self.values.first().copied().unwrap_or(props.min()), props),
                self.alignment_fraction(self.values.last().copied().unwrap_or(props.min()), props),
            )
        } else {
            (
                0.0,
                self.alignment_fraction(self.values.first().copied().unwrap_or(props.min()), props),
            )
        };

        SliderIndicatorStyleState {
            root: self.root_state(props),
            start_fraction,
            end_fraction,
            positioned: self.alignment_positioned(props),
        }
    }

    pub fn thumb_state(&self, index: usize, props: &SliderProps) -> SliderThumbStyleState {
        let value = self.values.get(index).copied().unwrap_or(props.min());
        let fraction = self.alignment_fraction(value, props);
        let disabled = props.disabled() || self.thumb_disabled(index);
        let z_index = if self.active_thumb == Some(index) {
            2
        } else if self.last_used_thumb == Some(index) {
            1
        } else {
            0
        };
        let half_thumb_main_axis = self
            .thumb_bounds
            .get(index)
            .copied()
            .flatten()
            .map(|bounds| match props.orientation() {
                SliderOrientation::Horizontal => f64::from(bounds.size.width.as_f32()) / 2.0,
                SliderOrientation::Vertical => f64::from(bounds.size.height.as_f32()) / 2.0,
            })
            .unwrap_or(0.0);
        let edge_offset = self.edge_travel(props).map(|travel| fraction * travel);

        SliderThumbStyleState {
            root: self.root_state(props),
            index,
            value,
            formatted_value: props.format_value(value),
            focused: self.focused_thumb == Some(index),
            active: self.active_thumb == Some(index),
            disabled,
            fraction,
            positioned: self.alignment_positioned(props),
            half_thumb_main_axis,
            edge_offset,
            z_index,
        }
    }

    pub fn value_state(&self, props: &SliderProps) -> SliderValueStyleState {
        SliderValueStyleState {
            root: self.root_state(props),
            values: self.values.clone(),
            formatted_values: self
                .values
                .iter()
                .map(|value| props.format_value(*value))
                .collect(),
        }
    }

    pub fn label_state(&self, props: &SliderProps) -> SliderLabelStyleState {
        SliderLabelStyleState {
            root: self.root_state(props),
        }
    }

    fn thumb_disabled(&self, index: usize) -> bool {
        self.thumbs
            .get(index)
            .map(|thumb| thumb.disabled)
            .unwrap_or(false)
    }

    fn clear_interaction(&mut self) {
        self.pressed_thumb = None;
        self.track_pressed = false;
        self.move_count = 0;
        self.dragging = false;
        self.thumb_offset = 0.0;
        self.drag_start_values = Vec::from([]);
    }

    /// Whether edge-aligned parts have the measurements needed to position.
    fn alignment_positioned(&self, props: &SliderProps) -> bool {
        match props.thumb_alignment() {
            SliderThumbAlignment::Center => true,
            SliderThumbAlignment::Edge => self.edge_travel(props).is_some(),
        }
    }

    /// The main-axis travel in pixels for edge alignment (control − thumb).
    fn edge_travel(&self, props: &SliderProps) -> Option<f64> {
        if props.thumb_alignment() != SliderThumbAlignment::Edge {
            return None;
        }
        let control = self.control_bounds?;
        let thumb = self.thumb_bounds.iter().copied().flatten().next()?;
        let travel = match props.orientation() {
            SliderOrientation::Horizontal => {
                f64::from(control.size.width.as_f32()) - f64::from(thumb.size.width.as_f32())
            }
            SliderOrientation::Vertical => {
                f64::from(control.size.height.as_f32()) - f64::from(thumb.size.height.as_f32())
            }
        };
        Some(travel.max(0.0))
    }

    fn alignment_fraction(&self, value: f64, props: &SliderProps) -> f64 {
        value_to_fraction(value, props.min(), props.max())
    }

    fn edge_inset(&self, props: &SliderProps) -> f64 {
        if props.thumb_alignment() != SliderThumbAlignment::Edge {
            return 0.0;
        }
        self.thumb_bounds
            .iter()
            .copied()
            .flatten()
            .next()
            .map(|bounds| match props.orientation() {
                SliderOrientation::Horizontal => f64::from(bounds.size.width.as_f32()) / 2.0,
                SliderOrientation::Vertical => f64::from(bounds.size.height.as_f32()) / 2.0,
            })
            .unwrap_or(0.0)
    }

    /// Converts a pointer position to a step-rounded, clamped value using the
    /// measured control bounds.
    fn position_to_value(
        &self,
        position: Point<Pixels>,
        direction: TextDirection,
        props: &SliderProps,
    ) -> Option<f64> {
        let control = self.control_bounds?;
        let inset = self.edge_inset(props);
        let fraction = match props.orientation() {
            SliderOrientation::Horizontal => position_to_fraction(
                f64::from(position.x.as_f32()),
                f64::from(control.origin.x.as_f32()),
                f64::from(control.size.width.as_f32()),
                inset,
                direction.is_rtl(),
            ),
            SliderOrientation::Vertical => position_to_fraction(
                f64::from(position.y.as_f32()),
                f64::from(control.origin.y.as_f32()),
                f64::from(control.size.height.as_f32()),
                inset,
                true,
            ),
        };

        let raw = fraction_to_value(fraction, props.min(), props.max());
        let rounded = round_value_to_step(raw, props.step(), props.min());
        Some(clamp(rounded, props.min(), props.max()))
    }

    /// The nearest enabled thumb by main-axis midpoint distance; ties go to the
    /// later index. Falls back to value-derived midpoints when unmeasured.
    fn closest_enabled_thumb(
        &self,
        position: Point<Pixels>,
        direction: TextDirection,
        props: &SliderProps,
    ) -> Option<usize> {
        let count = self.values.len();
        if count == 0 {
            return None;
        }

        let target_value = self.position_to_value(position, direction, props)?;
        let mut best: Option<(usize, f64)> = None;
        for index in 0..count {
            if self.thumb_disabled(index) {
                continue;
            }
            let distance = match self.thumb_bounds.get(index).copied().flatten() {
                Some(bounds) => {
                    let midpoint = match props.orientation() {
                        SliderOrientation::Horizontal => f64::from(bounds.center().x.as_f32()),
                        SliderOrientation::Vertical => f64::from(bounds.center().y.as_f32()),
                    };
                    let pointer = match props.orientation() {
                        SliderOrientation::Horizontal => f64::from(position.x.as_f32()),
                        SliderOrientation::Vertical => f64::from(position.y.as_f32()),
                    };
                    (pointer - midpoint).abs()
                }
                None => (target_value - self.values[index]).abs(),
            };
            let better = best
                .map(|(_, best_distance)| distance <= best_distance)
                .unwrap_or(true);
            if better {
                best = Some((index, distance));
            }
        }
        best.map(|(index, _)| index)
    }

    /// Resolves a pointer-proposed value through collision behavior and
    /// minimum-distance validation into a proposal.
    fn propose_pointer_value(
        &self,
        next_value: f64,
        pressed_index: usize,
        reason: SliderChangeReason,
        props: &SliderProps,
    ) -> Option<SliderProposal> {
        if !next_value.is_finite() {
            return None;
        }

        if self.values.len() < 2 {
            let next = clamp(next_value, props.min(), props.max());
            if values_equal(&[next], &self.values) {
                return None;
            }
            return Some(SliderProposal {
                values: Vec::from([next]),
                reason,
                thumb_index: 0,
                new_pressed_index: None,
                did_swap: false,
                commit_immediately: false,
            });
        }

        let initial_values = if self.drag_start_values.is_empty() {
            None
        } else {
            Some(self.drag_start_values.as_slice())
        };
        let result = resolve_thumb_collision(
            props.thumb_collision_behavior(),
            &self.values,
            initial_values,
            pressed_index,
            next_value,
            props.min(),
            props.max(),
            props.step(),
            props.min_steps_between_values(),
        );

        if !validate_minimum_distance(
            &result.values,
            props.step(),
            props.min_steps_between_values(),
        ) {
            return None;
        }
        if values_equal(&result.values, &self.values) {
            return None;
        }

        Some(SliderProposal {
            values: result.values,
            reason,
            thumb_index: result.thumb_index,
            new_pressed_index: if result.did_swap {
                Some(result.thumb_index)
            } else {
                None
            },
            did_swap: result.did_swap,
            commit_immediately: false,
        })
    }
}

/// Element-wise value equality.
pub fn values_equal(left: &[f64], right: &[f64]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right.iter())
            .all(|(left, right)| left == right)
}

/// Formats a value for display when no format closure is provided.
pub fn format_slider_value(value: f64) -> SharedString {
    SharedString::from(value.to_string())
}

#[cfg(test)]
mod tests {
    use gpui::{point, px, size, Bounds, Point};

    use crate::slider::{
        SliderChangeReason, SliderKeyboardStep, SliderProps, SliderRuntime,
        SliderThumbCollisionBehavior, SliderValues,
    };
    use crate::utils::TextDirection;

    fn props() -> SliderProps {
        SliderProps::new(
            None,
            0.0,
            100.0,
            1.0,
            10.0,
            0.0,
            Default::default(),
            SliderThumbCollisionBehavior::Push,
            Default::default(),
            false,
            None,
            None,
            None,
        )
    }

    fn control_bounds() -> Bounds<gpui::Pixels> {
        Bounds::new(point(px(0.0), px(0.0)), size(px(100.0), px(20.0)))
    }

    fn at(x: f32) -> Point<gpui::Pixels> {
        point(px(x), px(10.0))
    }

    #[test]
    fn uncontrolled_default_initializes_to_single_min() {
        let runtime = SliderRuntime::new(None, 5.0, 100.0);
        assert_eq!(runtime.current_values(), SliderValues::Single(5.0));
    }

    #[test]
    fn unsorted_range_defaults_are_sorted() {
        let runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([80.0, 20.0]))),
            0.0,
            100.0,
        );
        assert_eq!(
            runtime.current_values(),
            SliderValues::Range(Vec::from([20.0, 80.0]))
        );
    }

    #[test]
    fn single_default_out_of_range_is_clamped() {
        let runtime = SliderRuntime::new(Some(SliderValues::Single(150.0)), 0.0, 100.0);
        assert_eq!(runtime.current_values(), SliderValues::Single(100.0));
    }

    #[test]
    fn controlled_reconcile_replaces_values() {
        let mut runtime = SliderRuntime::new(None, 0.0, 100.0);
        runtime.reconcile(Some(SliderValues::Single(40.0)), true, &props());
        assert_eq!(runtime.current_values(), SliderValues::Single(40.0));
    }

    #[test]
    fn track_press_proposes_track_press_reason_and_begins_dragging() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());

        let proposal = runtime
            .press_track(at(60.0), TextDirection::Ltr, &props())
            .expect("track press should propose");
        assert_eq!(proposal.reason, SliderChangeReason::TrackPress);
        assert_eq!(proposal.values, [60.0]);

        let root = runtime.root_state(&props());
        assert!(root.dragging);
    }

    #[test]
    fn thumb_press_applies_no_change_until_movement() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());

        let focus_target = runtime.press_thumb(0, at(20.0), TextDirection::Ltr, &props());
        assert_eq!(focus_target, Some(0));
        assert_eq!(runtime.current_values(), SliderValues::Single(20.0));
        assert!(!runtime.root_state(&props()).dragging);
    }

    #[test]
    fn dragging_becomes_true_after_intentional_threshold() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());
        runtime.press_thumb(0, at(20.0), TextDirection::Ltr, &props());

        for offset in [21.0, 22.0] {
            if let Some(proposal) = runtime.drag_to(at(offset), TextDirection::Ltr, &props()) {
                runtime.apply_proposal(&proposal, true);
            }
            assert!(!runtime.root_state(&props()).dragging);
        }
        if let Some(proposal) = runtime.drag_to(at(23.0), TextDirection::Ltr, &props()) {
            runtime.apply_proposal(&proposal, true);
        }
        assert!(runtime.root_state(&props()).dragging);
    }

    #[test]
    fn drag_fires_with_drag_reason_and_release_commits_once() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());

        let proposal = runtime
            .press_track(at(50.0), TextDirection::Ltr, &props())
            .expect("press should propose");
        runtime.apply_proposal(&proposal, true);

        let proposal = runtime
            .drag_to(at(70.0), TextDirection::Ltr, &props())
            .expect("drag should propose");
        assert_eq!(proposal.reason, SliderChangeReason::Drag);
        runtime.apply_proposal(&proposal, true);

        let commit = runtime.release().expect("release should commit");
        assert_eq!(commit.0, [70.0]);
        assert!(runtime.release().is_none());
    }

    #[test]
    fn release_without_change_commits_nothing() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());
        runtime.press_thumb(0, at(20.0), TextDirection::Ltr, &props());
        assert!(runtime.release().is_none());
    }

    #[test]
    fn rtl_inverts_position_mapping() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());

        let proposal = runtime
            .press_track(at(25.0), TextDirection::Rtl, &props())
            .expect("track press should propose");
        assert_eq!(proposal.values, [75.0]);
    }

    #[test]
    fn keyboard_step_increments_and_commits_immediately() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        let proposal = runtime
            .keyboard_step(0, SliderKeyboardStep::Increment, &props())
            .expect("keyboard should propose");
        assert_eq!(proposal.values, [21.0]);
        assert_eq!(proposal.reason, SliderChangeReason::Keyboard);
        assert!(proposal.commit_immediately);
    }

    #[test]
    fn keyboard_home_end_clamp_to_neighbors_in_range_mode() {
        let mut runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([20.0, 80.0]))),
            0.0,
            100.0,
        );
        let proposal = runtime
            .keyboard_step(0, SliderKeyboardStep::End, &props())
            .expect("end should propose");
        assert_eq!(proposal.values, [80.0, 80.0]);

        let proposal = runtime
            .keyboard_step(1, SliderKeyboardStep::Home, &props())
            .expect("home should propose");
        assert_eq!(proposal.values, [20.0, 20.0]);
    }

    #[test]
    fn keyboard_never_pushes_or_swaps() {
        let mut runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([20.0, 30.0]))),
            0.0,
            100.0,
        );
        for _ in 0..20 {
            if let Some(proposal) =
                runtime.keyboard_step(0, SliderKeyboardStep::Increment, &props())
            {
                runtime.apply_proposal(&proposal, true);
            }
        }
        assert_eq!(
            runtime.current_values(),
            SliderValues::Range(Vec::from([30.0, 30.0]))
        );
    }

    #[test]
    fn keyboard_respects_minimum_distance() {
        let props = SliderProps::new(
            None,
            0.0,
            100.0,
            1.0,
            10.0,
            5.0,
            Default::default(),
            SliderThumbCollisionBehavior::Push,
            Default::default(),
            false,
            None,
            None,
            None,
        );
        let mut runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([20.0, 25.0]))),
            0.0,
            100.0,
        );
        assert!(runtime
            .keyboard_step(0, SliderKeyboardStep::Increment, &props)
            .is_none());
    }

    #[test]
    fn equal_value_proposal_is_a_no_op() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        assert!(runtime
            .keyboard_step(
                0,
                SliderKeyboardStep::Decrement,
                &SliderProps::new(
                    None,
                    20.0,
                    100.0,
                    1.0,
                    10.0,
                    0.0,
                    Default::default(),
                    SliderThumbCollisionBehavior::Push,
                    Default::default(),
                    false,
                    None,
                    None,
                    None,
                ),
            )
            .is_none());
    }

    #[test]
    fn disabled_slider_ignores_pointer_and_keyboard() {
        let disabled_props = SliderProps::new(
            None,
            0.0,
            100.0,
            1.0,
            10.0,
            0.0,
            Default::default(),
            SliderThumbCollisionBehavior::Push,
            Default::default(),
            true,
            None,
            None,
            None,
        );
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.set_control_bounds(control_bounds());
        assert!(runtime
            .press_track(at(60.0), TextDirection::Ltr, &disabled_props)
            .is_none());
        assert!(runtime
            .keyboard_step(0, SliderKeyboardStep::Increment, &disabled_props)
            .is_none());
    }

    #[test]
    fn swap_behavior_moves_pressed_index_only_when_applied() {
        let swap_props = SliderProps::new(
            None,
            0.0,
            100.0,
            1.0,
            10.0,
            0.0,
            Default::default(),
            SliderThumbCollisionBehavior::Swap,
            Default::default(),
            false,
            None,
            None,
            None,
        );
        let mut runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([20.0, 40.0]))),
            0.0,
            100.0,
        );
        runtime.set_control_bounds(control_bounds());
        runtime.press_thumb(0, at(20.0), TextDirection::Ltr, &swap_props);

        let proposal = runtime
            .drag_to(at(50.0), TextDirection::Ltr, &swap_props)
            .expect("drag should propose a swap");
        assert!(proposal.did_swap);
        assert_eq!(proposal.thumb_index, 1);

        // Canceled: pressed index must not leak the swap.
        let unchanged = runtime
            .drag_to(at(50.0), TextDirection::Ltr, &swap_props)
            .expect("pressed index should still be 0");
        assert!(unchanged.did_swap);

        // Applied: pressed index follows the swap.
        runtime.apply_proposal(&proposal, true);
        assert_eq!(
            runtime.current_values(),
            SliderValues::Range(Vec::from([40.0, 50.0]))
        );
    }

    #[test]
    fn stacked_max_thumb_press_retargets_to_first_of_stack() {
        let mut runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([100.0, 100.0]))),
            0.0,
            100.0,
        );
        runtime.set_control_bounds(control_bounds());
        let target = runtime.press_thumb(1, at(100.0), TextDirection::Ltr, &props());
        assert_eq!(target, Some(0));
    }

    #[test]
    fn track_press_tie_goes_to_later_thumb() {
        let mut runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([40.0, 60.0]))),
            0.0,
            100.0,
        );
        runtime.set_control_bounds(control_bounds());
        let proposal = runtime
            .press_track(at(50.0), TextDirection::Ltr, &props())
            .expect("track press should propose");
        assert_eq!(proposal.thumb_index, 1);
    }

    #[test]
    fn indicator_fractions_match_value_percents() {
        let runtime = SliderRuntime::new(
            Some(SliderValues::Range(Vec::from([20.0, 80.0]))),
            0.0,
            100.0,
        );
        let state = runtime.indicator_state(&props());
        assert_eq!(state.start_fraction, 0.2);
        assert_eq!(state.end_fraction, 0.8);

        let single = SliderRuntime::new(Some(SliderValues::Single(30.0)), 0.0, 100.0);
        let state = single.indicator_state(&props());
        assert_eq!(state.start_fraction, 0.0);
        assert_eq!(state.end_fraction, 0.3);
    }

    #[test]
    fn nan_proposals_are_rejected() {
        let mut runtime = SliderRuntime::new(Some(SliderValues::Single(20.0)), 0.0, 100.0);
        runtime.reconcile(Some(SliderValues::Single(f64::NAN)), true, &props());
        // Non-finite controlled values are dropped to min-clamped fallback.
        assert_eq!(runtime.current_values(), SliderValues::Single(0.0));
    }
}
