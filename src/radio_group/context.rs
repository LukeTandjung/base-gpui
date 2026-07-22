use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::radio_group::{
    RadioGroupProps, RadioGroupRuntime, RadioGroupValueChangeDetails, RadioGroupValueChangeReason,
    RadioGroupValueChangeSource,
};

pub struct RadioGroupContext<T: Clone + Eq + 'static> {
    runtime: Entity<RadioGroupRuntime<T>>,
    props: Rc<RadioGroupProps<T>>,
    controlled: Rc<Option<Option<T>>>,
}

impl<T: Clone + Eq + 'static> Clone for RadioGroupContext<T> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl<T: Clone + Eq + 'static> RadioGroupContext<T> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<T>>,
        default: Option<T>,
        props: RadioGroupProps<T>,
    ) -> Self {
        let selected = controlled.clone().unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| RadioGroupRuntime::new(selected));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&RadioGroupRuntime<T>, &RadioGroupProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut RadioGroupRuntime<T>) -> Output,
    ) -> Output {
        let controlled = self.controlled.as_ref().clone();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(selected) = controlled.clone() {
                runtime.sync_selected_from_context(selected);
            }

            let output = update(runtime);

            if let Some(selected) = controlled {
                runtime.sync_selected_from_context(selected);
            }

            cx.notify();
            output
        })
    }

    pub fn select(
        &self,
        value: T,
        source: RadioGroupValueChangeSource,
        radio_disabled: bool,
        radio_read_only: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled = self.controlled.as_ref().clone();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            let current = match controlled.as_ref() {
                Some(selected) => selected.clone(),
                None => runtime.selected_value(),
            };

            runtime.sync_selected_from_context(current.clone());
            let outcome = runtime.request_select(
                current,
                value,
                props.disabled() || radio_disabled,
                props.read_only() || radio_read_only,
            );

            if let Some(selected) = controlled.clone() {
                runtime.sync_selected_from_context(selected);
            }

            outcome
        });

        if !outcome.changed() {
            return;
        }

        let next_value = outcome.into_value();
        let mut details =
            RadioGroupValueChangeDetails::new(RadioGroupValueChangeReason::None, source, true);

        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(next_value.as_ref(), &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if controlled.is_none() {
            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_selected(next_value);
                cx.notify();
            });
        }
    }
}
