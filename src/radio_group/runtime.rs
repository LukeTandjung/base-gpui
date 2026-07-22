use gpui::FocusHandle;

use crate::radio_group::{
    RadioGroupIndicatorStyleState, RadioGroupProps, RadioGroupRadioStyleState,
    RadioGroupRootStyleState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RadioGroupValueChangeReason {
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RadioGroupValueChangeSource {
    Pointer,
    Keyboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioGroupValueChangeDetails {
    reason: RadioGroupValueChangeReason,
    source: RadioGroupValueChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl RadioGroupValueChangeDetails {
    pub fn new(
        reason: RadioGroupValueChangeReason,
        source: RadioGroupValueChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> RadioGroupValueChangeReason {
        self.reason
    }

    pub fn source(&self) -> RadioGroupValueChangeSource {
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

#[derive(Clone)]
pub struct RadioGroupRadioMetadata<T: Clone + Eq + 'static> {
    value: T,
    disabled: bool,
    read_only: bool,
    required: bool,
    index: usize,
}

impl<T: Clone + Eq + 'static> RadioGroupRadioMetadata<T> {
    pub fn new(value: T, disabled: bool, read_only: bool, required: bool, index: usize) -> Self {
        Self {
            value,
            disabled,
            read_only,
            required,
            index,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
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

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RadioGroupSelectionTarget<T: Clone + Eq + 'static> {
    value: T,
    disabled: bool,
    read_only: bool,
}

impl<T: Clone + Eq + 'static> RadioGroupSelectionTarget<T> {
    fn new(value: T, disabled: bool, read_only: bool) -> Self {
        Self {
            value,
            disabled,
            read_only,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn read_only(&self) -> bool {
        self.read_only
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    Next,
    Previous,
}

pub struct RadioGroupSelectOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    value: Option<T>,
}

impl<T: Clone + Eq + 'static> RadioGroupSelectOutcome<T> {
    fn new(changed: bool, value: Option<T>) -> Self {
        Self { changed, value }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<T> {
        self.value
    }
}

#[derive(Clone)]
pub struct RadioGroupRuntime<T: Clone + Eq + 'static> {
    selected: Option<T>,
    radios: Vec<RadioGroupRadioMetadata<T>>,
    highlighted_radio_index: Option<usize>,
    focused_radio_index: Option<usize>,
    radio_focus_handles: Vec<(usize, FocusHandle)>,
    has_seeded_initial_focus: bool,
}

impl<T: Clone + Eq + 'static> Default for RadioGroupRuntime<T> {
    fn default() -> Self {
        Self {
            selected: None,
            radios: Vec::new(),
            highlighted_radio_index: None,
            focused_radio_index: None,
            radio_focus_handles: Vec::new(),
            has_seeded_initial_focus: false,
        }
    }
}

impl<T: Clone + Eq + 'static> RadioGroupRuntime<T> {
    pub fn new(selected: Option<T>) -> Self {
        Self {
            selected,
            ..Self::default()
        }
    }

    /// Returns the currently observed selected value.
    pub fn selected_value(&self) -> Option<T> {
        self.selected.clone()
    }

    /// Reconciles the runtime with the externally observed selected value.
    pub fn sync_selected_from_context(&mut self, selected: Option<T>) {
        self.selected = selected;
    }

    /// Replaces the walked radio metadata and preserves the highlighted value when possible.
    pub fn sync_children(
        &mut self,
        mut radios: Vec<RadioGroupRadioMetadata<T>>,
        mut radio_focus_handles: Vec<(usize, FocusHandle)>,
    ) {
        let highlighted_value = self
            .highlighted_radio_index
            .and_then(|index| self.enabled_value_at_index(index).cloned());

        radios.sort_by_key(RadioGroupRadioMetadata::index);
        radio_focus_handles.sort_by_key(|(index, _)| *index);

        self.radios = radios;
        self.radio_focus_handles = radio_focus_handles;
        self.highlighted_radio_index = highlighted_value
            .as_ref()
            .and_then(|value| self.index_of_enabled_value(value))
            .or_else(|| self.highlight_for_selected());
    }

    /// Reconciles selected state without falling back when the selected radio is missing or disabled.
    pub fn reconcile(&mut self, observed_selected: Option<T>) {
        let selected_changed = self.selected != observed_selected;
        self.selected = observed_selected;

        if selected_changed || self.highlighted_radio_index.is_none() {
            self.sync_highlight_with_selected();
        }
    }

    /// Reconciles which registered radio currently owns focus.
    pub fn sync_focused_index(&mut self, focused_radio_index: Option<usize>) -> bool {
        if self.focused_radio_index == focused_radio_index {
            return false;
        }

        self.focused_radio_index = focused_radio_index;
        true
    }

    /// Returns the initial roving focus handle once, after children have been synced.
    pub fn take_initial_focus_handle(&mut self) -> Option<FocusHandle> {
        if self.has_seeded_initial_focus {
            return None;
        }

        self.has_seeded_initial_focus = true;
        self.highlighted_focus_handle()
    }

    /// Moves the roving highlight to the next or previous enabled radio, wrapping at the ends.
    pub fn move_highlight(&mut self, direction: Move) {
        match direction {
            Move::Next => {
                if let Some(index) = self.next_enabled_index(self.highlighted_radio_index) {
                    self.highlighted_radio_index = Some(index);
                }
            }
            Move::Previous => {
                if let Some(index) = self.previous_enabled_index(self.highlighted_radio_index) {
                    self.highlighted_radio_index = Some(index);
                }
            }
        }
    }

    /// Returns the focus handle for the highlighted radio, when one is registered.
    pub fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        self.highlighted_radio_index
            .and_then(|index| self.focus_handle_at_index(index))
    }

    /// Returns the current highlighted radio as a user-selection target.
    pub fn highlighted_selection_target(&self) -> Option<RadioGroupSelectionTarget<T>> {
        let index = self.highlighted_radio_index?;
        let radio = self.radio_at_index(index)?;

        Some(RadioGroupSelectionTarget::new(
            radio.value().clone(),
            radio.disabled(),
            radio.read_only(),
        ))
    }

    /// Computes whether a user selection can change the selected value without committing it.
    pub fn request_select(
        &self,
        current: Option<T>,
        value: T,
        disabled: bool,
        read_only: bool,
    ) -> RadioGroupSelectOutcome<T> {
        if disabled || read_only || current.as_ref() == Some(&value) {
            return RadioGroupSelectOutcome::new(false, Some(value));
        }

        RadioGroupSelectOutcome::new(true, Some(value))
    }

    /// Commits an accepted user selection to uncontrolled runtime state.
    pub fn commit_selected(&mut self, selected: Option<T>) -> bool {
        let changed = self.selected != selected;
        self.selected = selected;

        if changed {
            self.sync_highlight_with_selected();
        }

        changed
    }

    /// Returns the number of registered radios, for `aria_size_of_set`.
    pub fn radio_count(&self) -> usize {
        self.radios.len()
    }

    /// Returns the style state for `RadioGroupRoot`.
    pub fn root_state(&self, props: &RadioGroupProps<T>) -> RadioGroupRootStyleState {
        RadioGroupRootStyleState::new(
            props.disabled(),
            props.read_only(),
            props.required(),
            self.focused_radio_index.is_some(),
            self.selected.is_some(),
        )
    }

    /// Returns the style state for `RadioGroupRadio`.
    pub fn radio_state(
        &self,
        value: Option<&T>,
        disabled: bool,
        read_only: bool,
        required: bool,
        index: Option<usize>,
        props: &RadioGroupProps<T>,
    ) -> RadioGroupRadioStyleState {
        let checked = match (value, self.selected.as_ref()) {
            (Some(value), Some(selected)) => value == selected,
            _ => false,
        };
        let disabled = props.disabled() || disabled;
        let read_only = props.read_only() || read_only;
        let required = props.required() || required;
        let focused = self.focused_radio_index == index;
        let highlighted = self.highlighted_radio_index == index;
        let tab_stop = highlighted && !disabled;

        RadioGroupRadioStyleState::new(
            checked,
            disabled,
            read_only,
            required,
            focused,
            highlighted,
            tab_stop,
        )
    }

    /// Returns the style state for `RadioGroupIndicator`.
    pub fn indicator_state(
        &self,
        radio: RadioGroupRadioStyleState,
        keep_mounted: bool,
    ) -> RadioGroupIndicatorStyleState {
        RadioGroupIndicatorStyleState::new(radio, keep_mounted || radio.checked)
    }

    fn sync_highlight_with_selected(&mut self) {
        self.highlighted_radio_index = self.highlight_for_selected();
    }

    fn highlight_for_selected(&self) -> Option<usize> {
        self.selected
            .as_ref()
            .and_then(|value| self.index_of_enabled_value(value))
            .or_else(|| self.first_enabled_index())
    }

    fn first_enabled_index(&self) -> Option<usize> {
        self.radios
            .iter()
            .find(|radio| !radio.disabled())
            .map(RadioGroupRadioMetadata::index)
    }

    fn next_enabled_index(&self, current: Option<usize>) -> Option<usize> {
        let current = current.or_else(|| self.first_enabled_index())?;

        self.radios
            .iter()
            .find(|radio| !radio.disabled() && radio.index() > current)
            .map(RadioGroupRadioMetadata::index)
            .or_else(|| self.first_enabled_index())
    }

    fn previous_enabled_index(&self, current: Option<usize>) -> Option<usize> {
        let current = current.or_else(|| self.first_enabled_index())?;

        self.radios
            .iter()
            .rev()
            .find(|radio| !radio.disabled() && radio.index() < current)
            .map(RadioGroupRadioMetadata::index)
            .or_else(|| self.last_enabled_index())
    }

    fn last_enabled_index(&self) -> Option<usize> {
        self.radios
            .iter()
            .rev()
            .find(|radio| !radio.disabled())
            .map(RadioGroupRadioMetadata::index)
    }

    fn enabled_value_at_index(&self, index: usize) -> Option<&T> {
        self.radios
            .iter()
            .find(|radio| !radio.disabled() && radio.index() == index)
            .map(RadioGroupRadioMetadata::value)
    }

    fn index_of_enabled_value(&self, value: &T) -> Option<usize> {
        self.radios
            .iter()
            .find(|radio| !radio.disabled() && radio.value() == value)
            .map(RadioGroupRadioMetadata::index)
    }

    fn focus_handle_at_index(&self, index: usize) -> Option<FocusHandle> {
        self.radio_focus_handles
            .iter()
            .find(|(radio_index, _)| *radio_index == index)
            .map(|(_, focus_handle)| focus_handle.clone())
    }

    fn radio_at_index(&self, index: usize) -> Option<&RadioGroupRadioMetadata<T>> {
        self.radios.iter().find(|radio| radio.index() == index)
    }
}

#[cfg(test)]
mod tests {
    use super::{Move, RadioGroupRadioMetadata, RadioGroupRuntime};
    use crate::radio_group::RadioGroupProps;

    #[test]
    fn move_highlight_skips_disabled_radios() {
        let mut runtime = RadioGroupRuntime::new(Some(1));
        let props = RadioGroupProps::new(None, None, false, false, false, None);

        runtime.sync_children(
            vec![
                RadioGroupRadioMetadata::new(1, false, false, false, 0),
                RadioGroupRadioMetadata::new(2, true, false, false, 1),
                RadioGroupRadioMetadata::new(3, false, false, false, 2),
            ],
            Vec::new(),
        );
        runtime.reconcile(runtime.selected_value());

        runtime.move_highlight(Move::Next);

        assert!(
            !runtime
                .radio_state(Some(&1), false, false, false, Some(0), &props)
                .highlighted
        );
        assert!(
            runtime
                .radio_state(Some(&3), false, false, false, Some(2), &props)
                .highlighted
        );
    }

    #[test]
    fn reconcile_preserves_missing_selected_value() {
        let mut runtime = RadioGroupRuntime::new(Some(2));
        let props = RadioGroupProps::new(None, None, false, false, false, None);

        runtime.sync_children(
            vec![RadioGroupRadioMetadata::new(1, false, false, false, 0)],
            Vec::new(),
        );
        runtime.reconcile(runtime.selected_value());

        assert!(
            !runtime
                .radio_state(Some(&1), false, false, false, Some(0), &props)
                .checked
        );
        assert_eq!(runtime.selected_value(), Some(2));
    }
}
