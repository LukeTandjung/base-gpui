use std::cell::RefCell;

thread_local! {
    static FIELD_ITEM_DISABLED_STACK: RefCell<Vec<bool>> = const { RefCell::new(Vec::new()) };
}

pub fn current_field_item_disabled() -> bool {
    FIELD_ITEM_DISABLED_STACK
        .with(|stack| stack.borrow().last().copied())
        .unwrap_or(false)
}

pub fn with_field_item_disabled<Output>(disabled: bool, f: impl FnOnce() -> Output) -> Output {
    struct FieldItemGuard;

    impl Drop for FieldItemGuard {
        fn drop(&mut self) {
            FIELD_ITEM_DISABLED_STACK.with(|stack| {
                stack.borrow_mut().pop();
            });
        }
    }

    FIELD_ITEM_DISABLED_STACK.with(|stack| stack.borrow_mut().push(disabled));
    let _guard = FieldItemGuard;

    f()
}
