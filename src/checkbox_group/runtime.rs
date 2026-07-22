use gpui::{FocusHandle, SharedString};

use crate::{checkbox_group::CheckboxGroupProps, field::FieldValue};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CheckboxGroupValueChangeReason {
    #[default]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckboxGroupValueChangeDetails {
    reason: CheckboxGroupValueChangeReason,
    cancelable: bool,
    canceled: bool,
}

impl CheckboxGroupValueChangeDetails {
    pub fn new(reason: CheckboxGroupValueChangeReason, cancelable: bool) -> Self {
        Self {
            reason,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> CheckboxGroupValueChangeReason {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckboxGroupChildMetadata {
    key: SharedString,
    value: Option<SharedString>,
    disabled: bool,
    required: bool,
    parent: bool,
    checked: bool,
    focused: bool,
    focus_handle: Option<FocusHandle>,
}

impl CheckboxGroupChildMetadata {
    pub fn new(key: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            value: None,
            disabled: false,
            required: false,
            parent: false,
            checked: false,
            focused: false,
            focus_handle: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn maybe_value(mut self, value: Option<SharedString>) -> Self {
        self.value = value;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn parent(mut self, parent: bool) -> Self {
        self.parent = parent;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn focus_handle(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn key(&self) -> &SharedString {
        &self.key
    }

    pub fn value_ref(&self) -> Option<&SharedString> {
        self.value.as_ref()
    }

    pub fn disabled_value(&self) -> bool {
        self.disabled
    }

    pub fn required_value(&self) -> bool {
        self.required
    }

    pub fn parent_value(&self) -> bool {
        self.parent
    }

    pub fn checked_value(&self) -> bool {
        self.checked
    }

    pub fn focused_value(&self) -> bool {
        self.focused
    }

    pub fn focus_handle_ref(&self) -> Option<&FocusHandle> {
        self.focus_handle.as_ref()
    }
}

#[derive(Clone, Debug)]
struct RegisteredCheckbox {
    metadata: CheckboxGroupChildMetadata,
    generation: u64,
}

impl RegisteredCheckbox {
    fn new(metadata: CheckboxGroupChildMetadata, generation: u64) -> Self {
        Self {
            metadata,
            generation,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CheckboxGroupParentStatus {
    #[default]
    Mixed,
    On,
    Off,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckboxGroupValueChangeRequest {
    next_value: Vec<SharedString>,
    parent_status_after_commit: Option<CheckboxGroupParentStatus>,
    snapshot_after_commit: Option<Vec<SharedString>>,
}

impl CheckboxGroupValueChangeRequest {
    fn child(next_value: Vec<SharedString>) -> Self {
        Self {
            snapshot_after_commit: Some(next_value.clone()),
            next_value,
            parent_status_after_commit: Some(CheckboxGroupParentStatus::Mixed),
        }
    }

    fn parent(
        next_value: Vec<SharedString>,
        parent_status_after_commit: Option<CheckboxGroupParentStatus>,
    ) -> Self {
        Self {
            next_value,
            parent_status_after_commit,
            snapshot_after_commit: None,
        }
    }

    pub fn next_value(&self) -> &[SharedString] {
        &self.next_value
    }
}

#[derive(Clone, Debug, Default)]
pub struct CheckboxGroupRuntime {
    value: Vec<SharedString>,
    children: Vec<RegisteredCheckbox>,
    generation: u64,
    parent_snapshot: Vec<SharedString>,
    parent_status: CheckboxGroupParentStatus,
}

impl CheckboxGroupRuntime {
    pub fn new(value: Vec<SharedString>) -> Self {
        Self {
            parent_snapshot: value.clone(),
            value,
            ..Self::default()
        }
    }

    /// Reconciles the runtime with the externally observed selected values.
    pub fn sync_value_from_context(&mut self, value: Vec<SharedString>) {
        self.value = normalize_values(value);
    }

    /// Starts one descendant checkbox registration pass.
    pub fn begin_registration_pass(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    /// Registers or refreshes one grouped checkbox for the current pass.
    pub fn register_checkbox(&mut self, metadata: CheckboxGroupChildMetadata) -> bool {
        if let Some(child) = self
            .children
            .iter_mut()
            .find(|child| child.metadata.key == metadata.key)
        {
            let changed = child.metadata != metadata;
            child.metadata = metadata;
            child.generation = self.generation;
            return changed;
        }

        self.children
            .push(RegisteredCheckbox::new(metadata, self.generation));
        true
    }

    /// Finishes descendant registration and prunes removed checkboxes.
    pub fn finish_registration_pass(&mut self) -> bool {
        let previous_len = self.children.len();
        self.children
            .retain(|child| child.generation == self.generation);

        previous_len != self.children.len()
    }

    /// Returns the current selected values.
    pub fn value(&self) -> &[SharedString] {
        &self.value
    }

    /// Returns whether the given child value is currently checked.
    pub fn checked_for_value(&self, value: &SharedString) -> bool {
        self.value.contains(value)
    }

    /// Returns whether a parent checkbox should render checked.
    pub fn parent_checked(&self, props: &CheckboxGroupProps) -> bool {
        let all_values = props.all_values();
        all_values.iter().all(|value| self.value.contains(value))
            && self.selected_all_values_count(props) == all_values.len()
    }

    /// Returns whether a parent checkbox should render indeterminate.
    pub fn parent_indeterminate(&self, props: &CheckboxGroupProps) -> bool {
        let selected_count = self.selected_all_values_count(props);
        selected_count != props.all_values().len() && selected_count > 0
    }

    /// Computes a requested child checkbox value change without committing it.
    pub fn request_child_value_change(
        &self,
        value: Option<&SharedString>,
        checked: bool,
        props: &CheckboxGroupProps,
    ) -> Option<CheckboxGroupValueChangeRequest> {
        if props.disabled() {
            return None;
        }

        let value = value?;
        let mut next_value = self.value.clone();

        if checked {
            if !next_value.contains(value) {
                next_value.push(value.clone());
            }
        } else {
            next_value.retain(|item| item != value);
        }

        next_value = normalize_values(next_value);
        if next_value == self.value {
            return None;
        }

        Some(CheckboxGroupValueChangeRequest::child(next_value))
    }

    /// Computes a requested parent checkbox value change without committing it.
    pub fn request_parent_value_change(
        &self,
        props: &CheckboxGroupProps,
    ) -> Option<CheckboxGroupValueChangeRequest> {
        if props.disabled() {
            return None;
        }

        let none = self.parent_none_values(props);
        let all = self.parent_all_values(props);
        let snapshot = self.parent_snapshot_for_all_values(props);
        let all_on_or_off = snapshot.len() == all.len() || snapshot.is_empty();

        let (next_value, next_status) = if all_on_or_off {
            if self.value_equivalent_for_all_values(&all, props) {
                (none, None)
            } else {
                (all, None)
            }
        } else {
            match self.parent_status {
                CheckboxGroupParentStatus::Mixed => (all, Some(CheckboxGroupParentStatus::On)),
                CheckboxGroupParentStatus::On => (none, Some(CheckboxGroupParentStatus::Off)),
                CheckboxGroupParentStatus::Off => {
                    (snapshot, Some(CheckboxGroupParentStatus::Mixed))
                }
            }
        };

        let next_value = normalize_values(next_value);
        if next_value == self.value && next_status.is_none() {
            return None;
        }

        Some(CheckboxGroupValueChangeRequest::parent(
            next_value,
            next_status,
        ))
    }

    /// Commits accepted selected values to uncontrolled runtime state.
    pub fn commit_value(&mut self, value: Vec<SharedString>) -> bool {
        let value = normalize_values(value);
        if self.value == value {
            return false;
        }

        self.value = value;
        true
    }

    /// Records accepted parent-cycle metadata after a child or parent change.
    pub fn accept_request(&mut self, request: &CheckboxGroupValueChangeRequest) -> bool {
        let mut changed = false;

        if let Some(snapshot) = request.snapshot_after_commit.as_ref() {
            let snapshot = normalize_values(snapshot.clone());
            if self.parent_snapshot != snapshot {
                self.parent_snapshot = snapshot;
                changed = true;
            }
        }

        if let Some(status) = request.parent_status_after_commit {
            if self.parent_status != status {
                self.parent_status = status;
                changed = true;
            }
        }

        changed
    }

    /// Returns the representative Field value for the group.
    pub fn field_value(&self) -> FieldValue {
        FieldValue::List(self.value.clone())
    }

    /// Returns whether any enabled non-parent child requires a value.
    pub fn required(&self) -> bool {
        self.children.iter().any(|child| {
            !child.metadata.parent && !child.metadata.disabled && child.metadata.required
        })
    }

    /// Returns whether any enabled required non-parent child is unchecked.
    pub fn value_missing(&self) -> bool {
        self.children.iter().any(|child| {
            !child.metadata.parent
                && !child.metadata.disabled
                && child.metadata.required
                && child
                    .metadata
                    .value
                    .as_ref()
                    .map(|value| !self.value.contains(value))
                    .unwrap_or(true)
        })
    }

    /// Returns whether any selected value is present.
    pub fn filled(&self) -> bool {
        !self.value.is_empty()
    }

    /// Returns whether any grouped checkbox is focused.
    pub fn focused(&self) -> bool {
        self.children
            .iter()
            .any(|child| child.metadata.focused && !child.metadata.disabled)
    }

    /// Returns a focus handle for the first enabled non-parent grouped checkbox.
    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.children
            .iter()
            .find(|child| {
                !child.metadata.parent
                    && !child.metadata.disabled
                    && child.metadata.focus_handle.is_some()
            })
            .and_then(|child| child.metadata.focus_handle.clone())
            .or_else(|| {
                self.children
                    .iter()
                    .find(|child| !child.metadata.disabled && child.metadata.focus_handle.is_some())
                    .and_then(|child| child.metadata.focus_handle.clone())
            })
    }

    fn selected_all_values_count(&self, props: &CheckboxGroupProps) -> usize {
        props
            .all_values()
            .iter()
            .filter(|value| self.value.contains(value))
            .count()
    }

    fn parent_all_values(&self, props: &CheckboxGroupProps) -> Vec<SharedString> {
        props
            .all_values()
            .iter()
            .filter(|value| !self.disabled_for_value(value) || self.value.contains(*value))
            .cloned()
            .collect()
    }

    fn parent_none_values(&self, props: &CheckboxGroupProps) -> Vec<SharedString> {
        props
            .all_values()
            .iter()
            .filter(|value| self.disabled_for_value(value) && self.value.contains(*value))
            .cloned()
            .collect()
    }

    fn parent_snapshot_for_all_values(&self, props: &CheckboxGroupProps) -> Vec<SharedString> {
        props
            .all_values()
            .iter()
            .filter(|value| self.parent_snapshot.contains(value))
            .cloned()
            .collect()
    }

    fn value_equivalent_for_all_values(
        &self,
        all: &[SharedString],
        props: &CheckboxGroupProps,
    ) -> bool {
        let selected_count = props
            .all_values()
            .iter()
            .filter(|value| self.value.contains(value))
            .count();

        selected_count == all.len()
    }

    fn disabled_for_value(&self, value: &SharedString) -> bool {
        self.children.iter().any(|child| {
            !child.metadata.parent
                && child.metadata.value.as_ref() == Some(value)
                && child.metadata.disabled
        })
    }
}

fn normalize_values(values: Vec<SharedString>) -> Vec<SharedString> {
    let mut normalized = Vec::new();

    for value in values {
        if !normalized.contains(&value) {
            normalized.push(value);
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use gpui::SharedString;

    use super::{CheckboxGroupParentStatus, CheckboxGroupRuntime};
    use crate::checkbox_group::CheckboxGroupProps;

    fn values(values: &[&str]) -> Vec<SharedString> {
        values
            .iter()
            .map(|value| SharedString::from(*value))
            .collect()
    }

    #[test]
    fn parent_cycle_preserves_mixed_snapshot() {
        let mut runtime = CheckboxGroupRuntime::new(vec!["a".into()]);
        let props = CheckboxGroupProps::new(false, vec!["a".into(), "b".into()], None);

        let request = runtime.request_parent_value_change(&props).unwrap();
        assert_eq!(request.next_value(), values(&["a", "b"]));
        runtime.commit_value(request.next_value().to_vec());
        runtime.accept_request(&request);
        assert_eq!(runtime.parent_status, CheckboxGroupParentStatus::On);

        let request = runtime.request_parent_value_change(&props).unwrap();
        assert!(request.next_value().is_empty());
        runtime.commit_value(request.next_value().to_vec());
        runtime.accept_request(&request);
        assert_eq!(runtime.parent_status, CheckboxGroupParentStatus::Off);

        let request = runtime.request_parent_value_change(&props).unwrap();
        assert_eq!(request.next_value(), values(&["a"]));
    }
}
