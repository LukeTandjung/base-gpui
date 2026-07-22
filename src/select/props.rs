use std::rc::Rc;

use gpui::{App, SharedString, Window};

use crate::select::{SelectOpenChangeDetails, SelectValueChangeDetails};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectSelectionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectAlign {
    #[default]
    Start,
    Center,
    End,
}

pub type SelectValueChangeHandler<T> =
    Rc<dyn Fn(Option<&T>, &mut SelectValueChangeDetails, &mut Window, &mut App) + 'static>;

pub type SelectValuesChangeHandler<T> =
    Rc<dyn Fn(&[T], &mut SelectValueChangeDetails, &mut Window, &mut App) + 'static>;

pub type SelectOpenChangeHandler =
    Rc<dyn Fn(bool, &mut SelectOpenChangeDetails, &mut Window, &mut App) + 'static>;

pub type SelectLabelResolver<T> = Rc<dyn Fn(&T) -> SharedString + 'static>;

pub type SelectValueSerializer<T> = Rc<dyn Fn(&T) -> SharedString + 'static>;

pub type SelectValueComparator<T> = Rc<dyn Fn(&T, &T) -> bool + 'static>;

pub type SelectMultipleValueFormatter<T> =
    Rc<dyn Fn(&[SharedString], &[T]) -> SharedString + 'static>;

pub struct SelectProps<T: Clone + Eq + 'static> {
    name: Option<SharedString>,
    form: Option<SharedString>,
    disabled: bool,
    read_only: bool,
    required: bool,
    modal: bool,
    selection_mode: SelectSelectionMode,
    highlight_item_on_hover: bool,
    label_resolver: Option<SelectLabelResolver<T>>,
    value_serializer: Option<SelectValueSerializer<T>>,
    value_comparator: Option<SelectValueComparator<T>>,
    multiple_value_formatter: Option<SelectMultipleValueFormatter<T>>,
    on_value_change: Option<SelectValueChangeHandler<T>>,
    on_values_change: Option<SelectValuesChangeHandler<T>>,
    on_open_change: Option<SelectOpenChangeHandler>,
}

impl<T: Clone + Eq + 'static> Clone for SelectProps<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            form: self.form.clone(),
            disabled: self.disabled,
            read_only: self.read_only,
            required: self.required,
            modal: self.modal,
            selection_mode: self.selection_mode,
            highlight_item_on_hover: self.highlight_item_on_hover,
            label_resolver: self.label_resolver.clone(),
            value_serializer: self.value_serializer.clone(),
            value_comparator: self.value_comparator.clone(),
            multiple_value_formatter: self.multiple_value_formatter.clone(),
            on_value_change: self.on_value_change.clone(),
            on_values_change: self.on_values_change.clone(),
            on_open_change: self.on_open_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> SelectProps<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<SharedString>,
        form: Option<SharedString>,
        disabled: bool,
        read_only: bool,
        required: bool,
        modal: bool,
        selection_mode: SelectSelectionMode,
        highlight_item_on_hover: bool,
        label_resolver: Option<SelectLabelResolver<T>>,
        value_serializer: Option<SelectValueSerializer<T>>,
        value_comparator: Option<SelectValueComparator<T>>,
        multiple_value_formatter: Option<SelectMultipleValueFormatter<T>>,
        on_value_change: Option<SelectValueChangeHandler<T>>,
        on_values_change: Option<SelectValuesChangeHandler<T>>,
        on_open_change: Option<SelectOpenChangeHandler>,
    ) -> Self {
        Self {
            name,
            form,
            disabled,
            read_only,
            required,
            modal,
            selection_mode,
            highlight_item_on_hover,
            label_resolver,
            value_serializer,
            value_comparator,
            multiple_value_formatter,
            on_value_change,
            on_values_change,
            on_open_change,
        }
    }

    pub fn name(&self) -> Option<&SharedString> {
        self.name.as_ref()
    }

    pub fn form(&self) -> Option<&SharedString> {
        self.form.as_ref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn selection_mode(&self) -> SelectSelectionMode {
        self.selection_mode
    }

    pub fn highlight_item_on_hover(&self) -> bool {
        self.highlight_item_on_hover
    }

    pub fn label_for_value(&self, value: &T) -> Option<SharedString> {
        self.label_resolver.as_ref().map(|resolver| resolver(value))
    }

    pub fn serialize_value(&self, value: &T) -> Option<SharedString> {
        self.value_serializer
            .as_ref()
            .map(|serializer| serializer(value))
    }

    pub fn value_comparator(&self) -> Option<SelectValueComparator<T>> {
        self.value_comparator.clone()
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

    pub fn on_value_change(&self) -> Option<&SelectValueChangeHandler<T>> {
        self.on_value_change.as_ref()
    }

    pub fn on_values_change(&self) -> Option<&SelectValuesChangeHandler<T>> {
        self.on_values_change.as_ref()
    }

    pub fn on_open_change(&self) -> Option<&SelectOpenChangeHandler> {
        self.on_open_change.as_ref()
    }
}
