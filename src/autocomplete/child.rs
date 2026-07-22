use gpui::{AnyElement, IntoElement};

use crate::{
    autocomplete::{
        AutocompleteClear, AutocompleteInput, AutocompleteInputGroup, AutocompletePortal,
        AutocompleteTrigger, AutocompleteValue,
    },
    combobox::ComboboxChild,
};

/// Root children of an `AutocompleteRoot`: the reused Combobox parts (routed
/// through the existing `ComboboxChild` wiring) plus the Autocomplete-only
/// `AutocompleteValue` part.
pub enum AutocompleteChild<T: Clone + Eq + 'static> {
    Combobox(ComboboxChild<T>),
    Value(Box<AutocompleteValue<T>>),
}

impl<T: Clone + Eq + 'static> From<AutocompleteInput<T>> for AutocompleteChild<T> {
    fn from(value: AutocompleteInput<T>) -> Self {
        Self::Combobox(ComboboxChild::Input(Box::new(value)))
    }
}

impl<T: Clone + Eq + 'static> From<AutocompleteInputGroup<T>> for AutocompleteChild<T> {
    fn from(value: AutocompleteInputGroup<T>) -> Self {
        Self::Combobox(ComboboxChild::InputGroup(Box::new(value)))
    }
}

impl<T: Clone + Eq + 'static> From<AutocompleteTrigger<T>> for AutocompleteChild<T> {
    fn from(value: AutocompleteTrigger<T>) -> Self {
        Self::Combobox(ComboboxChild::Trigger(Box::new(value)))
    }
}

impl<T: Clone + Eq + 'static> From<AutocompleteClear<T>> for AutocompleteChild<T> {
    fn from(value: AutocompleteClear<T>) -> Self {
        Self::Combobox(ComboboxChild::Clear(Box::new(value)))
    }
}

impl<T: Clone + Eq + 'static> From<AutocompletePortal<T>> for AutocompleteChild<T> {
    fn from(value: AutocompletePortal<T>) -> Self {
        Self::Combobox(ComboboxChild::Portal(Box::new(value)))
    }
}

impl<T: Clone + Eq + 'static> From<AutocompleteValue<T>> for AutocompleteChild<T> {
    fn from(value: AutocompleteValue<T>) -> Self {
        Self::Value(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<AnyElement> for AutocompleteChild<T> {
    fn from(value: AnyElement) -> Self {
        Self::Combobox(ComboboxChild::Any(value))
    }
}

impl<T: Clone + Eq + 'static> IntoElement for AutocompleteChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Combobox(child) => child.into_any_element(),
            Self::Value(value) => (*value).into_any_element(),
        }
    }
}
