use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::checkbox::{
    CheckboxCheckedChangeDetails, CheckboxCheckedChangeReason, CheckboxCheckedChangeSource,
    CheckboxProps, CheckboxRuntime,
};

pub struct CheckboxContext {
    runtime: Entity<CheckboxRuntime>,
    props: Rc<CheckboxProps>,
    controlled: Rc<Option<Option<bool>>>,
}

impl Clone for CheckboxContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl CheckboxContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<bool>>,
        default: Option<bool>,
        props: CheckboxProps,
    ) -> Self {
        let checked = controlled.unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| CheckboxRuntime::new(checked));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&CheckboxRuntime, &CheckboxProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut CheckboxRuntime) -> Output,
    ) -> Output {
        let controlled = *self.controlled.as_ref();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(checked) = controlled {
                runtime.sync_checked_from_context(checked);
            }

            let output = update(runtime);

            if let Some(checked) = controlled {
                runtime.sync_checked_from_context(checked);
            }

            cx.notify();
            output
        })
    }

    pub fn request_toggle(
        &self,
        source: CheckboxCheckedChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<bool> {
        let controlled = *self.controlled.as_ref();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            let current = match controlled {
                Some(checked) => checked,
                None => runtime.checked_value(),
            };

            runtime.sync_checked_from_context(current);
            let outcome = runtime.request_toggle(props.disabled(), props.read_only());

            if let Some(checked) = controlled {
                runtime.sync_checked_from_context(checked);
            }

            outcome
        });

        if !outcome.changed() {
            return None;
        }

        let next_checked = outcome.checked();
        let mut details =
            CheckboxCheckedChangeDetails::new(CheckboxCheckedChangeReason::None, source, true);

        if let Some(on_checked_change) = self.props.on_checked_change() {
            on_checked_change(next_checked, &mut details, window, cx);
        }

        if details.is_canceled() {
            return None;
        }

        Some(next_checked)
    }

    pub fn commit_checked(&self, checked: bool, cx: &mut App) {
        if self.controlled.as_ref().is_some() {
            return;
        }

        self.runtime.update(cx, |runtime, cx| {
            if runtime.commit_checked(checked) {
                cx.notify();
            }
        });
    }

    pub fn toggle(&self, source: CheckboxCheckedChangeSource, window: &mut Window, cx: &mut App) {
        if let Some(next_checked) = self.request_toggle(source, window, cx) {
            self.commit_checked(next_checked, cx);
        }
    }
}
