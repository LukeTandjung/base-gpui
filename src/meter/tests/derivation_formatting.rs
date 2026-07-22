use std::{cell::RefCell, rc::Rc};

use crate::meter::{MeterFormatHandler, MeterRuntime};

#[test]
fn default_formatting_is_percent_of_range() {
    let state = MeterRuntime::new(50.0, 0.0, 100.0, None).state();
    assert_eq!(state.formatted, "50%");

    // Non-default range: text stays in sync with the indicator fill.
    let state = MeterRuntime::new(30.0, 20.0, 40.0, None).state();
    assert_eq!(state.formatted, "50%");
}

#[test]
fn custom_format_receives_raw_unclamped_value() {
    let calls: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::from([])));
    let observed = Rc::clone(&calls);
    let format: MeterFormatHandler = Rc::new(move |value| {
        observed.borrow_mut().push(value);
        format!("{value} bytes")
    });

    let state = MeterRuntime::new(150.0, 0.0, 100.0, Some(&format)).state();
    assert_eq!(state.formatted, "150 bytes");
    assert_eq!(calls.borrow().as_slice(), &[150.0]);
}
