use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::combobox::{ComboboxChangeDetails, ComboboxItemHighlightDetails};

/// Selection mode for the Combobox core. `ComboboxRoot` only exposes
/// Single/Multiple via `.multiple(bool)`; `None` is kept public for the
/// Autocomplete port, which is this core with `selection_mode = None`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ComboboxSelectionMode {
    #[default]
    Single,
    Multiple,
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ComboboxSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ComboboxAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Base UI `autoHighlight: boolean | 'always'`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ComboboxAutoHighlight {
    #[default]
    Off,
    OnInputChange,
    Always,
}

pub type ComboboxValueChangeHandler<T> =
    Rc<dyn Fn(Option<&T>, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static>;

pub type ComboboxValuesChangeHandler<T> =
    Rc<dyn Fn(&[T], &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static>;

pub type ComboboxInputValueChangeHandler =
    Rc<dyn Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static>;

pub type ComboboxOpenChangeHandler =
    Rc<dyn Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static>;

pub type ComboboxItemHighlightedHandler<T> =
    Rc<dyn Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App) + 'static>;

pub type ComboboxLabelResolver<T> = Rc<dyn Fn(&T) -> SharedString + 'static>;

pub type ComboboxValueSerializer<T> = Rc<dyn Fn(&T) -> SharedString + 'static>;

/// Custom filter: `(item value, resolved label, trimmed query) -> keep`.
pub type ComboboxFilter<T> = Rc<dyn Fn(&T, Option<&SharedString>, &str) -> bool + 'static>;

pub type ComboboxMultipleValueFormatter<T> =
    Rc<dyn Fn(&[SharedString], &[T]) -> SharedString + 'static>;

/// Full configuration surface of the Combobox core. `ComboboxRoot` hides the
/// Autocomplete-only knobs (`selection_mode = None`, `fill_input_on_item_press`,
/// `keep_highlight`, `submit_on_item_click`) from its builder, but they stay
/// public and configurable here for the Autocomplete port.
pub struct ComboboxProps<T: Clone + Eq + 'static> {
    pub name: Option<SharedString>,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub selection_mode: ComboboxSelectionMode,
    pub open_on_input_click: bool,
    pub auto_highlight: ComboboxAutoHighlight,
    pub highlight_item_on_hover: bool,
    pub loop_focus: bool,
    pub keep_highlight: bool,
    /// None mode only: pressing an item writes its label into the input.
    pub fill_input_on_item_press: bool,
    /// Documented no-op hook until Form exposes programmatic submit.
    pub submit_on_item_click: bool,
    /// Autocomplete seam: keyboard/programmatic highlight previews the
    /// highlighted item's label in the input as an inline overlay.
    pub inline_autocomplete: bool,
    pub limit: Option<usize>,
    /// `None` with `filter_disabled == false` means the default
    /// case-insensitive contains match on the item label. Locale-aware
    /// collation is intentionally not ported; use a custom filter if needed.
    pub filter: Option<ComboboxFilter<T>>,
    /// Disables internal filtering entirely for externally filtered lists
    /// (Base UI `filter={null}`).
    pub filter_disabled: bool,
    pub label_resolver: Option<ComboboxLabelResolver<T>>,
    pub value_serializer: Option<ComboboxValueSerializer<T>>,
    pub multiple_value_formatter: Option<ComboboxMultipleValueFormatter<T>>,
    pub on_value_change: Option<ComboboxValueChangeHandler<T>>,
    pub on_values_change: Option<ComboboxValuesChangeHandler<T>>,
    pub on_input_value_change: Option<ComboboxInputValueChangeHandler>,
    pub on_open_change: Option<ComboboxOpenChangeHandler>,
    pub on_item_highlighted: Option<ComboboxItemHighlightedHandler<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxProps<T> {
    fn default() -> Self {
        Self {
            name: None,
            disabled: false,
            read_only: false,
            required: false,
            selection_mode: ComboboxSelectionMode::Single,
            open_on_input_click: true,
            auto_highlight: ComboboxAutoHighlight::Off,
            highlight_item_on_hover: true,
            loop_focus: true,
            keep_highlight: false,
            fill_input_on_item_press: true,
            submit_on_item_click: false,
            inline_autocomplete: false,
            limit: None,
            filter: None,
            filter_disabled: false,
            label_resolver: None,
            value_serializer: None,
            multiple_value_formatter: None,
            on_value_change: None,
            on_values_change: None,
            on_input_value_change: None,
            on_open_change: None,
            on_item_highlighted: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Clone for ComboboxProps<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            disabled: self.disabled,
            read_only: self.read_only,
            required: self.required,
            selection_mode: self.selection_mode,
            open_on_input_click: self.open_on_input_click,
            auto_highlight: self.auto_highlight,
            highlight_item_on_hover: self.highlight_item_on_hover,
            loop_focus: self.loop_focus,
            keep_highlight: self.keep_highlight,
            fill_input_on_item_press: self.fill_input_on_item_press,
            submit_on_item_click: self.submit_on_item_click,
            inline_autocomplete: self.inline_autocomplete,
            limit: self.limit,
            filter: self.filter.clone(),
            filter_disabled: self.filter_disabled,
            label_resolver: self.label_resolver.clone(),
            value_serializer: self.value_serializer.clone(),
            multiple_value_formatter: self.multiple_value_formatter.clone(),
            on_value_change: self.on_value_change.clone(),
            on_values_change: self.on_values_change.clone(),
            on_input_value_change: self.on_input_value_change.clone(),
            on_open_change: self.on_open_change.clone(),
            on_item_highlighted: self.on_item_highlighted.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxProps<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label_for_value(&self, value: &T) -> Option<SharedString> {
        self.label_resolver.as_ref().map(|resolver| resolver(value))
    }

    pub fn serialize_value(&self, value: &T) -> Option<SharedString> {
        self.value_serializer
            .as_ref()
            .map(|serializer| serializer(value))
    }

    pub fn format_multiple_value(
        &self,
        labels: &[SharedString],
        values: &[T],
    ) -> Option<SharedString> {
        self.multiple_value_formatter
            .as_ref()
            .map(|formatter| formatter(labels, values))
    }
}
