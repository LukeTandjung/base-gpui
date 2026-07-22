use gpui::TestAppContext;

use super::support::{open_meter, read_observations, MeterTestConfig};

#[gpui::test]
fn default_rendering_uses_the_formatted_string(cx: &mut TestAppContext) {
    let window = open_meter(
        cx,
        MeterTestConfig {
            value: 50.0,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().formatted, "50%");
}

#[gpui::test]
fn display_override_receives_formatted_and_raw_value(cx: &mut TestAppContext) {
    let window = open_meter(
        cx,
        MeterTestConfig {
            value: 50.0,
            use_display: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    let call = observations
        .display_calls
        .last()
        .cloned()
        .expect("display closure called");
    assert_eq!(call, (String::from("50%"), 50.0));
}

#[gpui::test]
fn root_format_callback_output_is_used_verbatim(cx: &mut TestAppContext) {
    let window = open_meter(
        cx,
        MeterTestConfig {
            value: 150.0,
            use_format: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.last_root_state().formatted, "150 raw");
    assert_eq!(observations.format_calls.last().copied(), Some(150.0));
}
