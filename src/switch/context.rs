use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::switch::{
    SwitchCheckedChangeDetails, SwitchCheckedChangeReason, SwitchCheckedChangeSource, SwitchProps,
    SwitchRuntime,
};

pub struct SwitchContext {
    runtime: Entity<SwitchRuntime>,
    props: Rc<SwitchProps>,
    controlled: Rc<Option<Option<bool>>>,
}

impl Clone for SwitchContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl SwitchContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<bool>>,
        default: Option<bool>,
        props: SwitchProps,
    ) -> Self {
        let checked = controlled.unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| SwitchRuntime::new(checked));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&SwitchRuntime, &SwitchProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut SwitchRuntime) -> Output,
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

    pub fn toggle(&self, source: SwitchCheckedChangeSource, window: &mut Window, cx: &mut App) {
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
            return;
        }

        let next_checked = outcome.checked();
        let mut details =
            SwitchCheckedChangeDetails::new(SwitchCheckedChangeReason::None, source, true);

        if let Some(on_checked_change) = self.props.on_checked_change() {
            on_checked_change(next_checked, &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if controlled.is_none() {
            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_checked(next_checked);
                cx.notify();
            });
        }
    }
}
