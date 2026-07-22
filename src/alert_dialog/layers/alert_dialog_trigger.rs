use gpui::{
    AnyElement, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

use crate::alert_dialog::AlertDialogHandle;
use crate::dialog::{DialogChild, DialogTrigger, DialogTriggerStyleState};

/// Base UI Alert Dialog trigger: a Dialog trigger whose detached binding is
/// typed to [`AlertDialogHandle`], so it cannot be wired to a plain Dialog.
#[derive(IntoElement)]
pub struct AlertDialogTrigger<P: Clone + 'static = ()> {
    inner: DialogTrigger<P>,
    handle: Option<AlertDialogHandle<P>>,
}

impl<P: Clone + 'static> Default for AlertDialogTrigger<P> {
    fn default() -> Self {
        Self {
            inner: DialogTrigger::new().id("alert-dialog-trigger"),
            handle: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for AlertDialogTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.inner.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for AlertDialogTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl<P: Clone + 'static> RenderOnce for AlertDialogTrigger<P> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.into_dialog_trigger()
    }
}

impl<P: Clone + 'static> From<AlertDialogTrigger<P>> for DialogChild<P> {
    fn from(value: AlertDialogTrigger<P>) -> Self {
        Self::from(value.into_dialog_trigger())
    }
}

impl<P: Clone + 'static> AlertDialogTrigger<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.inner = self.inner.id(id);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }

    /// Accessible name for the trigger, used when the visible child is an icon
    /// or other non-text content. Forwarded to the underlying Dialog trigger's
    /// `aria_label`.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.inner = self.inner.aria_label(label);
        self
    }

    pub fn payload(mut self, payload: P) -> Self {
        self.inner = self.inner.payload(payload);
        self
    }

    pub fn handle(mut self, handle: AlertDialogHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.inner = self.inner.style_with_state(style);
        self
    }

    fn into_dialog_trigger(self) -> DialogTrigger<P> {
        match self.handle {
            Some(handle) => self.inner.handle(handle.dialog_handle()),
            None => self.inner,
        }
    }
}
