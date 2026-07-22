use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::accordion::{
    AccordionChangeDetails, AccordionChangeReason, AccordionChangeSource,
    AccordionItemOpenChangeHandler, AccordionProps, AccordionRuntime,
};

pub struct AccordionContext<T: Clone + Eq + 'static> {
    runtime: Entity<AccordionRuntime<T>>,
    props: Rc<AccordionProps<T>>,
    controlled: Rc<Option<Vec<T>>>,
}

impl<T: Clone + Eq + 'static> Clone for AccordionContext<T> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionContext<T> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Vec<T>>,
        default: Vec<T>,
        props: AccordionProps<T>,
    ) -> Self {
        let values = controlled.clone().unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| AccordionRuntime::new(values));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&AccordionRuntime<T>, &AccordionProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut AccordionRuntime<T>) -> Output,
    ) -> Output {
        let controlled = self.controlled.as_ref().clone();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(values) = controlled.clone() {
                runtime.sync_values_from_context(values);
            }

            let output = update(runtime);

            if let Some(values) = controlled {
                runtime.sync_values_from_context(values);
            }

            cx.notify();
            output
        })
    }

    pub fn toggle_item(
        &self,
        value: T,
        item_disabled: bool,
        item_on_open_change: Option<AccordionItemOpenChangeHandler>,
        source: AccordionChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled = self.controlled.as_ref().clone();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            let current = controlled.clone().unwrap_or_else(|| runtime.open_values());

            runtime.sync_values_from_context(current);
            let outcome =
                runtime.request_toggle(&value, props.disabled(), item_disabled, props.multiple());

            if let Some(values) = controlled.clone() {
                runtime.sync_values_from_context(values);
            }

            outcome
        });

        if !outcome.changed() {
            return;
        }

        let next_open = outcome.next_open();
        let next_values = outcome.into_values();
        let mut details =
            AccordionChangeDetails::new(AccordionChangeReason::TriggerPress, source, true);

        if let Some(on_open_change) = item_on_open_change {
            on_open_change(next_open, &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(&next_values, &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if controlled.is_none() {
            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_values(next_values);
                cx.notify();
            });
        }
    }
}

pub struct AccordionItemContext<T: Clone + Eq + 'static> {
    accordion: AccordionContext<T>,
    value: T,
    index: usize,
    disabled: bool,
    on_open_change: Option<AccordionItemOpenChangeHandler>,
}

impl<T: Clone + Eq + 'static> Clone for AccordionItemContext<T> {
    fn clone(&self) -> Self {
        Self {
            accordion: self.accordion.clone(),
            value: self.value.clone(),
            index: self.index,
            disabled: self.disabled,
            on_open_change: self.on_open_change.clone(),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionItemContext<T> {
    pub fn new(
        accordion: AccordionContext<T>,
        value: T,
        index: usize,
        disabled: bool,
        on_open_change: Option<AccordionItemOpenChangeHandler>,
    ) -> Self {
        Self {
            accordion,
            value,
            index,
            disabled,
            on_open_change,
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&AccordionRuntime<T>, &AccordionProps<T>, &T, usize, bool) -> Output,
    ) -> Output {
        self.accordion.read(cx, |runtime, props| {
            read(runtime, props, &self.value, self.index, self.disabled)
        })
    }

    pub fn toggle(&self, source: AccordionChangeSource, window: &mut Window, cx: &mut App) {
        self.accordion.toggle_item(
            self.value.clone(),
            self.disabled,
            self.on_open_change.clone(),
            source,
            window,
            cx,
        );
    }

    pub fn root_keep_mounted(&self, cx: &App) -> bool {
        self.accordion.read(cx, |_, props| props.keep_mounted())
    }
}
