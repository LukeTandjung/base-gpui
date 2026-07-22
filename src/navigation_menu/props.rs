use std::{rc::Rc, time::Duration};

use gpui::{App, Window};

use crate::navigation_menu::NavigationMenuValueChangeDetails;

/// Base UI `OPEN_DELAY`: hover-open waits 50ms by default.
pub const DEFAULT_NAVIGATION_MENU_DELAY: Duration = Duration::from_millis(50);
/// Base UI `CLOSE_DELAY`: hover-close waits 50ms by default.
pub const DEFAULT_NAVIGATION_MENU_CLOSE_DELAY: Duration = Duration::from_millis(50);
/// Base UI `PATIENT_CLICK_THRESHOLD`: clicks within this window after a hover
/// open keep the menu open instead of toggling it closed.
pub const NAVIGATION_MENU_PATIENT_CLICK_THRESHOLD: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuAlign {
    Start,
    #[default]
    Center,
    End,
}

/// Transition suppression facts: `Initial` for a menu that rendered already
/// open, `Resize` around window-resize repositioning.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuInstant {
    #[default]
    None,
    Initial,
    Resize,
}

pub type NavigationMenuValueChangeHandler<T> =
    Rc<dyn Fn(Option<T>, &mut NavigationMenuValueChangeDetails, &mut Window, &mut App) + 'static>;

pub type NavigationMenuOpenChangeCompleteHandler<T> =
    Rc<dyn Fn(Option<T>, &NavigationMenuValueChangeDetails, &mut Window, &mut App) + 'static>;

pub struct NavigationMenuProps<T: Clone + Eq + 'static> {
    delay: Duration,
    close_delay: Duration,
    orientation: NavigationMenuOrientation,
    on_value_change: Option<NavigationMenuValueChangeHandler<T>>,
    on_open_change_complete: Option<NavigationMenuOpenChangeCompleteHandler<T>>,
}

impl<T: Clone + Eq + 'static> Clone for NavigationMenuProps<T> {
    fn clone(&self) -> Self {
        Self {
            delay: self.delay,
            close_delay: self.close_delay,
            orientation: self.orientation,
            on_value_change: self.on_value_change.clone(),
            on_open_change_complete: self.on_open_change_complete.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuProps<T> {
    pub fn new(
        delay: Duration,
        close_delay: Duration,
        orientation: NavigationMenuOrientation,
        on_value_change: Option<NavigationMenuValueChangeHandler<T>>,
        on_open_change_complete: Option<NavigationMenuOpenChangeCompleteHandler<T>>,
    ) -> Self {
        Self {
            delay,
            close_delay,
            orientation,
            on_value_change,
            on_open_change_complete,
        }
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
    }

    pub fn orientation(&self) -> NavigationMenuOrientation {
        self.orientation
    }

    pub fn on_value_change(&self) -> Option<&NavigationMenuValueChangeHandler<T>> {
        self.on_value_change.as_ref()
    }

    pub fn on_open_change_complete(&self) -> Option<&NavigationMenuOpenChangeCompleteHandler<T>> {
        self.on_open_change_complete.as_ref()
    }
}
