use std::rc::Rc;

use gpui::{App, Window};

use crate::dialog::DialogOpenChangeReason;
use crate::drawer::{DrawerSnapPoint, DrawerSwipeDirection};

/// Cancelable details passed to `on_snap_point_change`, reusing the dialog
/// reason vocabulary (`Swipe` for gesture-driven changes).
#[derive(Clone, Debug)]
pub struct DrawerSnapPointChangeDetails {
    reason: DialogOpenChangeReason,
    cancelable: bool,
    canceled: bool,
}

impl DrawerSnapPointChangeDetails {
    pub fn new(reason: DialogOpenChangeReason, cancelable: bool) -> Self {
        Self {
            reason,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> DialogOpenChangeReason {
        self.reason
    }

    pub fn cancelable(&self) -> bool {
        self.cancelable
    }

    pub fn cancel(&mut self) {
        if self.cancelable {
            self.canceled = true;
        }
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }
}

pub type DrawerSnapPointChangeHandler = Rc<
    dyn Fn(Option<DrawerSnapPoint>, &mut DrawerSnapPointChangeDetails, &mut Window, &mut App)
        + 'static,
>;

/// Drawer-specific root configuration. Dialog-level props stay on
/// `DialogProps<P>`.
pub struct DrawerProps {
    swipe_direction: DrawerSwipeDirection,
    snap_points: Vec<DrawerSnapPoint>,
    snap_to_sequential_points: bool,
    default_snap_point: Option<DrawerSnapPoint>,
    on_snap_point_change: Option<DrawerSnapPointChangeHandler>,
}

impl Clone for DrawerProps {
    fn clone(&self) -> Self {
        Self {
            swipe_direction: self.swipe_direction,
            snap_points: self.snap_points.clone(),
            snap_to_sequential_points: self.snap_to_sequential_points,
            default_snap_point: self.default_snap_point,
            on_snap_point_change: self.on_snap_point_change.clone(),
        }
    }
}

impl DrawerProps {
    pub fn new(
        swipe_direction: DrawerSwipeDirection,
        snap_points: Vec<DrawerSnapPoint>,
        snap_to_sequential_points: bool,
        default_snap_point: Option<DrawerSnapPoint>,
        on_snap_point_change: Option<DrawerSnapPointChangeHandler>,
    ) -> Self {
        Self {
            swipe_direction,
            snap_points,
            snap_to_sequential_points,
            default_snap_point,
            on_snap_point_change,
        }
    }

    pub fn swipe_direction(&self) -> DrawerSwipeDirection {
        self.swipe_direction
    }

    pub fn snap_points(&self) -> &Vec<DrawerSnapPoint> {
        &self.snap_points
    }

    pub fn snap_to_sequential_points(&self) -> bool {
        self.snap_to_sequential_points
    }

    pub fn default_snap_point(&self) -> Option<DrawerSnapPoint> {
        self.default_snap_point
    }

    pub fn on_snap_point_change(&self) -> Option<&DrawerSnapPointChangeHandler> {
        self.on_snap_point_change.as_ref()
    }
}
