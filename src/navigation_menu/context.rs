use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::navigation_menu::{
    NavigationMenuProps, NavigationMenuRuntime, NavigationMenuValueChangeDetails,
    NavigationMenuValueChangeReason, NavigationMenuValueChangeSource,
};

/// Typed close command handed to a nested root so a nested `LinkPress`
/// close cascades to its parent root.
pub type NavigationMenuParentClose = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// Thin injection vehicle: entity plumbing plus the controlled/uncontrolled
/// rule. The controlled value (`Some(...)`) always wins over the uncontrolled
/// runtime value. Nested roots keep a parent close command so a nested
/// `LinkPress` close cascades up (Base UI's `setValue(null)` propagation).
pub struct NavigationMenuContext<T: Clone + Eq + 'static> {
    id: ElementId,
    runtime: Entity<NavigationMenuRuntime<T>>,
    props: Rc<NavigationMenuProps<T>>,
    controlled: Rc<Option<Option<T>>>,
    parent_close: Option<NavigationMenuParentClose>,
}

impl<T: Clone + Eq + 'static> Clone for NavigationMenuContext<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
            parent_close: self.parent_close.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuContext<T> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<T>>,
        default_value: Option<T>,
        props: NavigationMenuProps<T>,
    ) -> Self {
        let id = id.into();
        let initial = controlled.clone().unwrap_or(default_value);
        let orientation = props.orientation();
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            NavigationMenuRuntime::new(initial, orientation)
        });

        Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
            parent_close: None,
        }
    }

    pub fn root_id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn with_parent_close(mut self, parent_close: NavigationMenuParentClose) -> Self {
        self.parent_close = Some(parent_close);
        self
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&NavigationMenuRuntime<T>, &NavigationMenuProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut NavigationMenuRuntime<T>) -> Output,
    ) -> Output {
        let controlled = self.controlled.as_ref().clone();
        self.runtime.update(cx, |runtime, cx| {
            runtime.reconcile(controlled.clone());
            let output = update(runtime);
            runtime.reconcile(controlled);
            cx.notify();
            output
        })
    }

    /// The one value-changing command: resolves controlled vs uncontrolled,
    /// deduplicates, fires `on_value_change` (cancelable) before uncontrolled
    /// mutation, commits, records reason facts, and fires
    /// `on_open_change_complete` after a close (immediately: no transition
    /// infrastructure). A `LinkPress` close cascades to the parent root.
    pub fn set_value(
        &self,
        next: Option<T>,
        reason: NavigationMenuValueChangeReason,
        source: NavigationMenuValueChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let controlled = self.controlled.as_ref().clone();
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            runtime.reconcile(controlled.clone());
            let current = controlled
                .clone()
                .unwrap_or_else(|| runtime.current_value());
            runtime.request_value(current, next)
        });

        if !outcome.changed() {
            return false;
        }

        let next_value = outcome.into_value();
        let mut details = NavigationMenuValueChangeDetails::new(reason, source, true);
        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(next_value.clone(), &mut details, window, cx);
        }
        if details.is_canceled() {
            return false;
        }

        let closed = next_value.is_none();
        self.runtime.update(cx, |runtime, cx| {
            runtime.commit_value(next_value.clone(), controlled.is_none());
            runtime.record_change(reason);
            cx.notify();
        });

        if closed {
            if let Some(on_open_change_complete) = self.props.on_open_change_complete() {
                on_open_change_complete(None, &details, window, cx);
            }
            if !self.runtime.read(cx).focus_return_blocked() {
                if let Some(focus_handle) = self.runtime.read(cx).active_trigger_focus_handle() {
                    focus_handle.focus(window, cx);
                }
            }
            if reason == NavigationMenuValueChangeReason::LinkPress {
                if let Some(parent_close) = self.parent_close.clone() {
                    parent_close(window, cx);
                }
            }
        }

        true
    }

    pub fn close(
        &self,
        reason: NavigationMenuValueChangeReason,
        source: NavigationMenuValueChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_value(None, reason, source, window, cx)
    }
}
