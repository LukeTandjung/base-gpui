use gpui::FocusHandle;

use crate::toggle_group::{ToggleGroupProps, ToggleGroupStyleState};

/// The group and the activating toggle share one details object per
/// activation, so the group details are the Toggle details type.
pub type ToggleGroupValueChangeDetails = crate::toggle::TogglePressedChangeDetails;
pub type ToggleGroupValueChangeReason = crate::toggle::TogglePressedChangeReason;
pub type ToggleGroupValueChangeSource = crate::toggle::TogglePressedChangeSource;

#[derive(Clone)]
pub struct ToggleGroupToggleMetadata<T: Clone + Eq + 'static> {
    value: Option<T>,
    disabled: bool,
    index: usize,
}

impl<T: Clone + Eq + 'static> ToggleGroupToggleMetadata<T> {
    pub fn new(value: Option<T>, disabled: bool, index: usize) -> Self {
        Self {
            value,
            disabled,
            index,
        }
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToggleGroupMove {
    Next,
    Previous,
    First,
    Last,
}

pub struct ToggleGroupCommitOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    value: Vec<T>,
}

impl<T: Clone + Eq + 'static> ToggleGroupCommitOutcome<T> {
    fn new(changed: bool, value: Vec<T>) -> Self {
        Self { changed, value }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn value(&self) -> &[T] {
        &self.value
    }

    pub fn into_value(self) -> Vec<T> {
        self.value
    }
}

#[derive(Clone)]
pub struct ToggleGroupRuntime<T: Clone + Eq + 'static> {
    value: Vec<T>,
    toggles: Vec<ToggleGroupToggleMetadata<T>>,
    highlighted_index: Option<usize>,
    focused_index: Option<usize>,
    focus_handles: Vec<(usize, FocusHandle)>,
}

impl<T: Clone + Eq + 'static> Default for ToggleGroupRuntime<T> {
    fn default() -> Self {
        Self {
            value: Vec::new(),
            toggles: Vec::new(),
            highlighted_index: None,
            focused_index: None,
            focus_handles: Vec::new(),
        }
    }
}

impl<T: Clone + Eq + 'static> ToggleGroupRuntime<T> {
    pub fn new(value: Vec<T>) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }

    /// Returns the currently observed group value.
    pub fn value(&self) -> &[T] {
        &self.value
    }

    /// Returns an owned copy of the currently observed group value.
    pub fn value_vec(&self) -> Vec<T> {
        self.value.clone()
    }

    /// Reconciles the runtime with the externally observed group value.
    pub fn sync_value_from_context(&mut self, value: Vec<T>) {
        self.value = value;
    }

    /// Replaces the walked toggle metadata and focus handles.
    pub fn sync_children(
        &mut self,
        mut toggles: Vec<ToggleGroupToggleMetadata<T>>,
        mut focus_handles: Vec<(usize, FocusHandle)>,
    ) {
        toggles.sort_by_key(ToggleGroupToggleMetadata::index);
        focus_handles.sort_by_key(|(index, _)| *index);

        self.toggles = toggles;
        self.focus_handles = focus_handles;
    }

    /// Reconciles the observed group value and seeds the roving tab stop on
    /// the first enabled toggle when no valid tab stop exists.
    pub fn reconcile(&mut self, observed_value: Vec<T>) {
        self.value = observed_value;

        let highlight_valid = self
            .highlighted_index
            .map(|index| self.is_enabled_index(index))
            .unwrap_or(false);
        if !highlight_valid {
            self.highlighted_index = self.first_enabled_index();
        }
    }

    /// Reconciles which registered toggle currently owns focus.
    pub fn sync_focused_index(&mut self, focused_index: Option<usize>) -> bool {
        if self.focused_index == focused_index {
            return false;
        }

        self.focused_index = focused_index;
        true
    }

    /// Returns the registered toggle metadata, in source order. Kept queryable
    /// so a future Toolbar port can consume per-item disabled facts.
    pub fn toggles(&self) -> &[ToggleGroupToggleMetadata<T>] {
        &self.toggles
    }

    /// Answers whether the group value contains the toggle's membership value.
    pub fn toggle_pressed(&self, value: Option<&T>) -> bool {
        match value {
            Some(value) => self.value.contains(value),
            None => false,
        }
    }

    /// Returns the roving tab-stop index.
    pub fn highlighted_index(&self) -> Option<usize> {
        self.highlighted_index
    }

    /// Moves the roving highlight across enabled toggles, wrapping only when
    /// `loop_focus` is set and clamping at the ends otherwise.
    pub fn move_highlight(&mut self, direction: ToggleGroupMove, loop_focus: bool) {
        let target = match direction {
            ToggleGroupMove::First => self.first_enabled_index(),
            ToggleGroupMove::Last => self.last_enabled_index(),
            ToggleGroupMove::Next => self.next_enabled_index(loop_focus),
            ToggleGroupMove::Previous => self.previous_enabled_index(loop_focus),
        };

        if let Some(index) = target {
            self.highlighted_index = Some(index);
        }
    }

    /// Returns the focus handle for the highlighted toggle, when registered.
    pub fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        let index = self.highlighted_index?;

        self.focus_handles
            .iter()
            .find(|(handle_index, _)| *handle_index == index)
            .map(|(_, focus_handle)| focus_handle.clone())
    }

    /// Computes the next group value for a toggle activation without
    /// committing it. Single mode replaces the value with `[value]` on press
    /// and empties it on unpress; multiple mode appends on press and removes
    /// the first occurrence on unpress.
    pub fn request_commit(
        &self,
        current: &[T],
        value: &T,
        next_pressed: bool,
        multiple: bool,
        disabled: bool,
    ) -> ToggleGroupCommitOutcome<T> {
        if disabled {
            return ToggleGroupCommitOutcome::new(false, current.to_vec());
        }

        let next = match (multiple, next_pressed) {
            (true, true) => {
                let mut next = current.to_vec();
                next.push(value.clone());
                next
            }
            (true, false) => {
                let mut next = current.to_vec();
                if let Some(position) = next.iter().position(|entry| entry == value) {
                    next.remove(position);
                }
                next
            }
            (false, true) => vec![value.clone()],
            (false, false) => Vec::new(),
        };
        let changed = next.as_slice() != current;

        ToggleGroupCommitOutcome::new(changed, next)
    }

    /// Commits an accepted activation to uncontrolled runtime state.
    pub fn commit_value(&mut self, value: Vec<T>) -> bool {
        let changed = self.value != value;
        self.value = value;
        changed
    }

    /// Returns the style state for the `ToggleGroup` layer.
    pub fn group_state(&self, props: &ToggleGroupProps<T>) -> ToggleGroupStyleState {
        ToggleGroupStyleState::new(props.disabled(), props.orientation(), props.multiple())
    }

    fn is_enabled_index(&self, index: usize) -> bool {
        self.toggles
            .iter()
            .any(|toggle| !toggle.disabled() && toggle.index() == index)
    }

    fn first_enabled_index(&self) -> Option<usize> {
        self.toggles
            .iter()
            .find(|toggle| !toggle.disabled())
            .map(ToggleGroupToggleMetadata::index)
    }

    fn last_enabled_index(&self) -> Option<usize> {
        self.toggles
            .iter()
            .rev()
            .find(|toggle| !toggle.disabled())
            .map(ToggleGroupToggleMetadata::index)
    }

    fn next_enabled_index(&self, loop_focus: bool) -> Option<usize> {
        let current = self
            .highlighted_index
            .or_else(|| self.first_enabled_index())?;

        self.toggles
            .iter()
            .find(|toggle| !toggle.disabled() && toggle.index() > current)
            .map(ToggleGroupToggleMetadata::index)
            .or_else(|| match loop_focus {
                true => self.first_enabled_index(),
                false => None,
            })
    }

    fn previous_enabled_index(&self, loop_focus: bool) -> Option<usize> {
        let current = self
            .highlighted_index
            .or_else(|| self.first_enabled_index())?;

        self.toggles
            .iter()
            .rev()
            .find(|toggle| !toggle.disabled() && toggle.index() < current)
            .map(ToggleGroupToggleMetadata::index)
            .or_else(|| match loop_focus {
                true => self.last_enabled_index(),
                false => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{ToggleGroupMove, ToggleGroupRuntime, ToggleGroupToggleMetadata};

    fn runtime_with_three_toggles(second_disabled: bool) -> ToggleGroupRuntime<i32> {
        let mut runtime = ToggleGroupRuntime::new(Vec::new());

        runtime.sync_children(
            vec![
                ToggleGroupToggleMetadata::new(Some(1), false, 0),
                ToggleGroupToggleMetadata::new(Some(2), second_disabled, 1),
                ToggleGroupToggleMetadata::new(Some(3), false, 2),
            ],
            Vec::new(),
        );
        runtime.reconcile(runtime.value_vec());
        runtime
    }

    #[test]
    fn single_mode_replaces_and_empties_the_group_value() {
        let runtime = runtime_with_three_toggles(false);

        let pressed = runtime.request_commit(&[1], &2, true, false, false);
        assert_eq!(pressed.value(), &[2]);

        let unpressed = runtime.request_commit(&[2], &2, false, false, false);
        assert!(unpressed.value().is_empty());
    }

    #[test]
    fn multiple_mode_appends_and_removes_first_occurrence() {
        let runtime = runtime_with_three_toggles(false);

        let pressed = runtime.request_commit(&[1], &2, true, true, false);
        assert_eq!(pressed.value(), &[1, 2]);

        let unpressed = runtime.request_commit(&[2, 1, 2], &2, false, true, false);
        assert_eq!(unpressed.value(), &[1, 2]);
    }

    #[test]
    fn move_highlight_skips_disabled_and_clamps_without_loop() {
        let mut runtime = runtime_with_three_toggles(true);
        assert_eq!(runtime.highlighted_index(), Some(0));

        runtime.move_highlight(ToggleGroupMove::Next, false);
        assert_eq!(runtime.highlighted_index(), Some(2));

        runtime.move_highlight(ToggleGroupMove::Next, false);
        assert_eq!(runtime.highlighted_index(), Some(2));

        runtime.move_highlight(ToggleGroupMove::Next, true);
        assert_eq!(runtime.highlighted_index(), Some(0));
    }
}
