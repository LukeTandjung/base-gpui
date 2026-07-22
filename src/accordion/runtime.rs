use gpui::FocusHandle;

use crate::accordion::{
    AccordionHeaderStyleState, AccordionItemStyleState, AccordionPanelStyleState, AccordionProps,
    AccordionRootStyleState, AccordionTriggerStyleState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccordionChangeReason {
    TriggerPress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccordionChangeSource {
    Pointer,
    Keyboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccordionChangeDetails {
    reason: AccordionChangeReason,
    source: AccordionChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl AccordionChangeDetails {
    pub fn new(
        reason: AccordionChangeReason,
        source: AccordionChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> AccordionChangeReason {
        self.reason
    }

    pub fn source(&self) -> AccordionChangeSource {
        self.source
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

pub type AccordionValueChangeDetails = AccordionChangeDetails;
pub type AccordionItemOpenChangeDetails = AccordionChangeDetails;

#[derive(Clone)]
pub struct AccordionItemMetadata<T: Clone + Eq + 'static> {
    value: T,
    disabled: bool,
    index: usize,
    trigger_focus_handle: Option<FocusHandle>,
    trigger_focused: bool,
}

impl<T: Clone + Eq + 'static> AccordionItemMetadata<T> {
    pub fn new(
        value: T,
        disabled: bool,
        index: usize,
        trigger_focus_handle: Option<FocusHandle>,
        trigger_focused: bool,
    ) -> Self {
        Self {
            value,
            disabled,
            index,
            trigger_focus_handle,
            trigger_focused,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn trigger_focus_handle(&self) -> Option<&FocusHandle> {
        self.trigger_focus_handle.as_ref()
    }

    pub fn trigger_focused(&self) -> bool {
        self.trigger_focused
    }
}

pub struct AccordionToggleOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    next_open: bool,
    values: Vec<T>,
}

impl<T: Clone + Eq + 'static> AccordionToggleOutcome<T> {
    fn new(changed: bool, next_open: bool, values: Vec<T>) -> Self {
        Self {
            changed,
            next_open,
            values,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn next_open(&self) -> bool {
        self.next_open
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn into_values(self) -> Vec<T> {
        self.values
    }
}

#[derive(Clone)]
pub struct AccordionRuntime<T: Clone + Eq + 'static> {
    values: Vec<T>,
    items: Vec<AccordionItemMetadata<T>>,
}

impl<T: Clone + Eq + 'static> AccordionRuntime<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values: unique_values(values),
            items: Vec::new(),
        }
    }

    /// Returns the currently observed open values.
    pub fn open_values(&self) -> Vec<T> {
        self.values.clone()
    }

    /// Reconciles the runtime with the externally observed open values.
    pub fn sync_values_from_context(&mut self, values: Vec<T>) {
        self.values = unique_values(values);
    }

    /// Returns the number of items registered during the last render pass.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Reconciles the item metadata observed during the current render pass.
    pub fn sync_children(&mut self, mut items: Vec<AccordionItemMetadata<T>>) {
        items.sort_by_key(AccordionItemMetadata::index);
        self.items = items;
    }

    /// Computes the next root value requested by a user item toggle without committing it.
    pub fn request_toggle(
        &self,
        value: &T,
        root_disabled: bool,
        item_disabled: bool,
        multiple: bool,
    ) -> AccordionToggleOutcome<T> {
        if root_disabled || item_disabled {
            return AccordionToggleOutcome::new(false, self.item_open(value), self.values.clone());
        }

        let currently_open = self.item_open(value);
        let next_open = !currently_open;
        let values = next_values_for_toggle(&self.values, value, next_open, multiple);
        let changed = values != self.values;

        AccordionToggleOutcome::new(changed, next_open, values)
    }

    /// Commits accepted open values to uncontrolled runtime state.
    pub fn commit_values(&mut self, values: Vec<T>) -> bool {
        let values = unique_values(values);
        let changed = self.values != values;
        self.values = values;
        changed
    }

    /// Answers whether a value is currently open.
    pub fn item_open(&self, value: &T) -> bool {
        self.values.iter().any(|open_value| open_value == value)
    }

    /// Returns the style state for `AccordionRoot`.
    pub fn root_state(&self, props: &AccordionProps<T>) -> AccordionRootStyleState<T> {
        AccordionRootStyleState::new(
            self.values.clone(),
            props.disabled(),
            props.multiple(),
            props.orientation(),
        )
    }

    /// Returns the style state shared by item-shaped parts.
    pub fn item_state(
        &self,
        value: &T,
        index: usize,
        item_disabled: bool,
        props: &AccordionProps<T>,
    ) -> AccordionItemStyleState<T> {
        let disabled = props.disabled() || item_disabled;
        AccordionItemStyleState::new(
            value.clone(),
            self.values.clone(),
            self.item_open(value),
            disabled,
            index,
            props.orientation(),
        )
    }

    /// Returns the style state for `AccordionHeader`.
    pub fn header_state(
        &self,
        value: &T,
        index: usize,
        item_disabled: bool,
        props: &AccordionProps<T>,
    ) -> AccordionHeaderStyleState<T> {
        AccordionHeaderStyleState::new(self.item_state(value, index, item_disabled, props))
    }

    /// Returns the style state for `AccordionTrigger`.
    pub fn trigger_state(
        &self,
        value: &T,
        index: usize,
        item_disabled: bool,
        props: &AccordionProps<T>,
    ) -> AccordionTriggerStyleState<T> {
        AccordionTriggerStyleState::new(
            self.item_state(value, index, item_disabled, props),
            self.trigger_focused(index),
        )
    }

    /// Returns the style state for `AccordionPanel`.
    pub fn panel_state(
        &self,
        value: &T,
        index: usize,
        item_disabled: bool,
        keep_mounted: bool,
        props: &AccordionProps<T>,
    ) -> AccordionPanelStyleState<T> {
        AccordionPanelStyleState::new(
            self.item_state(value, index, item_disabled, props),
            keep_mounted,
        )
    }

    fn trigger_focused(&self, index: usize) -> bool {
        self.items
            .iter()
            .any(|item| item.index() == index && item.trigger_focused())
    }
}

fn next_values_for_toggle<T: Clone + Eq + 'static>(
    current: &[T],
    value: &T,
    next_open: bool,
    multiple: bool,
) -> Vec<T> {
    if multiple {
        if next_open {
            let mut next = unique_values(current.to_vec());
            if !next.iter().any(|open_value| open_value == value) {
                next.push(value.clone());
            }
            next
        } else {
            current
                .iter()
                .filter(|open_value| *open_value != value)
                .cloned()
                .collect()
        }
    } else if next_open {
        Vec::from([value.clone()])
    } else {
        Vec::new()
    }
}

fn unique_values<T: Clone + Eq + 'static>(values: Vec<T>) -> Vec<T> {
    values.into_iter().fold(Vec::new(), |mut unique, value| {
        if !unique.iter().any(|existing| existing == &value) {
            unique.push(value);
        }
        unique
    })
}

#[cfg(test)]
mod tests {
    use super::{AccordionRuntime, AccordionToggleOutcome};

    fn opened(outcome: AccordionToggleOutcome<&'static str>) -> Vec<&'static str> {
        outcome.values().to_vec()
    }

    #[test]
    fn single_mode_opening_replaces_current_value() {
        let runtime = AccordionRuntime::new(Vec::from(["first"]));

        let outcome = runtime.request_toggle(&"second", false, false, false);

        assert_eq!(opened(outcome), Vec::from(["second"]));
    }

    #[test]
    fn multiple_mode_opening_appends_value() {
        let runtime = AccordionRuntime::new(Vec::from(["first"]));

        let outcome = runtime.request_toggle(&"second", false, false, true);

        assert_eq!(opened(outcome), Vec::from(["first", "second"]));
    }

    #[test]
    fn disabled_toggle_request_is_noop() {
        let runtime = AccordionRuntime::new(Vec::from(["first"]));

        let outcome = runtime.request_toggle(&"second", true, false, false);

        assert!(!outcome.changed());
        assert_eq!(opened(outcome), Vec::from(["first"]));
    }
}
