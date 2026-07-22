use gpui::{actions, App, KeyBinding};

pub const ACCORDION_TRIGGER_KEY_CONTEXT: &str = "AccordionTrigger";

actions!(base_gpui_accordion, [AccordionToggle]);

pub fn init(cx: &mut App) {
    let context = Some(ACCORDION_TRIGGER_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("space", AccordionToggle, context),
        KeyBinding::new("enter", AccordionToggle, context),
    ]);
}
