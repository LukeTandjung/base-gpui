use gpui::{actions, App, KeyBinding};

pub const NUMBER_FIELD_KEY_CONTEXT: &str = "NumberField";

actions!(
    base_gpui_number_field,
    [
        NumberFieldStepUp,
        NumberFieldStepDown,
        NumberFieldStepUpSmall,
        NumberFieldStepDownSmall,
        NumberFieldStepUpLarge,
        NumberFieldStepDownLarge,
        NumberFieldMin,
        NumberFieldMax,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(NUMBER_FIELD_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("up", NumberFieldStepUp, context),
        KeyBinding::new("down", NumberFieldStepDown, context),
        KeyBinding::new("alt-up", NumberFieldStepUpSmall, context),
        KeyBinding::new("alt-down", NumberFieldStepDownSmall, context),
        KeyBinding::new("shift-up", NumberFieldStepUpLarge, context),
        KeyBinding::new("shift-down", NumberFieldStepDownLarge, context),
        KeyBinding::new("home", NumberFieldMin, context),
        KeyBinding::new("end", NumberFieldMax, context),
    ]);
}
