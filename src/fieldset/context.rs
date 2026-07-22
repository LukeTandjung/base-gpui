use std::{cell::RefCell, rc::Rc};

use gpui::{App, ElementId, Entity, Window};

use crate::fieldset::{FieldsetProps, FieldsetRuntime};

thread_local! {
    static FIELDSET_CONTEXT_STACK: RefCell<Vec<FieldsetContext>> = const { RefCell::new(Vec::new()) };
}

pub fn current_fieldset_context() -> Option<FieldsetContext> {
    FIELDSET_CONTEXT_STACK.with(|stack| stack.borrow().last().cloned())
}

pub fn current_fieldset_disabled() -> bool {
    current_fieldset_context()
        .map(|context| context.disabled())
        .unwrap_or(false)
}

pub fn with_fieldset_context<Output>(
    context: FieldsetContext,
    f: impl FnOnce() -> Output,
) -> Output {
    struct FieldsetContextGuard;

    impl Drop for FieldsetContextGuard {
        fn drop(&mut self) {
            FIELDSET_CONTEXT_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    FIELDSET_CONTEXT_STACK.with(|stack| stack.borrow_mut().push(context));
    let _guard = FieldsetContextGuard;

    f()
}

pub struct FieldsetContext {
    runtime: Entity<FieldsetRuntime>,
    props: Rc<FieldsetProps>,
}

impl Clone for FieldsetContext {
    fn clone(&self) -> Self {
        Self {
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
        }
    }
}

impl FieldsetContext {
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        props: FieldsetProps,
    ) -> Self {
        let runtime = window.use_keyed_state(id, cx, |_, _| FieldsetRuntime::new());

        Self {
            runtime,
            props: Rc::new(props),
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&FieldsetRuntime, &FieldsetProps) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut FieldsetRuntime) -> Output,
    ) -> Output {
        self.runtime.update(cx, |runtime, cx| {
            let output = update(runtime);

            cx.notify();
            output
        })
    }

    pub fn disabled(&self) -> bool {
        self.props.disabled()
    }
}
