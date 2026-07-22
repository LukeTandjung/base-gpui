use gpui::{App, ElementId, Window};

use crate::dialog::DialogHandle;

/// A handle for driving an Alert Dialog imperatively (open/close/unmount) and
/// for binding detached [`crate::alert_dialog::AlertDialogTrigger`]s.
///
/// This is a genuinely distinct type from [`DialogHandle`], replacing Base UI's
/// private `__alertDialogBrand` marker: a plain Dialog handle cannot be passed
/// where an `AlertDialogHandle` is required, and vice versa.
///
/// ```compile_fail
/// let handle = base_gpui::dialog::create_dialog_handle::<()>();
/// let _root = base_gpui::alert_dialog::AlertDialogRoot::<()>::new().handle(handle);
/// ```
pub struct AlertDialogHandle<P: Clone + 'static>(DialogHandle<P>);

impl<P: Clone + 'static> Clone for AlertDialogHandle<P> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<P: Clone + 'static> Default for AlertDialogHandle<P> {
    fn default() -> Self {
        Self(DialogHandle::default())
    }
}

impl<P: Clone + 'static> AlertDialogHandle<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(
        &self,
        trigger_id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.0.open(trigger_id, window, cx)
    }

    pub fn open_with_payload(&self, payload: P, window: &mut Window, cx: &mut App) -> bool {
        self.0.open_with_payload(payload, window, cx)
    }

    pub fn close(&self, window: &mut Window, cx: &mut App) -> bool {
        self.0.close(window, cx)
    }

    pub fn unmount(&self, cx: &mut App) -> bool {
        self.0.unmount(cx)
    }

    pub fn is_open(&self, cx: &App) -> bool {
        self.0.is_open(cx)
    }

    /// Internal wiring accessor used by the Alert Dialog root/trigger layers to
    /// bind the underlying Dialog machinery. Do not pass the returned handle to
    /// a plain `DialogRoot`; that would bypass the alert-dialog invariants.
    pub fn dialog_handle(&self) -> DialogHandle<P> {
        self.0.clone()
    }
}

pub fn create_alert_dialog_handle<P: Clone + 'static>() -> AlertDialogHandle<P> {
    AlertDialogHandle::new()
}
