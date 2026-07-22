use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::toggle_group::{ToggleGroupProps, ToggleGroupRuntime, ToggleGroupValueChangeDetails};

pub struct ToggleGroupContext<T: Clone + Eq + 'static> {
    runtime: Entity<ToggleGroupRuntime<T>>,
    props: Rc<ToggleGroupProps<T>>,
    controlled: Rc<Option<Vec<T>>>,
}

impl<T: Clone + Eq + 'static> Clone for ToggleGroupContext<T> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl<T: Clone + Eq + 'static> ToggleGroupContext<T> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Vec<T>>,
        default: Vec<T>,
        props: ToggleGroupProps<T>,
    ) -> Self {
        let value = controlled.clone().unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| ToggleGroupRuntime::new(value));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&ToggleGroupRuntime<T>, &ToggleGroupProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut ToggleGroupRuntime<T>) -> Output,
    ) -> Output {
        let controlled = self.controlled.as_ref().clone();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(value) = controlled.clone() {
                runtime.sync_value_from_context(value);
            }

            let output = update(runtime);

            if let Some(value) = controlled {
                runtime.sync_value_from_context(value);
            }

            cx.notify();
            output
        })
    }

    /// Routes a grouped toggle's accepted local activation into the group:
    /// computes the next group value, fires `on_value_change` with the same
    /// shared details object the toggle callback saw, and commits to
    /// uncontrolled state only when uncanceled.
    pub fn commit_toggle(
        &self,
        value: T,
        next_pressed: bool,
        details: &mut ToggleGroupValueChangeDetails,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled = self.controlled.as_ref().clone();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            let current = match controlled.as_ref() {
                Some(value) => value.clone(),
                None => runtime.value_vec(),
            };

            runtime.sync_value_from_context(current.clone());
            let outcome = runtime.request_commit(
                &current,
                &value,
                next_pressed,
                props.multiple(),
                props.disabled(),
            );

            if let Some(value) = controlled.clone() {
                runtime.sync_value_from_context(value);
            }

            outcome
        });

        if !outcome.changed() {
            return;
        }

        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(outcome.value(), details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if controlled.is_none() {
            let next_value = outcome.into_value();

            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_value(next_value);
                cx.notify();
            });
        }
    }
}
