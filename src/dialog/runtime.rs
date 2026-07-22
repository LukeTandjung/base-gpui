use gpui::{ElementId, FocusHandle, SharedString};

use crate::dialog::{
    DialogBackdropStyleState, DialogCloseStyleState, DialogDescriptionStyleState, DialogModalMode,
    DialogPopupStyleState, DialogPortalStyleState, DialogProps, DialogRootStyleState,
    DialogTitleStyleState, DialogTriggerStyleState, DialogViewportStyleState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DialogOpenChangeReason {
    TriggerPress,
    OutsidePress,
    EscapeKey,
    ClosePress,
    FocusOut,
    ImperativeAction,
    Swipe,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DialogOpenChangeSource {
    Pointer,
    Touch,
    Keyboard,
    Focus,
    Imperative,
    Unknown,
    #[default]
    None,
}

#[derive(Clone)]
pub struct DialogOpenChangeDetails<P: Clone + 'static> {
    reason: DialogOpenChangeReason,
    source: DialogOpenChangeSource,
    trigger_id: Option<ElementId>,
    payload: Option<P>,
    cancelable: bool,
    canceled: bool,
    prevent_unmount_on_close: bool,
}

impl<P: Clone + 'static> DialogOpenChangeDetails<P> {
    pub fn new(
        reason: DialogOpenChangeReason,
        source: DialogOpenChangeSource,
        trigger_id: Option<ElementId>,
        payload: Option<P>,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            trigger_id,
            payload,
            cancelable,
            canceled: false,
            prevent_unmount_on_close: false,
        }
    }

    pub fn reason(&self) -> DialogOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> DialogOpenChangeSource {
        self.source
    }

    pub fn trigger_id(&self) -> Option<&ElementId> {
        self.trigger_id.as_ref()
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
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

    pub fn prevent_unmount_on_close(&mut self) {
        self.prevent_unmount_on_close = true;
    }

    pub fn prevents_unmount_on_close(&self) -> bool {
        self.prevent_unmount_on_close
    }
}

#[derive(Clone)]
pub struct DialogTriggerMetadata<P: Clone + 'static> {
    id: ElementId,
    source_id: ElementId,
    focus_handle: Option<FocusHandle>,
    disabled: bool,
    payload: Option<P>,
    order: usize,
    detached: bool,
}

impl<P: Clone + 'static> DialogTriggerMetadata<P> {
    pub fn new(
        id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        disabled: bool,
        payload: Option<P>,
        order: usize,
        detached: bool,
    ) -> Self {
        Self {
            id,
            source_id,
            focus_handle: Some(focus_handle),
            disabled,
            payload,
            order,
            detached,
        }
    }

    pub fn id(&self) -> &ElementId {
        &self.id
    }

    pub fn source_id(&self) -> &ElementId {
        &self.source_id
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn detached(&self) -> bool {
        self.detached
    }
}

pub struct DialogOpenChangeOutcome<P: Clone + 'static> {
    changed: bool,
    open: bool,
    active_trigger_id: Option<ElementId>,
    source_trigger_id: Option<ElementId>,
    payload: Option<P>,
}

impl<P: Clone + 'static> DialogOpenChangeOutcome<P> {
    fn new(
        changed: bool,
        open: bool,
        active_trigger_id: Option<ElementId>,
        source_trigger_id: Option<ElementId>,
        payload: Option<P>,
    ) -> Self {
        Self {
            changed,
            open,
            active_trigger_id,
            source_trigger_id,
            payload,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn open(&self) -> bool {
        self.open
    }

    pub fn source_trigger_id(&self) -> Option<&ElementId> {
        self.source_trigger_id.as_ref()
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn into_parts(self) -> (bool, Option<ElementId>, Option<ElementId>, Option<P>) {
        (
            self.open,
            self.active_trigger_id,
            self.source_trigger_id,
            self.payload,
        )
    }
}

#[derive(Clone)]
pub struct DialogRuntime<P: Clone + 'static> {
    open: bool,
    mounted: bool,
    prevent_unmount_on_close: bool,
    modal_mode: DialogModalMode,
    disable_pointer_dismissal: bool,
    triggers: Vec<DialogTriggerMetadata<P>>,
    active_trigger_id: Option<ElementId>,
    active_payload: Option<P>,
    title_ids: Vec<ElementId>,
    description_ids: Vec<ElementId>,
    popup_focus_handles: Vec<FocusHandle>,
    previous_focus: Option<FocusHandle>,
    focus_popup_on_open: bool,
    nested: bool,
    nested_dialog_count: usize,
}

impl<P: Clone + 'static> DialogRuntime<P> {
    /// Creates the runtime with the initially observed open state and trigger association.
    pub fn new(
        open: bool,
        active_trigger_id: Option<ElementId>,
        modal_mode: DialogModalMode,
        disable_pointer_dismissal: bool,
    ) -> Self {
        Self {
            open,
            mounted: open,
            prevent_unmount_on_close: false,
            modal_mode,
            disable_pointer_dismissal,
            triggers: Vec::new(),
            active_trigger_id,
            active_payload: None,
            title_ids: Vec::new(),
            description_ids: Vec::new(),
            popup_focus_handles: Vec::new(),
            previous_focus: None,
            focus_popup_on_open: open,
            nested: false,
            nested_dialog_count: 0,
        }
    }

    /// Synchronizes the runtime with externally observed root props.
    pub fn sync_props(&mut self, props: &DialogProps<P>) {
        self.modal_mode = props.modal_mode();
        self.disable_pointer_dismissal = props.disable_pointer_dismissal();
    }

    /// Reconciles controlled open state from the root context.
    pub fn sync_open_from_context(&mut self, open: bool) {
        self.open = open;
        self.mounted = open || self.prevent_unmount_on_close;
        if open {
            self.prevent_unmount_on_close = false;
        }
    }

    /// Reconciles controlled active trigger state from the root context.
    pub fn sync_trigger_id_from_context(&mut self, trigger_id: Option<ElementId>) {
        self.active_trigger_id = trigger_id;
        self.active_payload = self
            .active_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id))
            .and_then(|trigger| trigger.payload.clone());
    }

    /// Reconciles trigger metadata observed in the current render pass.
    pub fn sync_triggers(&mut self, mut triggers: Vec<DialogTriggerMetadata<P>>) {
        triggers.sort_by_key(DialogTriggerMetadata::order);
        triggers.extend(
            self.triggers
                .iter()
                .filter(|trigger| trigger.detached())
                .cloned(),
        );
        self.triggers = triggers;
        if let Some(active_trigger_id) = self.active_trigger_id.clone() {
            self.active_payload = self
                .trigger_by_id(&active_trigger_id)
                .and_then(|trigger| trigger.payload.clone())
                .or_else(|| self.active_payload.clone());
        }
    }

    /// Registers a detached trigger observed outside the root child tree.
    pub fn register_detached_trigger(&mut self, trigger: DialogTriggerMetadata<P>) {
        self.triggers
            .retain(|existing| existing.id() != trigger.id());
        self.triggers.push(trigger);
    }

    /// Reconciles logical title metadata observed in the current render pass.
    pub fn sync_title_ids(&mut self, ids: Vec<ElementId>) {
        self.title_ids = ids;
    }

    /// Reconciles logical description metadata observed in the current render pass.
    pub fn sync_description_ids(&mut self, ids: Vec<ElementId>) {
        self.description_ids = ids;
    }

    /// Reconciles focus handles inside the popup observed in the current render pass.
    pub fn sync_popup_focus_handles(&mut self, handles: Vec<FocusHandle>) {
        self.popup_focus_handles = handles;
    }

    /// Captures the focus handle that should be restored if no trigger receives final focus.
    pub fn capture_previous_focus(&mut self, focus: Option<FocusHandle>) {
        self.previous_focus = focus;
    }

    /// Returns the currently observed open value.
    pub fn open_value(&self) -> bool {
        self.open
    }

    /// Returns whether pointer dismissal should currently close this dialog.
    pub fn pointer_dismissal_enabled(&self) -> bool {
        !self.disable_pointer_dismissal
    }

    /// Requests opening from an optional trigger, allowing missing trigger ids to open without association.
    pub fn request_open(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> DialogOpenChangeOutcome<P> {
        let trigger = trigger_id.as_ref().and_then(|id| self.trigger_by_id(id));
        if trigger
            .map(DialogTriggerMetadata::disabled)
            .unwrap_or(false)
        {
            return DialogOpenChangeOutcome::new(
                false,
                current,
                self.active_trigger_id.clone(),
                None,
                None,
            );
        }

        let active_trigger_id = trigger.map(|trigger| trigger.id().clone());
        let source_trigger_id = trigger.map(|trigger| trigger.source_id().clone());
        let payload = trigger.and_then(|trigger| trigger.payload.clone());
        let active_changed = self.active_trigger_id != active_trigger_id;
        let changed = !current || active_changed;

        DialogOpenChangeOutcome::new(changed, true, active_trigger_id, source_trigger_id, payload)
    }

    /// Requests opening with an explicit payload and no trigger association.
    pub fn request_open_with_payload(
        &self,
        current: bool,
        payload: P,
    ) -> DialogOpenChangeOutcome<P> {
        let changed = !current || self.active_trigger_id.is_some() || self.active_payload.is_some();
        DialogOpenChangeOutcome::new(changed, true, None, None, Some(payload))
    }

    /// Requests closing while preserving trigger and payload details for callbacks/focus return.
    pub fn request_close(&self, current: bool) -> DialogOpenChangeOutcome<P> {
        if !current && !self.prevent_unmount_on_close {
            return DialogOpenChangeOutcome::new(
                false,
                current,
                self.active_trigger_id.clone(),
                self.active_source_trigger_id(),
                self.active_payload(),
            );
        }

        DialogOpenChangeOutcome::new(
            true,
            false,
            self.active_trigger_id.clone(),
            self.active_source_trigger_id(),
            self.active_payload(),
        )
    }

    /// Commits an accepted open/close transition.
    pub fn commit_open(
        &mut self,
        open: bool,
        active_trigger_id: Option<ElementId>,
        active_payload: Option<P>,
        prevent_unmount_on_close: bool,
        commit_open_state: bool,
        commit_trigger_state: bool,
    ) {
        if commit_trigger_state {
            self.active_trigger_id = active_trigger_id;
            self.active_payload = active_payload;
        }

        if commit_open_state {
            self.open = open;
        }

        if open {
            self.mounted = true;
            self.prevent_unmount_on_close = false;
            self.focus_popup_on_open = true;
        } else {
            self.prevent_unmount_on_close = prevent_unmount_on_close;
            self.mounted = prevent_unmount_on_close;
            self.focus_popup_on_open = false;
        }
    }

    /// Forces a prevented close to unmount.
    pub fn force_unmount(&mut self) {
        if !self.open {
            self.prevent_unmount_on_close = false;
            self.mounted = false;
        }
    }

    /// Returns whether a trigger exists and is enabled.
    pub fn can_open_trigger(&self, trigger_id: &ElementId) -> bool {
        self.trigger_by_id(trigger_id)
            .map(|trigger| !trigger.disabled())
            .unwrap_or(false)
    }

    /// Returns the active trigger focus handle, if any.
    pub fn active_trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.active_trigger()
            .and_then(DialogTriggerMetadata::focus_handle)
    }

    /// Returns the captured previous focus handle, if any.
    pub fn previous_focus_handle(&self) -> Option<FocusHandle> {
        self.previous_focus.clone()
    }

    /// Takes a pending popup focus request after an open transition.
    pub fn take_popup_focus_on_open(&mut self) -> Option<FocusHandle> {
        if !self.open || !self.focus_popup_on_open || !self.modal_mode.traps_focus() {
            return None;
        }
        self.focus_popup_on_open = false;
        self.popup_focus_handles.first().cloned()
    }

    /// Returns the next focus handle within the popup focus set, wrapping at either end.
    pub fn popup_focus_neighbor(
        &self,
        current: Option<&FocusHandle>,
        reverse: bool,
    ) -> Option<FocusHandle> {
        if self.popup_focus_handles.is_empty() {
            return None;
        }

        let current_index = current.and_then(|current| {
            self.popup_focus_handles
                .iter()
                .position(|handle| handle == current)
        });
        let len = self.popup_focus_handles.len();
        let next_index = match (current_index, reverse) {
            (Some(0), true) | (None, true) => len - 1,
            (Some(index), true) => index.saturating_sub(1),
            (Some(index), false) => (index + 1) % len,
            (None, false) => 0,
        };

        self.popup_focus_handles.get(next_index).cloned()
    }

    pub fn root_state(&self, props: &DialogProps<P>) -> DialogRootStyleState<P> {
        DialogRootStyleState::new(
            self.open,
            self.mounted_value(false),
            props.modal_mode(),
            props.disable_pointer_dismissal(),
            self.active_source_trigger_id(),
            self.active_payload(),
            self.nested,
            self.nested_dialog_count,
            !self.triggers.is_empty(),
        )
    }

    pub fn trigger_state(
        &self,
        trigger_id: &ElementId,
        disabled: bool,
        payload_present: bool,
    ) -> DialogTriggerStyleState<P> {
        let active = self.active_trigger_id.as_ref() == Some(trigger_id);
        let trigger_payload = self
            .trigger_by_id(trigger_id)
            .and_then(|trigger| trigger.payload.clone());

        DialogTriggerStyleState::new(
            disabled,
            self.open && active,
            active,
            false,
            payload_present || trigger_payload.is_some(),
            trigger_payload,
        )
    }

    pub fn portal_state(&self, keep_mounted: bool) -> DialogPortalStyleState {
        DialogPortalStyleState::new(self.open, self.mounted_value(keep_mounted))
    }

    pub fn backdrop_state(
        &self,
        keep_mounted: bool,
        force_rendered: bool,
    ) -> DialogBackdropStyleState {
        DialogBackdropStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.nested,
            force_rendered,
        )
    }

    pub fn viewport_state(&self, keep_mounted: bool) -> DialogViewportStyleState<P> {
        DialogViewportStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.nested,
            self.nested_dialog_count,
            self.active_source_trigger_id(),
            self.active_payload(),
        )
    }

    pub fn popup_state(&self, keep_mounted: bool) -> DialogPopupStyleState<P> {
        DialogPopupStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.nested,
            self.nested_dialog_count,
            self.active_source_trigger_id(),
            self.active_payload(),
            self.modal_mode,
        )
    }

    pub fn title_state(&self) -> DialogTitleStyleState<P> {
        DialogTitleStyleState::new(self.open, self.active_payload())
    }

    pub fn description_state(&self) -> DialogDescriptionStyleState<P> {
        DialogDescriptionStyleState::new(self.open, self.active_payload())
    }

    pub fn close_state(&self, disabled: bool, focused: bool) -> DialogCloseStyleState {
        DialogCloseStyleState::new(disabled, self.open, focused)
    }

    fn mounted_value(&self, keep_mounted: bool) -> bool {
        self.open || self.mounted || keep_mounted || self.prevent_unmount_on_close
    }

    fn trigger_by_id(&self, trigger_id: &ElementId) -> Option<&DialogTriggerMetadata<P>> {
        self.triggers
            .iter()
            .find(|trigger| trigger.id() == trigger_id)
    }

    fn active_trigger(&self) -> Option<&DialogTriggerMetadata<P>> {
        self.active_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id))
    }

    fn active_source_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger()
            .map(|trigger| trigger.source_id().clone())
            .or_else(|| self.active_trigger_id.clone())
    }

    pub fn active_payload(&self) -> Option<P> {
        self.active_trigger()
            .and_then(|trigger| trigger.payload.clone())
            .or_else(|| self.active_payload.clone())
    }
}

pub fn scoped_dialog_trigger_id(root_id: &ElementId, trigger_id: &ElementId) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(trigger_id.to_string())))
}

pub fn scoped_dialog_part_id(root_id: &ElementId, part: &str) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(part)))
}

#[cfg(test)]
mod tests {
    use gpui::ElementId;

    use crate::dialog::{DialogModalMode, DialogRuntime};

    #[test]
    fn default_runtime_starts_closed_and_unmounted() {
        let runtime = DialogRuntime::<()>::new(false, None, DialogModalMode::Modal, false);

        assert!(!runtime.open_value());
        assert!(!runtime.portal_state(false).mounted);
    }

    #[test]
    fn request_open_without_matching_trigger_opens_without_association() {
        let runtime = DialogRuntime::<()>::new(false, None, DialogModalMode::Modal, false);

        let outcome = runtime.request_open(false, Some(ElementId::from("missing")));

        assert!(outcome.changed());
        assert!(outcome.open());
        assert!(outcome.source_trigger_id().is_none());
    }

    #[test]
    fn closing_with_prevent_unmount_keeps_portal_mounted() {
        let mut runtime = DialogRuntime::<()>::new(true, None, DialogModalMode::Modal, false);

        runtime.commit_open(false, None, None, true, true, true);

        assert!(!runtime.open_value());
        assert!(runtime.portal_state(false).mounted);
    }
}
