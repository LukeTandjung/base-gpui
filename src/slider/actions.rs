use gpui::{actions, App, KeyBinding};

pub const SLIDER_THUMB_KEY_CONTEXT: &str = "SliderThumb";

actions!(
    base_gpui_slider,
    [
        SliderStepUp,
        SliderStepDown,
        SliderStepLeft,
        SliderStepRight,
        SliderStepUpLarge,
        SliderStepDownLarge,
        SliderStepLeftLarge,
        SliderStepRightLarge,
        SliderPageUp,
        SliderPageDown,
        SliderHome,
        SliderEnd,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(SLIDER_THUMB_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("up", SliderStepUp, context),
        KeyBinding::new("down", SliderStepDown, context),
        KeyBinding::new("left", SliderStepLeft, context),
        KeyBinding::new("right", SliderStepRight, context),
        KeyBinding::new("shift-up", SliderStepUpLarge, context),
        KeyBinding::new("shift-down", SliderStepDownLarge, context),
        KeyBinding::new("shift-left", SliderStepLeftLarge, context),
        KeyBinding::new("shift-right", SliderStepRightLarge, context),
        KeyBinding::new("pageup", SliderPageUp, context),
        KeyBinding::new("pagedown", SliderPageDown, context),
        KeyBinding::new("home", SliderHome, context),
        KeyBinding::new("end", SliderEnd, context),
    ]);
}
