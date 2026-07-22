use gpui::{ElementId, Pixels, Point, Rems};

use crate::dialog::{
    DialogBackdropStyleState, DialogModalMode, DialogPopupStyleState, DialogViewportStyleState,
};

/// The direction the drawer swipes toward to dismiss.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DrawerSwipeDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl DrawerSwipeDirection {
    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
    }

    pub fn is_horizontal(self) -> bool {
        !self.is_vertical()
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    /// Unit vector pointing toward dismissal, in (x, y) screen coordinates.
    pub fn dismiss_unit(self) -> (f32, f32) {
        match self {
            Self::Up => (0.0, -1.0),
            Self::Down => (0.0, 1.0),
            Self::Left => (-1.0, 0.0),
            Self::Right => (1.0, 0.0),
        }
    }
}

/// A drawer snap point. `Fraction` covers Base UI numbers `<= 1` (fraction of the
/// viewport height); `Px` covers absolute pixel values; `Rems` resolves against the
/// window rem size.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerSnapPoint {
    Fraction(f32),
    Px(Pixels),
    Rems(Rems),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawerPopupStyleState<P: Clone + 'static> {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
    pub expanded: bool,
    pub nested: bool,
    pub nested_drawer_open: bool,
    pub nested_drawer_count: usize,
    pub nested_drawer_swiping: bool,
    pub nested_swipe_progress: f32,
    pub swipe_direction: DrawerSwipeDirection,
    pub swiping: bool,
    pub swipe_movement: Point<Pixels>,
    pub snap_point_offset: Pixels,
    pub popup_height: Option<Pixels>,
    pub frontmost_height: Option<Pixels>,
    pub swipe_strength: f32,
    pub swipe_dismissed: bool,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
    pub modal_mode: DialogModalMode,
}

impl<P: Clone + 'static> DrawerPopupStyleState<P> {
    pub fn from_dialog(dialog: DialogPopupStyleState<P>, drawer: DrawerPopupFacts) -> Self {
        Self {
            open: dialog.open,
            closed: dialog.closed,
            mounted: dialog.mounted,
            present: dialog.present,
            transitioning: dialog.transitioning,
            expanded: drawer.expanded,
            nested: drawer.nested,
            nested_drawer_open: drawer.nested_drawer_count > 0,
            nested_drawer_count: drawer.nested_drawer_count,
            nested_drawer_swiping: drawer.nested_drawer_swiping,
            nested_swipe_progress: drawer.nested_swipe_progress,
            swipe_direction: drawer.swipe_direction,
            swiping: drawer.swiping,
            swipe_movement: drawer.swipe_movement,
            snap_point_offset: drawer.snap_point_offset,
            popup_height: drawer.popup_height,
            frontmost_height: drawer.frontmost_height,
            swipe_strength: drawer.swipe_strength,
            swipe_dismissed: drawer.swipe_dismissed,
            active_trigger_id: dialog.active_trigger_id,
            active_payload: dialog.active_payload,
            payload_present: dialog.payload_present,
            modal_mode: dialog.modal_mode,
        }
    }
}

/// Drawer-runtime facts merged into the reused dialog part states.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawerPopupFacts {
    pub expanded: bool,
    pub nested: bool,
    pub nested_drawer_count: usize,
    pub nested_drawer_swiping: bool,
    pub nested_swipe_progress: f32,
    pub swipe_direction: DrawerSwipeDirection,
    pub swiping: bool,
    pub swipe_movement: Point<Pixels>,
    pub snap_point_offset: Pixels,
    pub popup_height: Option<Pixels>,
    pub frontmost_height: Option<Pixels>,
    pub swipe_strength: f32,
    pub swipe_dismissed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawerBackdropStyleState {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
    pub swipe_progress: f32,
    pub frontmost_height: Option<Pixels>,
    pub swiping: bool,
    pub swipe_dismissed: bool,
    pub nested: bool,
    pub force_rendered: bool,
    pub rendered: bool,
}

impl DrawerBackdropStyleState {
    pub fn from_dialog(
        dialog: DialogBackdropStyleState,
        nested: bool,
        force_rendered: bool,
        swipe_progress: f32,
        frontmost_height: Option<Pixels>,
        swiping: bool,
        swipe_dismissed: bool,
    ) -> Self {
        Self {
            open: dialog.open,
            closed: dialog.closed,
            mounted: dialog.mounted,
            present: dialog.present,
            transitioning: dialog.transitioning,
            swipe_progress,
            frontmost_height,
            swiping,
            swipe_dismissed,
            nested,
            force_rendered,
            rendered: dialog.present && (!nested || force_rendered),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawerViewportStyleState<P: Clone + 'static> {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
    pub nested: bool,
    pub nested_drawer_open: bool,
    pub nested_drawer_count: usize,
    pub nested_drawer_swiping: bool,
    pub swiping: bool,
    pub swipe_progress: f32,
    pub swipe_direction: DrawerSwipeDirection,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
}

impl<P: Clone + 'static> DrawerViewportStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_dialog(
        dialog: DialogViewportStyleState<P>,
        nested: bool,
        nested_drawer_count: usize,
        nested_drawer_swiping: bool,
        swiping: bool,
        swipe_progress: f32,
        swipe_direction: DrawerSwipeDirection,
    ) -> Self {
        Self {
            open: dialog.open,
            closed: dialog.closed,
            mounted: dialog.mounted,
            present: dialog.present,
            transitioning: dialog.transitioning,
            nested,
            nested_drawer_open: nested_drawer_count > 0,
            nested_drawer_count,
            nested_drawer_swiping,
            swiping,
            swipe_progress,
            swipe_direction,
            active_trigger_id: dialog.active_trigger_id,
            active_payload: dialog.active_payload,
            payload_present: dialog.payload_present,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawerSwipeAreaStyleState {
    pub open: bool,
    pub swiping: bool,
    pub swipe_direction: DrawerSwipeDirection,
    pub disabled: bool,
}

impl DrawerSwipeAreaStyleState {
    pub fn new(
        open: bool,
        swiping: bool,
        swipe_direction: DrawerSwipeDirection,
        disabled: bool,
    ) -> Self {
        Self {
            open,
            swiping,
            swipe_direction,
            disabled,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DrawerContentStyleState {
    pub open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawerIndentStyleState {
    pub active: bool,
    pub swipe_progress: f32,
    pub frontmost_height: Option<Pixels>,
}

impl Default for DrawerIndentStyleState {
    fn default() -> Self {
        Self {
            active: false,
            swipe_progress: 0.0,
            frontmost_height: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DrawerIndentBackgroundStyleState {
    pub active: bool,
}
