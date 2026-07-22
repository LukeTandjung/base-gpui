use gpui::{Bounds, Pixels, SharedString, Size};

use crate::{
    select::{SelectAlign, SelectOpenChangeSource, SelectSelectionMode, SelectSide},
    utils::PresenceState,
};

#[derive(Clone, PartialEq)]
pub struct SelectRootStyleState<T: Clone + Eq + 'static> {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub open: bool,
    pub open_source: SelectOpenChangeSource,
    pub touch_open: bool,
    pub focused: bool,
    pub filled: bool,
    pub dirty: bool,
    pub touched: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub selection_mode: SelectSelectionMode,
    pub value_present: bool,
    pub selected_value: Option<T>,
    pub selected_values: Vec<T>,
    pub selected_index: Option<usize>,
}

impl<T: Clone + Eq + 'static> SelectRootStyleState<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disabled: bool,
        read_only: bool,
        required: bool,
        open: bool,
        open_source: SelectOpenChangeSource,
        focused: bool,
        dirty: bool,
        touched: bool,
        valid: Option<bool>,
        selection_mode: SelectSelectionMode,
        selected_value: Option<T>,
        selected_values: Vec<T>,
        selected_index: Option<usize>,
    ) -> Self {
        let value_present = match selection_mode {
            SelectSelectionMode::Single => selected_value.is_some(),
            SelectSelectionMode::Multiple => !selected_values.is_empty(),
        };

        Self {
            disabled,
            read_only,
            required,
            open,
            open_source,
            touch_open: open_source == SelectOpenChangeSource::Touch,
            focused,
            filled: value_present,
            dirty,
            touched,
            valid,
            invalid: valid == Some(false),
            selection_mode,
            value_present,
            selected_value,
            selected_values,
            selected_index,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SelectTriggerStyleState<T: Clone + Eq + 'static> {
    pub root: SelectRootStyleState<T>,
    pub placeholder: bool,
    pub popup_side: SelectSide,
}

impl<T: Clone + Eq + 'static> SelectTriggerStyleState<T> {
    pub fn new(root: SelectRootStyleState<T>, popup_side: SelectSide) -> Self {
        Self {
            placeholder: !root.value_present,
            root,
            popup_side,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SelectValueStyleState<T: Clone + Eq + 'static> {
    pub selection_mode: SelectSelectionMode,
    pub selected_value: Option<T>,
    pub selected_values: Vec<T>,
    pub selected_labels: Vec<SharedString>,
    pub placeholder: bool,
    pub value_present: bool,
    pub display_text: SharedString,
}

impl<T: Clone + Eq + 'static> SelectValueStyleState<T> {
    pub fn new(
        selection_mode: SelectSelectionMode,
        selected_value: Option<T>,
        selected_values: Vec<T>,
        selected_labels: Vec<SharedString>,
        placeholder: bool,
        display_text: SharedString,
    ) -> Self {
        let value_present = match selection_mode {
            SelectSelectionMode::Single => selected_value.is_some(),
            SelectSelectionMode::Multiple => !selected_values.is_empty(),
        };

        Self {
            selection_mode,
            selected_value,
            selected_values,
            selected_labels,
            placeholder,
            value_present,
            display_text,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectIconStyleState {
    pub open: bool,
}

impl SelectIconStyleState {
    pub fn new(open: bool) -> Self {
        Self { open }
    }
}

#[derive(Clone, PartialEq)]
pub struct SelectPositionerStyleState {
    pub open: bool,
    pub open_source: SelectOpenChangeSource,
    pub touch_open: bool,
    pub side: SelectSide,
    pub align: SelectAlign,
    pub anchor_hidden: bool,
    pub anchor_available: bool,
    pub anchor_bounds: Option<Bounds<Pixels>>,
    pub popup_bounds: Option<Bounds<Pixels>>,
    pub available_size: Option<Size<Pixels>>,
    pub anchor_width: Option<Pixels>,
    pub anchor_height: Option<Pixels>,
    pub popup_width: Option<Pixels>,
    pub popup_height: Option<Pixels>,
    pub available_width: Option<Pixels>,
    pub available_height: Option<Pixels>,
    pub align_item_with_trigger_active: bool,
    pub align_item_transform_origin_y_percent: Option<f32>,
}

impl SelectPositionerStyleState {
    pub fn new(
        open: bool,
        open_source: SelectOpenChangeSource,
        side: SelectSide,
        align: SelectAlign,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
    ) -> Self {
        Self {
            open,
            open_source,
            touch_open: open_source == SelectOpenChangeSource::Touch,
            side,
            align,
            anchor_hidden: anchor_bounds.is_none(),
            anchor_available: anchor_bounds.is_some(),
            anchor_width: anchor_bounds.map(|bounds| bounds.size.width),
            anchor_height: anchor_bounds.map(|bounds| bounds.size.height),
            popup_width: popup_bounds.map(|bounds| bounds.size.width),
            popup_height: popup_bounds.map(|bounds| bounds.size.height),
            available_width: available_size.map(|size| size.width),
            available_height: available_size.map(|size| size.height),
            align_item_with_trigger_active: false,
            align_item_transform_origin_y_percent: None,
            anchor_bounds,
            popup_bounds,
            available_size,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectPortalStyleState {
    pub open: bool,
    pub mounted: bool,
}

impl SelectPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self { open, mounted }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: SelectSide,
    pub align: SelectAlign,
    pub transitioning: bool,
}

impl SelectPopupStyleState {
    pub fn new(open: bool, mounted: bool, side: SelectSide, align: SelectAlign) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            side,
            align,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectBackdropStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
}

impl SelectBackdropStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectArrowStyleState {
    pub open: bool,
    pub side: SelectSide,
    pub align: SelectAlign,
    pub uncentered: bool,
}

impl SelectArrowStyleState {
    pub fn new(open: bool, side: SelectSide, align: SelectAlign, uncentered: bool) -> Self {
        Self {
            open,
            side,
            align,
            uncentered,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SelectItemStyleState<T: Clone + Eq + 'static> {
    pub selected: bool,
    pub highlighted: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub root_disabled: bool,
    pub focused: bool,
    pub tab_stop: bool,
    pub index: Option<usize>,
    pub group_index: Option<usize>,
    pub item_bounds: Option<Bounds<Pixels>>,
    pub item_text_bounds: Option<Bounds<Pixels>>,
    pub value: Option<T>,
}

impl<T: Clone + Eq + 'static> SelectItemStyleState<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        selected: bool,
        highlighted: bool,
        disabled: bool,
        read_only: bool,
        root_disabled: bool,
        focused: bool,
        tab_stop: bool,
        index: Option<usize>,
        group_index: Option<usize>,
        item_bounds: Option<Bounds<Pixels>>,
        item_text_bounds: Option<Bounds<Pixels>>,
        value: Option<T>,
    ) -> Self {
        Self {
            selected,
            highlighted,
            disabled,
            read_only,
            root_disabled,
            focused,
            tab_stop,
            index,
            group_index,
            item_bounds,
            item_text_bounds,
            value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectItemIndicatorStyleState {
    pub selected: bool,
    pub present: bool,
    pub transitioning: bool,
}

impl SelectItemIndicatorStyleState {
    pub fn new(selected: bool, present: bool) -> Self {
        let presence = PresenceState::new(selected, present);
        Self {
            selected,
            present: presence.present,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectScrollArrowDirection {
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectScrollArrowStyleState {
    pub direction: SelectScrollArrowDirection,
    pub visible: bool,
    pub present: bool,
    pub side: SelectSide,
    pub transitioning: bool,
}

impl SelectScrollArrowStyleState {
    pub fn new(
        direction: SelectScrollArrowDirection,
        visible: bool,
        side: SelectSide,
        keep_mounted: bool,
    ) -> Self {
        let presence = PresenceState::new(visible, keep_mounted);
        Self {
            direction,
            visible,
            present: presence.present,
            side,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectListStyleState {
    pub open: bool,
    pub item_count: usize,
}

impl SelectListStyleState {
    pub fn new(open: bool, item_count: usize) -> Self {
        Self { open, item_count }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectGroupStyleState {
    pub item_count: usize,
    pub group_index: Option<usize>,
    pub label: Option<SharedString>,
}

impl SelectGroupStyleState {
    pub fn new(item_count: usize, group_index: Option<usize>, label: Option<SharedString>) -> Self {
        Self {
            item_count,
            group_index,
            label,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectGroupLabelStyleState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectItemTextStyleState {
    pub selected: bool,
    pub highlighted: bool,
    pub bounds: Option<Bounds<Pixels>>,
}

impl SelectItemTextStyleState {
    pub fn new(selected: bool, highlighted: bool, bounds: Option<Bounds<Pixels>>) -> Self {
        Self {
            selected,
            highlighted,
            bounds,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectSeparatorStyleState;
