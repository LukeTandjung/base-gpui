use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use gpui::{App, FocusHandle, SharedString, Window};

use crate::toast::{
    ToastActionStyleState, ToastCloseStyleState, ToastContentStyleState,
    ToastDescriptionStyleState, ToastPriority, ToastRootStyleState, ToastSwipeDirection,
    ToastTitleStyleState, ToastTransitionStatus, ToastType, ToastViewportStyleState,
};

/// Default auto-dismiss timeout (Base UI `timeout` default).
pub const TOAST_DEFAULT_TIMEOUT: Duration = Duration::from_millis(5000);
/// Default visible-toast limit (Base UI `limit` default).
pub const TOAST_DEFAULT_LIMIT: usize = 3;

const SWIPE_REAL_THRESHOLD: f32 = 1.0;
const SWIPE_DISMISS_THRESHOLD: f32 = 40.0;
const SWIPE_REVERSE_CANCEL_THRESHOLD: f32 = 10.0;

static TOAST_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Typed toast identity. Runtime-generated ids are unique per process;
/// caller-supplied ids enable upsert-by-id.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ToastId(SharedString);

impl ToastId {
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self(id.into())
    }

    /// Generates a process-unique toast id.
    pub fn generate() -> Self {
        let count = TOAST_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(SharedString::from(format!("base-gpui-toast-{count}")))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// Callback fired for toast lifecycle events (`onClose` / `onRemove`).
pub type ToastCallback = Rc<dyn Fn(&mut App) + 'static>;

/// Callback fired when a toast action button activates.
pub type ToastActionHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// Typed action definition replacing Base UI's `actionProps` DOM-props bag.
#[derive(Clone)]
pub struct ToastActionDef {
    label: SharedString,
    on_click: ToastActionHandler,
}

impl ToastActionDef {
    pub fn new(
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            on_click: Rc::new(on_click),
        }
    }

    pub fn label(&self) -> SharedString {
        self.label.clone()
    }

    pub fn on_click(&self) -> ToastActionHandler {
        Rc::clone(&self.on_click)
    }
}

/// Options for `add`/`update`. On `add`, `Some` fields override defaults; on
/// `update`, `Some` fields replace the live record's fields and `None` fields
/// are left untouched (Base UI object-spread parity).
pub struct ToastOptions<P: Clone + 'static> {
    pub id: Option<ToastId>,
    pub title: Option<SharedString>,
    pub description: Option<SharedString>,
    pub toast_type: Option<ToastType>,
    pub timeout: Option<Duration>,
    pub priority: Option<ToastPriority>,
    pub action: Option<ToastActionDef>,
    pub on_close: Option<ToastCallback>,
    pub on_remove: Option<ToastCallback>,
    pub payload: Option<P>,
}

impl<P: Clone + 'static> Default for ToastOptions<P> {
    fn default() -> Self {
        Self {
            id: None,
            title: None,
            description: None,
            toast_type: None,
            timeout: None,
            priority: None,
            action: None,
            on_close: None,
            on_remove: None,
            payload: None,
        }
    }
}

impl<P: Clone + 'static> ToastOptions<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: ToastId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = Some(toast_type);
        self
    }

    /// A `Duration::ZERO` timeout makes the toast sticky (never auto-dismissed).
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn priority(mut self, priority: ToastPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn action(mut self, action: ToastActionDef) -> Self {
        self.action = Some(action);
        self
    }

    pub fn on_close(mut self, on_close: impl Fn(&mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(on_close));
        self
    }

    pub fn on_remove(mut self, on_remove: impl Fn(&mut App) + 'static) -> Self {
        self.on_remove = Some(Rc::new(on_remove));
        self
    }

    pub fn payload(mut self, payload: P) -> Self {
        self.payload = Some(payload);
        self
    }
}

/// Public facts about one toast — the manager-facing `toasts` array analog and
/// the input to the viewport's per-toast content builder.
pub struct ToastFacts<P: Clone + 'static> {
    pub id: ToastId,
    pub title: Option<SharedString>,
    pub description: Option<SharedString>,
    pub toast_type: Option<ToastType>,
    pub priority: ToastPriority,
    pub payload: Option<P>,
    pub limited: bool,
    pub ending: bool,
    pub height: f32,
    pub action: Option<ToastActionDef>,
}

/// A timer decision the runtime hands back to layers/context for execution:
/// the runtime decides, generation-counted background timers execute.
#[derive(Clone, Debug, PartialEq)]
pub enum ToastTimerOp {
    Schedule {
        id: ToastId,
        generation: u64,
        duration: Duration,
    },
    Cancel {
        id: ToastId,
    },
}

/// Outcome of `add_toast`.
pub struct ToastAddOutcome {
    pub id: ToastId,
    pub timer_ops: Vec<ToastTimerOp>,
}

/// Outcome of `close_toast`: `on_close` callbacks to fire (the runtime never
/// calls user callbacks itself) plus timer cancellations.
pub struct ToastCloseOutcome {
    pub closed: Vec<ToastId>,
    pub on_close: Vec<ToastCallback>,
    pub timer_ops: Vec<ToastTimerOp>,
}

/// Outcome of releasing a swipe gesture.
#[derive(Clone, Debug, PartialEq)]
pub struct ToastSwipeRelease {
    pub id: ToastId,
    pub dismiss: bool,
    pub direction: Option<ToastSwipeDirection>,
}

struct ToastTimer {
    /// Remaining duration when not running; full duration before first start.
    remaining: Duration,
    /// `Some` while running: the instant the current countdown started.
    started_at: Option<Instant>,
    /// Generation of the last scheduled background timer; stale completions
    /// are no-ops.
    generation: u64,
}

struct ToastRecord<P: Clone + 'static> {
    id: ToastId,
    title: Option<SharedString>,
    description: Option<SharedString>,
    toast_type: Option<ToastType>,
    timeout: Option<Duration>,
    priority: ToastPriority,
    action: Option<ToastActionDef>,
    on_close: Option<ToastCallback>,
    on_remove: Option<ToastCallback>,
    payload: Option<P>,
    update_generation: u64,
    transition: Option<ToastTransitionStatus>,
    limited: bool,
    height: f32,
    measured: bool,
    timer: Option<ToastTimer>,
}

struct SwipeGesture {
    id: ToastId,
    start: (f32, f32),
    allowed: Vec<ToastSwipeDirection>,
    real: bool,
    locked_horizontal: Option<bool>,
    direction: Option<ToastSwipeDirection>,
    movement: (f32, f32),
    along: f32,
    max_along: f32,
    canceled: bool,
}

fn signed_sqrt_damping(delta: f32) -> f32 {
    delta.signum() * delta.abs().sqrt()
}

/// The single deep module owning all toast state: the newest-first queue,
/// per-toast records and pausable timer bookkeeping, hover/focus/window-focus
/// facts, limit/timeout defaults, the swipe state machine, and derived
/// stacking metadata. Plain `&mut self` commands and part-shaped queries;
/// never calls user callbacks; unit-testable against injected `Instant`s.
pub struct ToastRuntime<P: Clone + 'static> {
    toasts: Vec<ToastRecord<P>>,
    default_timeout: Duration,
    limit: usize,
    hovering: bool,
    focused: bool,
    window_focused: bool,
    timer_generation: u64,
    gesture: Option<SwipeGesture>,
    previous_focus: Option<FocusHandle>,
}

impl<P: Clone + 'static> Default for ToastRuntime<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Clone + 'static> ToastRuntime<P> {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            default_timeout: TOAST_DEFAULT_TIMEOUT,
            limit: TOAST_DEFAULT_LIMIT,
            hovering: false,
            focused: false,
            window_focused: true,
            timer_generation: 0,
            gesture: None,
            previous_focus: None,
        }
    }

    /// Syncs the provider-configured defaults observed this render and
    /// recomputes `limited` flags (Base UI `syncProviderProps` parity).
    pub fn sync_provider_props(&mut self, timeout: Duration, limit: usize) {
        self.default_timeout = timeout;
        self.limit = limit;
        self.recompute_limited();
    }

    /// Whether all dismiss timers are currently held (viewport hovered,
    /// focus-visible focused, or the window unfocused).
    pub fn paused(&self) -> bool {
        self.hovering || self.focused || !self.window_focused
    }

    fn next_timer_generation(&mut self) -> u64 {
        self.timer_generation += 1;
        self.timer_generation
    }

    fn resolved_duration(&self, record: &ToastRecord<P>) -> Option<Duration> {
        if record.toast_type == Some(ToastType::Loading) {
            return None;
        }
        let duration = record.timeout.unwrap_or(self.default_timeout);
        if duration.is_zero() {
            None
        } else {
            Some(duration)
        }
    }

    fn find(&self, id: &ToastId) -> Option<usize> {
        self.toasts.iter().position(|toast| toast.id == *id)
    }

    fn is_ending(record: &ToastRecord<P>) -> bool {
        record.transition == Some(ToastTransitionStatus::Ending)
    }

    fn recompute_limited(&mut self) {
        let limit = self.limit;
        let mut active_index = 0usize;
        for record in self.toasts.iter_mut() {
            if Self::is_ending(record) {
                record.limited = false;
                continue;
            }
            record.limited = active_index >= limit;
            active_index += 1;
        }
    }

    /// Resets hover/focus facts once no active toasts remain so a future toast
    /// starts from a collapsed viewport.
    fn reset_facts_if_empty(&mut self) {
        if !self.toasts.iter().any(|record| !Self::is_ending(record)) {
            self.hovering = false;
            self.focused = false;
        }
    }

    /// Starts (or holds, when paused) a toast's timer for its full resolved
    /// duration. Emits a schedule op only when the timer actually starts.
    fn reset_timer(&mut self, index: usize, now: Instant, ops: &mut Vec<ToastTimerOp>) {
        let paused = self.paused();
        let duration = self.resolved_duration(&self.toasts[index]);
        let generation = self.next_timer_generation();
        let record = &mut self.toasts[index];
        if record.timer.is_some() {
            ops.push(ToastTimerOp::Cancel {
                id: record.id.clone(),
            });
        }
        match duration {
            None => record.timer = None,
            Some(duration) => {
                if paused {
                    record.timer = Some(ToastTimer {
                        remaining: duration,
                        started_at: None,
                        generation,
                    });
                } else {
                    record.timer = Some(ToastTimer {
                        remaining: duration,
                        started_at: Some(now),
                        generation,
                    });
                    ops.push(ToastTimerOp::Schedule {
                        id: record.id.clone(),
                        generation,
                        duration,
                    });
                }
            }
        }
    }

    /// Adds a toast (prepended; newest first) or upserts in place when a toast
    /// with the same id exists and is not ending. An upsert over an ending
    /// toast removes it silently (no `on_remove`) and adds a fresh toast.
    pub fn add_toast(&mut self, options: ToastOptions<P>, now: Instant) -> ToastAddOutcome {
        let mut ops = Vec::new();
        let id = options.id.clone().unwrap_or_else(ToastId::generate);

        if let Some(index) = self.find(&id) {
            if !Self::is_ending(&self.toasts[index]) {
                // Upsert in place: fields replaced, timer reset, generation
                // incremented, queue position unchanged.
                let record = &mut self.toasts[index];
                record.title = options.title;
                record.description = options.description;
                record.toast_type = options.toast_type;
                record.timeout = options.timeout;
                record.priority = options.priority.unwrap_or_default();
                record.action = options.action;
                if options.on_close.is_some() {
                    record.on_close = options.on_close;
                }
                if options.on_remove.is_some() {
                    record.on_remove = options.on_remove;
                }
                record.payload = options.payload;
                record.update_generation += 1;
                self.reset_timer(index, now, &mut ops);
                self.recompute_limited();
                return ToastAddOutcome { id, timer_ops: ops };
            }
            // Ending toast with the same id: remove silently, then add fresh.
            if self.toasts[index].timer.is_some() {
                ops.push(ToastTimerOp::Cancel { id: id.clone() });
            }
            self.toasts.remove(index);
        }

        self.toasts.insert(
            0,
            ToastRecord {
                id: id.clone(),
                title: options.title,
                description: options.description,
                toast_type: options.toast_type,
                timeout: options.timeout,
                priority: options.priority.unwrap_or_default(),
                action: options.action,
                on_close: options.on_close,
                on_remove: options.on_remove,
                payload: options.payload,
                update_generation: 0,
                transition: Some(ToastTransitionStatus::Starting),
                limited: false,
                height: 0.0,
                measured: false,
                timer: None,
            },
        );
        self.reset_timer(0, now, &mut ops);
        self.recompute_limited();
        ToastAddOutcome { id, timer_ops: ops }
    }

    /// Partially updates a live toast; `Some` fields replace, `None` fields
    /// stay. Ignored for ending/missing toasts (so async promise updates never
    /// block a dismissal). Clears the timer when the toast should no longer
    /// have one; (re)schedules when the timeout changed or the toast was
    /// previously `Loading`.
    pub fn update_toast(
        &mut self,
        id: &ToastId,
        options: ToastOptions<P>,
        now: Instant,
    ) -> Vec<ToastTimerOp> {
        let mut ops = Vec::new();
        let Some(index) = self.find(id) else {
            return ops;
        };
        if Self::is_ending(&self.toasts[index]) {
            return ops;
        }
        let previous_type = self.toasts[index].toast_type.clone();
        let previous_timeout = self.toasts[index].timeout;

        let record = &mut self.toasts[index];
        if let Some(title) = options.title {
            record.title = Some(title);
        }
        if let Some(description) = options.description {
            record.description = Some(description);
        }
        if let Some(toast_type) = options.toast_type {
            record.toast_type = Some(toast_type);
        }
        if let Some(timeout) = options.timeout {
            record.timeout = Some(timeout);
        }
        if let Some(priority) = options.priority {
            record.priority = priority;
        }
        if let Some(action) = options.action {
            record.action = Some(action);
        }
        if let Some(on_close) = options.on_close {
            record.on_close = Some(on_close);
        }
        if let Some(on_remove) = options.on_remove {
            record.on_remove = Some(on_remove);
        }
        if let Some(payload) = options.payload {
            record.payload = Some(payload);
        }
        record.update_generation += 1;

        let new_type = record.toast_type.clone();
        let new_timeout = record.timeout;
        let should_have_timer = self.resolved_duration(&self.toasts[index]).is_some();
        let was_loading = previous_type == Some(ToastType::Loading);
        if !should_have_timer {
            let record = &mut self.toasts[index];
            if record.timer.take().is_some() {
                ops.push(ToastTimerOp::Cancel { id: id.clone() });
            }
        } else if previous_timeout != new_timeout
            || was_loading
            || (previous_type != new_type && new_type.is_some())
        {
            self.reset_timer(index, now, &mut ops);
        }
        ops
    }

    /// Marks a toast (or all toasts when `id` is `None`) as `ending`, cancels
    /// timers, and returns the `on_close` callbacks to fire. Already-ending
    /// toasts are skipped so `on_close` fires exactly once. Removal is a
    /// separate `remove_toast` step deferred until the exit transition
    /// completes (immediate when no animation runs).
    pub fn close_toast(&mut self, id: Option<&ToastId>) -> ToastCloseOutcome {
        let mut outcome = ToastCloseOutcome {
            closed: Vec::new(),
            on_close: Vec::new(),
            timer_ops: Vec::new(),
        };
        for record in self.toasts.iter_mut() {
            if let Some(target) = id {
                if record.id != *target {
                    continue;
                }
            }
            if Self::is_ending(record) {
                continue;
            }
            record.transition = Some(ToastTransitionStatus::Ending);
            if record.timer.take().is_some() {
                outcome.timer_ops.push(ToastTimerOp::Cancel {
                    id: record.id.clone(),
                });
            }
            outcome.closed.push(record.id.clone());
            if let Some(on_close) = record.on_close.as_ref() {
                outcome.on_close.push(Rc::clone(on_close));
            }
        }
        if self
            .gesture
            .as_ref()
            .is_some_and(|gesture| outcome.closed.contains(&gesture.id) && id.is_some())
            || id.is_none()
        {
            self.gesture = None;
        }
        self.recompute_limited();
        self.reset_facts_if_empty();
        outcome
    }

    /// Removes an ending toast after its exit transition and returns its
    /// `on_remove` callback for the caller to fire.
    pub fn remove_toast(&mut self, id: &ToastId) -> Option<ToastCallback> {
        let index = self.find(id)?;
        let record = self.toasts.remove(index);
        self.recompute_limited();
        self.reset_facts_if_empty();
        record.on_remove
    }

    /// Pauses all running timers, recording each toast's remaining duration
    /// (`delay - elapsed`, clamped at zero). Idempotent.
    pub fn pause_timers(&mut self, now: Instant) {
        for record in self.toasts.iter_mut() {
            if let Some(timer) = record.timer.as_mut() {
                if let Some(started_at) = timer.started_at.take() {
                    let elapsed = now.saturating_duration_since(started_at);
                    timer.remaining = timer.remaining.saturating_sub(elapsed);
                }
            }
        }
    }

    /// Resumes all held timers with their remaining duration (full duration if
    /// never started), returning schedule ops. Idempotent: running timers are
    /// untouched.
    pub fn resume_timers(&mut self, now: Instant) -> Vec<ToastTimerOp> {
        let mut ops = Vec::new();
        for index in 0..self.toasts.len() {
            if Self::is_ending(&self.toasts[index]) {
                continue;
            }
            if self.toasts[index]
                .timer
                .as_ref()
                .is_some_and(|timer| timer.started_at.is_none())
            {
                let generation = self.next_timer_generation();
                let record = &mut self.toasts[index];
                let timer = record.timer.as_mut().expect("checked above");
                timer.started_at = Some(now);
                timer.generation = generation;
                ops.push(ToastTimerOp::Schedule {
                    id: record.id.clone(),
                    generation,
                    duration: timer.remaining,
                });
            }
        }
        ops
    }

    fn apply_pause_state(&mut self, was_paused: bool, now: Instant) -> Vec<ToastTimerOp> {
        if self.paused() && !was_paused {
            self.pause_timers(now);
            Vec::new()
        } else if !self.paused() && was_paused {
            self.resume_timers(now)
        } else {
            Vec::new()
        }
    }

    /// Records viewport hover; pauses/resumes timers on transitions.
    pub fn set_hovering(&mut self, hovering: bool, now: Instant) -> Vec<ToastTimerOp> {
        let was_paused = self.paused();
        self.hovering = hovering;
        self.apply_pause_state(was_paused, now)
    }

    /// Records viewport keyboard focus; pauses/resumes timers on transitions.
    pub fn set_focused(&mut self, focused: bool, now: Instant) -> Vec<ToastTimerOp> {
        let was_paused = self.paused();
        self.focused = focused;
        self.apply_pause_state(was_paused, now)
    }

    /// Records window activation (the `window blur/focus` analog); pauses or
    /// resumes timers on transitions.
    pub fn set_window_focused(&mut self, window_focused: bool, now: Instant) -> Vec<ToastTimerOp> {
        let was_paused = self.paused();
        self.window_focused = window_focused;
        self.apply_pause_state(was_paused, now)
    }

    /// Handles a fired background timer. A stale generation (after
    /// reset/pause/close) is a no-op; a live one closes the toast (transition
    /// to `ending`, not instant removal).
    pub fn timer_fired(&mut self, id: &ToastId, generation: u64) -> Option<ToastCloseOutcome> {
        let index = self.find(id)?;
        let record = &self.toasts[index];
        let live = record
            .timer
            .as_ref()
            .is_some_and(|timer| timer.generation == generation && timer.started_at.is_some());
        if !live || Self::is_ending(record) {
            return None;
        }
        Some(self.close_toast(Some(id)))
    }

    /// Records a toast's measured natural height; the first measurement clears
    /// the `Starting` transition so enter animations can key off it. Returns
    /// whether stacking metadata changed.
    pub fn set_toast_height(&mut self, id: &ToastId, height: f32) -> bool {
        let Some(index) = self.find(id) else {
            return false;
        };
        let record = &mut self.toasts[index];
        let mut changed = false;
        if !record.measured {
            record.measured = true;
            if record.transition == Some(ToastTransitionStatus::Starting) {
                record.transition = None;
            }
            changed = true;
        }
        if (record.height - height).abs() > f32::EPSILON {
            record.height = height;
            changed = true;
        }
        changed
    }

    /// Records the focus handle to restore when focus leaves the viewport.
    pub fn record_previous_focus(&mut self, focus: Option<FocusHandle>) {
        self.previous_focus = focus;
    }

    /// Takes the recorded previous-focus handle for restoration.
    pub fn take_previous_focus(&mut self) -> Option<FocusHandle> {
        self.previous_focus.take()
    }

    /// Begins a swipe on a toast. Refused for ending/limited/missing toasts,
    /// while another gesture is active, or when no direction is permitted.
    pub fn begin_swipe(
        &mut self,
        id: &ToastId,
        x: f32,
        y: f32,
        allowed: Vec<ToastSwipeDirection>,
    ) -> bool {
        if allowed.is_empty() || self.gesture.is_some() {
            return false;
        }
        let Some(index) = self.find(id) else {
            return false;
        };
        let record = &self.toasts[index];
        if Self::is_ending(record) || record.limited {
            return false;
        }
        self.gesture = Some(SwipeGesture {
            id: id.clone(),
            start: (x, y),
            allowed,
            real: false,
            locked_horizontal: None,
            direction: None,
            movement: (0.0, 0.0),
            along: 0.0,
            max_along: 0.0,
            canceled: false,
        });
        true
    }

    /// Advances the active swipe: locks the dominant axis past 1px when both
    /// axes are permitted, moves 1:1 along permitted directions, damps
    /// (`sqrt`) non-permitted displacement, and tracks reverse-cancel when
    /// only one direction on the locked axis is permitted.
    pub fn move_swipe(&mut self, x: f32, y: f32) -> bool {
        let Some(gesture) = self.gesture.as_mut() else {
            return false;
        };
        let delta_x = x - gesture.start.0;
        let delta_y = y - gesture.start.1;

        let horizontal_allowed = gesture
            .allowed
            .iter()
            .any(|direction| direction.is_horizontal());
        let vertical_allowed = gesture
            .allowed
            .iter()
            .any(|direction| !direction.is_horizontal());

        if !gesture.real && delta_x.abs().max(delta_y.abs()) >= SWIPE_REAL_THRESHOLD {
            gesture.real = true;
            gesture.locked_horizontal = Some(if horizontal_allowed && vertical_allowed {
                delta_x.abs() > delta_y.abs()
            } else {
                horizontal_allowed
            });
        }

        let x_direction = if delta_x >= 0.0 {
            ToastSwipeDirection::Right
        } else {
            ToastSwipeDirection::Left
        };
        let y_direction = if delta_y >= 0.0 {
            ToastSwipeDirection::Down
        } else {
            ToastSwipeDirection::Up
        };

        let movement_x =
            if gesture.locked_horizontal == Some(true) && gesture.allowed.contains(&x_direction) {
                delta_x
            } else {
                signed_sqrt_damping(delta_x)
            };
        let movement_y =
            if gesture.locked_horizontal == Some(false) && gesture.allowed.contains(&y_direction) {
                delta_y
            } else {
                signed_sqrt_damping(delta_y)
            };
        gesture.movement = (movement_x, movement_y);

        match gesture.locked_horizontal {
            Some(true) => {
                if gesture.allowed.contains(&x_direction) {
                    gesture.direction = Some(x_direction);
                    gesture.along = delta_x.abs();
                } else {
                    gesture.direction = None;
                    gesture.along = 0.0;
                }
                let single_direction = !(gesture.allowed.contains(&ToastSwipeDirection::Left)
                    && gesture.allowed.contains(&ToastSwipeDirection::Right));
                gesture.max_along = gesture.max_along.max(gesture.along);
                if single_direction
                    && gesture.max_along - gesture.along >= SWIPE_REVERSE_CANCEL_THRESHOLD
                {
                    gesture.canceled = true;
                }
            }
            Some(false) => {
                if gesture.allowed.contains(&y_direction) {
                    gesture.direction = Some(y_direction);
                    gesture.along = delta_y.abs();
                } else {
                    gesture.direction = None;
                    gesture.along = 0.0;
                }
                let single_direction = !(gesture.allowed.contains(&ToastSwipeDirection::Up)
                    && gesture.allowed.contains(&ToastSwipeDirection::Down));
                gesture.max_along = gesture.max_along.max(gesture.along);
                if single_direction
                    && gesture.max_along - gesture.along >= SWIPE_REVERSE_CANCEL_THRESHOLD
                {
                    gesture.canceled = true;
                }
            }
            None => {}
        }
        true
    }

    /// Releases the swipe: dismisses past the 40px threshold in a permitted
    /// direction unless reverse-canceled; otherwise the toast springs back to
    /// rest with movement cleared.
    pub fn release_swipe(&mut self) -> Option<ToastSwipeRelease> {
        let gesture = self.gesture.take()?;
        let dismiss = gesture.real
            && !gesture.canceled
            && gesture.direction.is_some()
            && gesture.along >= SWIPE_DISMISS_THRESHOLD;
        Some(ToastSwipeRelease {
            id: gesture.id,
            dismiss,
            direction: if dismiss { gesture.direction } else { None },
        })
    }

    /// Cancels the active swipe (button loss, unmount), resetting to rest.
    pub fn cancel_swipe(&mut self) {
        self.gesture = None;
    }

    /// Whether a swipe gesture is being tracked (for release-outside capture).
    pub fn swiping(&self) -> bool {
        self.gesture.is_some()
    }

    fn visible_index(&self, index: usize) -> Option<usize> {
        if Self::is_ending(&self.toasts[index]) {
            return None;
        }
        Some(
            self.toasts[..index]
                .iter()
                .filter(|record| !Self::is_ending(record))
                .count(),
        )
    }

    fn offset_y(&self, index: usize) -> f32 {
        self.toasts[..index]
            .iter()
            .filter(|record| !Self::is_ending(record))
            .map(|record| record.height)
            .sum()
    }

    fn frontmost_height(&self) -> f32 {
        self.toasts
            .iter()
            .find(|record| !Self::is_ending(record))
            .map(|record| record.height)
            .unwrap_or(0.0)
    }

    /// The viewport's style state: `expanded` (hovering || focused) and the
    /// newest live toast's measured height for collapsed-stack sizing.
    pub fn viewport_state(&self) -> ToastViewportStyleState {
        ToastViewportStyleState {
            expanded: self.hovering || self.focused,
            frontmost_height: self.frontmost_height(),
            toast_count: self.toasts.len(),
        }
    }

    /// One toast root's style state: transition, expansion, limit flag, type,
    /// swipe facts, stacking index (visible while live, dom index while
    /// ending), cumulative offset, and measured height.
    pub fn root_state(&self, id: &ToastId) -> ToastRootStyleState {
        let Some(index) = self.find(id) else {
            return ToastRootStyleState::default();
        };
        let record = &self.toasts[index];
        let gesture = self
            .gesture
            .as_ref()
            .filter(|gesture| gesture.id == *id && gesture.real);
        ToastRootStyleState {
            transition_status: record.transition.clone(),
            priority: record.priority,
            expanded: self.hovering || self.focused,
            limited: record.limited,
            toast_type: record.toast_type.clone(),
            swiping: gesture.is_some(),
            swipe_direction: gesture.and_then(|gesture| gesture.direction),
            swipe_movement_x: gesture.map(|gesture| gesture.movement.0).unwrap_or(0.0),
            swipe_movement_y: gesture.map(|gesture| gesture.movement.1).unwrap_or(0.0),
            index: self.visible_index(index).or(Some(index)),
            visible_index: self.visible_index(index),
            offset_y: self.offset_y(index),
            height: record.height,
        }
    }

    /// One toast content container's style state.
    pub fn content_state(&self, id: &ToastId) -> ToastContentStyleState {
        let visible_index = self.find(id).and_then(|index| self.visible_index(index));
        ToastContentStyleState {
            expanded: self.hovering || self.focused,
            behind: visible_index.is_some_and(|index| index > 0),
        }
    }

    /// One toast title's style state (with the default title text).
    pub fn title_state(&self, id: &ToastId) -> ToastTitleStyleState {
        let record = self.find(id).map(|index| &self.toasts[index]);
        ToastTitleStyleState {
            toast_type: record.and_then(|record| record.toast_type.clone()),
            title: record.and_then(|record| record.title.clone()),
        }
    }

    /// One toast description's style state (with the default description text).
    pub fn description_state(&self, id: &ToastId) -> ToastDescriptionStyleState {
        let record = self.find(id).map(|index| &self.toasts[index]);
        ToastDescriptionStyleState {
            toast_type: record.and_then(|record| record.toast_type.clone()),
            description: record.and_then(|record| record.description.clone()),
        }
    }

    /// One toast close button's style state.
    pub fn close_state(&self, id: &ToastId) -> ToastCloseStyleState {
        ToastCloseStyleState {
            toast_type: self
                .find(id)
                .and_then(|index| self.toasts[index].toast_type.clone()),
            expanded: self.hovering || self.focused,
        }
    }

    /// One toast action button's style state (with the action definition).
    pub fn action_state(&self, id: &ToastId) -> ToastActionStyleState {
        let record = self.find(id).map(|index| &self.toasts[index]);
        ToastActionStyleState {
            toast_type: record.and_then(|record| record.toast_type.clone()),
            action: record.and_then(|record| record.action.clone()),
        }
    }

    /// Manager-facing facts about every toast, newest first — the documented
    /// exception mirroring Base UI's public `toasts` array.
    pub fn toasts(&self) -> Vec<ToastFacts<P>> {
        self.toasts
            .iter()
            .map(|record| ToastFacts {
                id: record.id.clone(),
                title: record.title.clone(),
                description: record.description.clone(),
                toast_type: record.toast_type.clone(),
                priority: record.priority,
                payload: record.payload.clone(),
                limited: record.limited,
                ending: Self::is_ending(record),
                height: record.height,
                action: record.action.clone(),
            })
            .collect()
    }

    /// A toast's remaining timer duration, if it currently has a timer.
    /// Test/diagnostic-shaped query used by the pause/resume suites.
    pub fn remaining_timeout(&self, id: &ToastId) -> Option<Duration> {
        let index = self.find(id)?;
        self.toasts[index]
            .timer
            .as_ref()
            .map(|timer| timer.remaining)
    }

    /// A toast's update generation (incremented on every upsert/update).
    pub fn update_generation(&self, id: &ToastId) -> Option<u64> {
        self.find(id)
            .map(|index| self.toasts[index].update_generation)
    }
}
