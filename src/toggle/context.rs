use std::rc::Rc;

use gpui::{App, ElementId, Entity, Window};

use crate::toggle::{
    TogglePressedChangeDetails, TogglePressedChangeReason, TogglePressedChangeSource, ToggleProps,
    ToggleRuntime,
};

pub struct ToggleContext {
    runtime: Entity<ToggleRuntime>,
    props: Rc<ToggleProps>,
    controlled: Rc<Option<Option<bool>>>,
}

impl Clone for ToggleContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl ToggleContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Option<bool>>,
        default: Option<bool>,
        props: ToggleProps,
    ) -> Self {
        let pressed = controlled.unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| ToggleRuntime::new(pressed));

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&ToggleRuntime, &ToggleProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut ToggleRuntime) -> Output,
    ) -> Output {
        let controlled = *self.controlled.as_ref();
        let disabled = self.props.disabled();

        self.runtime.update(cx, |runtime, cx| {
            runtime.sync_own_disabled(disabled);
            if let Some(pressed) = controlled {
                runtime.sync_pressed_from_context(pressed);
            }

            let output = update(runtime);

            if let Some(pressed) = controlled {
                runtime.sync_pressed_from_context(pressed);
            }

            cx.notify();
            output
        })
    }

    pub fn toggle(&self, source: TogglePressedChangeSource, window: &mut Window, cx: &mut App) {
        let controlled = *self.controlled.as_ref();
        let disabled = self.props.disabled();
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            runtime.sync_own_disabled(disabled);
            let current = match controlled {
                Some(pressed) => pressed,
                None => runtime.pressed_value(),
            };

            runtime.sync_pressed_from_context(current);
            let outcome = runtime.request_toggle();

            if let Some(pressed) = controlled {
                runtime.sync_pressed_from_context(pressed);
            }

            outcome
        });

        if !outcome.changed() {
            return;
        }

        let next_pressed = outcome.pressed();
        let mut details =
            TogglePressedChangeDetails::new(TogglePressedChangeReason::None, source, true);

        if let Some(on_pressed_change) = self.props.on_pressed_change() {
            on_pressed_change(next_pressed, &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        if controlled.is_none() {
            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_pressed(next_pressed);
                cx.notify();
            });
        }
    }
}
