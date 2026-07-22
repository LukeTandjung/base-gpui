use crate::radio_group::{RadioGroupProps, RadioGroupRadioMetadata, RadioGroupRuntime};

#[test]
fn option_value_can_distinguish_no_selection_from_selected_null_like_value() {
    let mut runtime = RadioGroupRuntime::<Option<&'static str>>::new(Some(None));
    let props = RadioGroupProps::new(None, None, false, false, false, None);

    runtime.sync_children(
        vec![
            RadioGroupRadioMetadata::new(None, false, false, false, 0),
            RadioGroupRadioMetadata::new(Some("express"), false, false, false, 1),
        ],
        Vec::new(),
    );
    runtime.reconcile(runtime.selected_value());

    assert!(
        runtime
            .radio_state(Some(&None), false, false, false, Some(0), &props)
            .checked
    );
    assert!(
        !runtime
            .radio_state(Some(&Some("express")), false, false, false, Some(1), &props)
            .checked
    );

    runtime.reconcile(None);

    assert!(
        !runtime
            .radio_state(Some(&None), false, false, false, Some(0), &props)
            .checked
    );
}
