use gpui::{Bounds, Pixels, SharedString, Size};

use crate::{
    combobox::{ComboboxAlign, ComboboxSelectionMode, ComboboxSide},
    utils::PresenceState,
};

#[derive(Clone, PartialEq)]
pub struct ComboboxRootStyleState<T: Clone + Eq + 'static> {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub open: bool,
    pub focused: bool,
    pub filled: bool,
    pub dirty: bool,
    pub touched: bool,
    pub valid: Option<bool>,
    pub invalid: bool,
    pub selection_mode: ComboboxSelectionMode,
    pub value_present: bool,
    pub input_non_empty: bool,
    pub list_empty: bool,
    pub selected_value: Option<T>,
    pub selected_values: Vec<T>,
    pub input_value: SharedString,
}

impl<T: Clone + Eq + 'static> ComboboxRootStyleState<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disabled: bool,
        read_only: bool,
        required: bool,
        open: bool,
        focused: bool,
        dirty: bool,
        touched: bool,
        valid: Option<bool>,
        selection_mode: ComboboxSelectionMode,
        selected_value: Option<T>,
        selected_values: Vec<T>,
        input_value: SharedString,
        list_empty: bool,
    ) -> Self {
        let value_present = match selection_mode {
            ComboboxSelectionMode::Single => selected_value.is_some(),
            ComboboxSelectionMode::Multiple => !selected_values.is_empty(),
            ComboboxSelectionMode::None => !input_value.is_empty(),
        };

        Self {
            disabled,
            read_only,
            required,
            open,
            focused,
            filled: value_present,
            dirty,
            touched,
            valid,
            invalid: valid == Some(false),
            selection_mode,
            value_present,
            input_non_empty: !input_value.is_empty(),
            list_empty,
            selected_value,
            selected_values,
            input_value,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ComboboxInputStyleState<T: Clone + Eq + 'static> {
    pub root: ComboboxRootStyleState<T>,
    pub popup_side: ComboboxSide,
}

impl<T: Clone + Eq + 'static> ComboboxInputStyleState<T> {
    pub fn new(root: ComboboxRootStyleState<T>, popup_side: ComboboxSide) -> Self {
        Self { root, popup_side }
    }
}

#[derive(Clone, PartialEq)]
pub struct ComboboxInputGroupStyleState<T: Clone + Eq + 'static> {
    pub root: ComboboxRootStyleState<T>,
    pub placeholder: bool,
    pub popup_side: ComboboxSide,
}

impl<T: Clone + Eq + 'static> ComboboxInputGroupStyleState<T> {
    pub fn new(root: ComboboxRootStyleState<T>, popup_side: ComboboxSide) -> Self {
        Self {
            placeholder: !root.value_present,
            root,
            popup_side,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ComboboxTriggerStyleState<T: Clone + Eq + 'static> {
    pub root: ComboboxRootStyleState<T>,
    pub placeholder: bool,
    pub popup_side: ComboboxSide,
}

impl<T: Clone + Eq + 'static> ComboboxTriggerStyleState<T> {
    pub fn new(root: ComboboxRootStyleState<T>, popup_side: ComboboxSide) -> Self {
        Self {
            placeholder: !root.value_present,
            root,
            popup_side,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct ComboboxValueStyleState<T: Clone + Eq + 'static> {
    pub selection_mode: ComboboxSelectionMode,
    pub selected_value: Option<T>,
    pub selected_values: Vec<T>,
    pub selected_labels: Vec<SharedString>,
    pub placeholder: bool,
    pub display_text: SharedString,
}

impl<T: Clone + Eq + 'static> ComboboxValueStyleState<T> {
    pub fn new(
        selection_mode: ComboboxSelectionMode,
        selected_value: Option<T>,
        selected_values: Vec<T>,
        selected_labels: Vec<SharedString>,
        placeholder: bool,
        display_text: SharedString,
    ) -> Self {
        Self {
            selection_mode,
            selected_value,
            selected_values,
            selected_labels,
            placeholder,
            display_text,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxIconStyleState {
    pub open: bool,
}

impl ComboboxIconStyleState {
    pub fn new(open: bool) -> Self {
        Self { open }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxLabelStyleState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxStatusStyleState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxEmptyStyleState {
    pub empty: bool,
}

impl ComboboxEmptyStyleState {
    pub fn new(empty: bool) -> Self {
        Self { empty }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxPortalStyleState {
    pub open: bool,
    pub mounted: bool,
}

impl ComboboxPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self { open, mounted }
    }
}

#[derive(Clone, PartialEq)]
pub struct ComboboxPositionerStyleState {
    pub open: bool,
    pub side: ComboboxSide,
    pub align: ComboboxAlign,
    pub empty: bool,
    pub anchor_hidden: bool,
    pub anchor_bounds: Option<Bounds<Pixels>>,
    pub popup_bounds: Option<Bounds<Pixels>>,
    pub available_size: Option<Size<Pixels>>,
    pub anchor_width: Option<Pixels>,
    pub anchor_height: Option<Pixels>,
    pub popup_width: Option<Pixels>,
    pub popup_height: Option<Pixels>,
}

impl ComboboxPositionerStyleState {
    pub fn new(
        open: bool,
        side: ComboboxSide,
        align: ComboboxAlign,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
        empty: bool,
    ) -> Self {
        Self {
            open,
            side,
            align,
            empty,
            anchor_hidden: anchor_bounds.is_none(),
            anchor_width: anchor_bounds.map(|bounds| bounds.size.width),
            anchor_height: anchor_bounds.map(|bounds| bounds.size.height),
            popup_width: popup_bounds.map(|bounds| bounds.size.width),
            popup_height: popup_bounds.map(|bounds| bounds.size.height),
            anchor_bounds,
            popup_bounds,
            available_size,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: ComboboxSide,
    pub align: ComboboxAlign,
    pub empty: bool,
    pub transitioning: bool,
}

impl ComboboxPopupStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        side: ComboboxSide,
        align: ComboboxAlign,
        empty: bool,
    ) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            side,
            align,
            empty,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxBackdropStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
}

impl ComboboxBackdropStyleState {
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
pub struct ComboboxArrowStyleState {
    pub open: bool,
    pub side: ComboboxSide,
    pub align: ComboboxAlign,
    pub uncentered: bool,
}

impl ComboboxArrowStyleState {
    pub fn new(open: bool, side: ComboboxSide, align: ComboboxAlign, uncentered: bool) -> Self {
        Self {
            open,
            side,
            align,
            uncentered,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxListStyleState {
    pub open: bool,
    pub item_count: usize,
    pub empty: bool,
}

impl ComboboxListStyleState {
    pub fn new(open: bool, item_count: usize, empty: bool) -> Self {
        Self {
            open,
            item_count,
            empty,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComboboxGroupStyleState {
    pub item_count: usize,
    pub group_index: Option<usize>,
    pub label: Option<SharedString>,
}

impl ComboboxGroupStyleState {
    pub fn new(item_count: usize, group_index: Option<usize>, label: Option<SharedString>) -> Self {
        Self {
            item_count,
            group_index,
            label,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxGroupLabelStyleState;

#[derive(Clone, PartialEq)]
pub struct ComboboxItemStyleState<T: Clone + Eq + 'static> {
    pub selected: bool,
    pub highlighted: bool,
    pub disabled: bool,
    pub root_disabled: bool,
    pub visible: bool,
    pub index: Option<usize>,
    pub value: Option<T>,
}

impl<T: Clone + Eq + 'static> ComboboxItemStyleState<T> {
    pub fn new(
        selected: bool,
        highlighted: bool,
        disabled: bool,
        root_disabled: bool,
        visible: bool,
        index: Option<usize>,
        value: Option<T>,
    ) -> Self {
        Self {
            selected,
            highlighted,
            disabled,
            root_disabled,
            visible,
            index,
            value,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxItemIndicatorStyleState {
    pub selected: bool,
    pub present: bool,
    pub transitioning: bool,
}

impl ComboboxItemIndicatorStyleState {
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
pub struct ComboboxChipsStyleState {
    pub chip_count: usize,
}

impl ComboboxChipsStyleState {
    pub fn new(chip_count: usize) -> Self {
        Self { chip_count }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxChipStyleState {
    pub highlighted: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub index: usize,
}

impl ComboboxChipStyleState {
    pub fn new(highlighted: bool, disabled: bool, read_only: bool, index: usize) -> Self {
        Self {
            highlighted,
            disabled,
            read_only,
            index,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxChipRemoveStyleState {
    pub disabled: bool,
}

impl ComboboxChipRemoveStyleState {
    pub fn new(disabled: bool) -> Self {
        Self { disabled }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxClearStyleState {
    pub visible: bool,
    pub disabled: bool,
    pub open: bool,
}

impl ComboboxClearStyleState {
    pub fn new(visible: bool, disabled: bool, open: bool) -> Self {
        Self {
            visible,
            disabled,
            open,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxSeparatorStyleState;
