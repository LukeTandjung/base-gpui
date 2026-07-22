use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, SharedString, Window};

use crate::checkbox_group::{
    CheckboxGroupChildMetadata, CheckboxGroupProps, CheckboxGroupRuntime,
    CheckboxGroupValueChangeDetails, CheckboxGroupValueChangeReason,
};

thread_local! {
    static CHECKBOX_GROUP_CONTEXT_STACK: RefCell<Vec<CheckboxGroupContext>> = const { RefCell::new(Vec::new()) };
}

pub fn current_checkbox_group_context() -> Option<CheckboxGroupContext> {
    CHECKBOX_GROUP_CONTEXT_STACK.with(|stack| stack.borrow().last().cloned())
}

pub fn with_checkbox_group_context<Output>(
    context: CheckboxGroupContext,
    f: impl FnOnce() -> Output,
) -> Output {
    struct CheckboxGroupContextGuard;

    impl Drop for CheckboxGroupContextGuard {
        fn drop(&mut self) {
            CHECKBOX_GROUP_CONTEXT_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    CHECKBOX_GROUP_CONTEXT_STACK.with(|stack| stack.borrow_mut().push(context));
    let _guard = CheckboxGroupContextGuard;

    f()
}

pub struct CheckboxGroupContext {
    runtime: Entity<CheckboxGroupRuntime>,
    props: Rc<CheckboxGroupProps>,
    controlled: Rc<Option<Vec<SharedString>>>,
}

impl Clone for CheckboxGroupContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled: Rc::clone(&self.controlled),
        }
    }
}

impl CheckboxGroupContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled: Option<Vec<SharedString>>,
        default: Vec<SharedString>,
        props: CheckboxGroupProps,
    ) -> Self {
        let value = controlled.clone().unwrap_or(default);
        let runtime = window.use_keyed_state(id, cx, |_, _| CheckboxGroupRuntime::new(value));
        if let Some(value) = controlled.as_ref() {
            runtime.update(cx, |runtime, _cx| {
                runtime.sync_value_from_context(value.clone());
            });
        }

        Self {
            runtime,
            props: Rc::new(props),
            controlled: Rc::new(controlled),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&CheckboxGroupRuntime, &CheckboxGroupProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut CheckboxGroupRuntime) -> Output,
    ) -> Output {
        let controlled = self.controlled.as_ref().clone();

        self.runtime.update(cx, |runtime, cx| {
            if let Some(value) = controlled.as_ref() {
                runtime.sync_value_from_context(value.clone());
            }

            let output = update(runtime);

            if let Some(value) = controlled.as_ref() {
                runtime.sync_value_from_context(value.clone());
            }

            cx.notify();
            output
        })
    }

    pub fn begin_registration_pass(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, _cx| {
            runtime.begin_registration_pass();
        });
    }

    pub fn finish_registration_pass(&self, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.finish_registration_pass() {
                cx.notify();
            }
        });
    }

    pub fn register_checkbox(&self, metadata: CheckboxGroupChildMetadata, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.register_checkbox(metadata) {
                cx.notify();
            }
        });
    }

    pub fn checked_for_value(&self, value: &SharedString, cx: &App) -> bool {
        self.read(cx, |runtime, _props| runtime.checked_for_value(value))
    }

    pub fn parent_checked(&self, cx: &App) -> bool {
        self.read(cx, |runtime, props| runtime.parent_checked(props))
    }

    pub fn parent_indeterminate(&self, cx: &App) -> bool {
        self.read(cx, |runtime, props| runtime.parent_indeterminate(props))
    }

    pub fn disabled(&self) -> bool {
        self.props.disabled()
    }

    pub fn toggle_child(
        &self,
        value: Option<SharedString>,
        checked: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled = self.controlled.as_ref().clone();
        let props = Rc::clone(&self.props);
        let request = self.runtime.update(cx, |runtime, _cx| {
            if let Some(value) = controlled.as_ref() {
                runtime.sync_value_from_context(value.clone());
            }

            runtime.request_child_value_change(value.as_ref(), checked, props.as_ref())
        });

        let Some(request) = request else {
            return;
        };

        self.accept_request(request, window, cx);
    }

    pub fn toggle_parent(&self, window: &mut Window, cx: &mut App) {
        let controlled = self.controlled.as_ref().clone();
        let props = Rc::clone(&self.props);
        let request = self.runtime.update(cx, |runtime, _cx| {
            if let Some(value) = controlled.as_ref() {
                runtime.sync_value_from_context(value.clone());
            }

            runtime.request_parent_value_change(props.as_ref())
        });

        let Some(request) = request else {
            return;
        };

        self.accept_request(request, window, cx);
    }

    fn accept_request(
        &self,
        request: crate::checkbox_group::CheckboxGroupValueChangeRequest,
        window: &mut Window,
        cx: &mut App,
    ) {
        let next_value = request.next_value().to_vec();
        let mut details =
            CheckboxGroupValueChangeDetails::new(CheckboxGroupValueChangeReason::None, true);

        if let Some(on_value_change) = self.props.on_value_change() {
            on_value_change(next_value.clone(), &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        let controlled = self.controlled.as_ref().is_some();
        self.runtime.update(cx, |runtime, cx| {
            if !controlled {
                runtime.commit_value(next_value);
            }
            runtime.accept_request(&request);
            cx.notify();
        });
    }
}
