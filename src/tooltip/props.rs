use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{AnyElement, App, ElementId, Window};

use crate::tooltip::TooltipOpenChangeDetails;

pub const DEFAULT_TOOLTIP_DELAY: Duration = Duration::from_millis(600);
pub const DEFAULT_TOOLTIP_CLOSE_DELAY: Duration = Duration::ZERO;
pub const DEFAULT_TOOLTIP_TIMEOUT: Duration = Duration::from_millis(400);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipSide {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipAlign {
    Start,
    #[default]
    Center,
    End,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipTrackCursorAxis {
    #[default]
    None,
    X,
    Y,
    Both,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipInstant {
    #[default]
    Delay,
    Instant,
}

#[derive(Clone, Default)]
pub struct TooltipDelayGroup {
    state: Rc<RefCell<TooltipDelayGroupState>>,
}

#[derive(Default)]
struct TooltipDelayGroupState {
    recently_visible: bool,
    generation: u64,
    active_root_id: Option<ElementId>,
}

impl TooltipDelayGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn instant(&self) -> TooltipInstant {
        match self.state.borrow().recently_visible {
            true => TooltipInstant::Instant,
            false => TooltipInstant::Delay,
        }
    }

    pub fn should_open_instantly(&self) -> bool {
        self.state.borrow().recently_visible
    }

    pub fn mark_recently_visible(&self) -> u64 {
        let mut state = self.state.borrow_mut();
        state.recently_visible = true;
        state.generation = state.generation.wrapping_add(1);
        state.generation
    }

    pub fn clear_recently_visible(&self, generation: u64) -> bool {
        let mut state = self.state.borrow_mut();
        if state.generation != generation {
            return false;
        }
        let changed = state.recently_visible;
        state.recently_visible = false;
        changed
    }

    pub fn active_root_id(&self) -> Option<ElementId> {
        self.state.borrow().active_root_id.clone()
    }

    pub fn claim_active_root(&self, root_id: ElementId) {
        self.state.borrow_mut().active_root_id = Some(root_id);
    }

    pub fn clear_active_root(&self, root_id: &ElementId) -> bool {
        let mut state = self.state.borrow_mut();
        if state.active_root_id.as_ref() == Some(root_id) {
            state.active_root_id = None;
            return true;
        }
        false
    }
}

pub type TooltipOpenChangeHandler<P> =
    Rc<dyn Fn(bool, &mut TooltipOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type TooltipOpenChangeCompleteHandler<P> =
    Rc<dyn Fn(bool, &TooltipOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type TooltipPayloadContentBuilder<P> =
    Rc<dyn Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TooltipProviderConfig {
    delay: Duration,
    close_delay: Duration,
    timeout: Duration,
}

impl Default for TooltipProviderConfig {
    fn default() -> Self {
        Self {
            delay: DEFAULT_TOOLTIP_DELAY,
            close_delay: DEFAULT_TOOLTIP_CLOSE_DELAY,
            timeout: DEFAULT_TOOLTIP_TIMEOUT,
        }
    }
}

impl TooltipProviderConfig {
    pub fn new(delay: Duration, close_delay: Duration, timeout: Duration) -> Self {
        Self {
            delay,
            close_delay,
            timeout,
        }
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_close_delay(mut self, close_delay: Duration) -> Self {
        self.close_delay = close_delay;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

pub struct TooltipProps<P: Clone + 'static> {
    disabled: bool,
    disable_hoverable_popup: bool,
    track_cursor_axis: TooltipTrackCursorAxis,
    provider: TooltipProviderConfig,
    delay_group: TooltipDelayGroup,
    on_open_change: Option<TooltipOpenChangeHandler<P>>,
    on_open_change_complete: Option<TooltipOpenChangeCompleteHandler<P>>,
}

impl<P: Clone + 'static> Clone for TooltipProps<P> {
    fn clone(&self) -> Self {
        Self {
            disabled: self.disabled,
            disable_hoverable_popup: self.disable_hoverable_popup,
            track_cursor_axis: self.track_cursor_axis,
            provider: self.provider,
            delay_group: self.delay_group.clone(),
            on_open_change: self.on_open_change.clone(),
            on_open_change_complete: self.on_open_change_complete.clone(),
        }
    }
}

impl<P: Clone + 'static> TooltipProps<P> {
    pub fn new(
        disabled: bool,
        disable_hoverable_popup: bool,
        track_cursor_axis: TooltipTrackCursorAxis,
        provider: TooltipProviderConfig,
        on_open_change: Option<TooltipOpenChangeHandler<P>>,
        on_open_change_complete: Option<TooltipOpenChangeCompleteHandler<P>>,
    ) -> Self {
        Self {
            disabled,
            disable_hoverable_popup,
            track_cursor_axis,
            provider,
            delay_group: TooltipDelayGroup::default(),
            on_open_change,
            on_open_change_complete,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn disable_hoverable_popup(&self) -> bool {
        self.disable_hoverable_popup
    }

    pub fn track_cursor_axis(&self) -> TooltipTrackCursorAxis {
        self.track_cursor_axis
    }

    pub fn provider(&self) -> TooltipProviderConfig {
        self.provider
    }

    pub fn delay_group(&self) -> TooltipDelayGroup {
        self.delay_group.clone()
    }

    pub fn with_delay_group(mut self, delay_group: TooltipDelayGroup) -> Self {
        self.delay_group = delay_group;
        self
    }

    pub fn on_open_change(&self) -> Option<&TooltipOpenChangeHandler<P>> {
        self.on_open_change.as_ref()
    }

    pub fn on_open_change_complete(&self) -> Option<&TooltipOpenChangeCompleteHandler<P>> {
        self.on_open_change_complete.as_ref()
    }
}
