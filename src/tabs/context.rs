use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::tabs::{TabsProps, TabsRuntime};

pub struct TabsContext<T: Clone + Eq + 'static> {
    runtime: Entity<TabsRuntime<T>>,
    props: Rc<TabsProps<T>>,
    controlled: Rc<Option<Option<T>>>,
}

impl<T: Clone + Eq + 'static> Clone for TabsContext<T> {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsContext<T> {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<T>>,
        default: Option<T>,
        props: TabsProps<T>,
    ) -> Self {
        let selected = controlled.clone().unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| TabsRuntime::new(selected));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&TabsRuntime<T>, &TabsProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut TabsRuntime<T>) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);

            cx.notify();
            output
        })
    }

    pub fn select(&self, value: Option<T>, window: &mut Window, cx: &mut App) {
        let controlled = self.controlled.as_ref().clone();
        let orientation = self.props.orientation();
        let outcome = self.runtime.update(cx, |runtime, cx| {
            let current = match controlled.as_ref() {
                Some(selected) => selected.clone(),
                None => runtime.selected_value(),
            };

            let outcome = runtime.select_from(current, value, orientation, controlled.is_none());

            cx.notify();
            outcome
        });

        if !outcome.changed() {
            return;
        }

        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(outcome.value(), window, cx);
        }
    }
}
