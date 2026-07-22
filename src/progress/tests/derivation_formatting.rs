use std::{cell::RefCell, rc::Rc};

use crate::progress::{ProgressFormatHandler, ProgressRuntime};

#[test]
fn default_formatting_is_percent_of_range() {
    let state = ProgressRuntime::new(Some(50.0), 0.0, 100.0, None).state();
    assert_eq!(state.formatted.as_deref(), Some("50%"));

    // Non-default range: text stays in sync with the indicator fill.
    let state = ProgressRuntime::new(Some(30.0), 20.0, 40.0, None).state();
    assert_eq!(state.formatted.as_deref(), Some("50%"));
}

#[test]
fn no_formatted_string_when_indeterminate() {
    let state = ProgressRuntime::new(None, 0.0, 100.0, None).state();
    assert_eq!(state.formatted, None);
}

#[test]
fn custom_format_receives_raw_unclamped_value() {
    let calls: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::from([])));
    let observed = Rc::clone(&calls);
    let format: ProgressFormatHandler = Rc::new(move |value| {
        observed.borrow_mut().push(value);
        format!("{value} bytes")
    });

    let state = ProgressRuntime::new(Some(150.0), 0.0, 100.0, Some(&format)).state();
    assert_eq!(state.formatted.as_deref(), Some("150 bytes"));
    assert_eq!(calls.borrow().as_slice(), &[150.0]);
}

#[test]
fn custom_format_is_not_called_when_indeterminate() {
    let calls: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::from([])));
    let observed = Rc::clone(&calls);
    let format: ProgressFormatHandler = Rc::new(move |value| {
        observed.borrow_mut().push(value);
        String::from("never")
    });

    let state = ProgressRuntime::new(None, 0.0, 100.0, Some(&format)).state();
    assert_eq!(state.formatted, None);
    assert!(calls.borrow().is_empty());

    let state = ProgressRuntime::new(Some(f64::NAN), 0.0, 100.0, Some(&format)).state();
    assert_eq!(state.formatted, None);
    assert!(calls.borrow().is_empty());
}
