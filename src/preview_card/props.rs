use std::{rc::Rc, time::Duration};

use gpui::{AnyElement, App, Window};

use crate::preview_card::PreviewCardOpenChangeDetails;

/// Base UI `OPEN_DELAY`: hover-open waits 600ms by default.
pub const DEFAULT_PREVIEW_CARD_DELAY: Duration = Duration::from_millis(600);
/// Base UI `CLOSE_DELAY`: hover-close waits 300ms by default.
pub const DEFAULT_PREVIEW_CARD_CLOSE_DELAY: Duration = Duration::from_millis(300);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PreviewCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PreviewCardAlign {
    Start,
    #[default]
    Center,
    End,
}

/// Base UI `instantType: 'focus' | 'dismiss'`; no delay-group variant exists
/// for Preview Card.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PreviewCardInstant {
    #[default]
    None,
    Focus,
    Dismiss,
}

pub type PreviewCardOpenChangeHandler<P> =
    Rc<dyn Fn(bool, &mut PreviewCardOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type PreviewCardOpenChangeCompleteHandler<P> =
    Rc<dyn Fn(bool, &PreviewCardOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type PreviewCardPayloadContentBuilder<P> =
    Rc<dyn Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static>;

pub struct PreviewCardProps<P: Clone + 'static> {
    delay: Duration,
    close_delay: Duration,
    on_open_change: Option<PreviewCardOpenChangeHandler<P>>,
    on_open_change_complete: Option<PreviewCardOpenChangeCompleteHandler<P>>,
}

impl<P: Clone + 'static> Clone for PreviewCardProps<P> {
    fn clone(&self) -> Self {
        Self {
            delay: self.delay,
            close_delay: self.close_delay,
            on_open_change: self.on_open_change.clone(),
            on_open_change_complete: self.on_open_change_complete.clone(),
        }
    }
}

impl<P: Clone + 'static> PreviewCardProps<P> {
    pub fn new(
        delay: Duration,
        close_delay: Duration,
        on_open_change: Option<PreviewCardOpenChangeHandler<P>>,
        on_open_change_complete: Option<PreviewCardOpenChangeCompleteHandler<P>>,
    ) -> Self {
        Self {
            delay,
            close_delay,
            on_open_change,
            on_open_change_complete,
        }
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
    }

    pub fn on_open_change(&self) -> Option<&PreviewCardOpenChangeHandler<P>> {
        self.on_open_change.as_ref()
    }

    pub fn on_open_change_complete(&self) -> Option<&PreviewCardOpenChangeCompleteHandler<P>> {
        self.on_open_change_complete.as_ref()
    }
}
