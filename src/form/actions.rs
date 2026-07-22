use gpui::{actions, App};

pub const FORM_KEY_CONTEXT: &str = "Form";

actions!(base_gpui_form, [FormSubmitAction, FormValidateAction]);

pub fn init(_cx: &mut App) {}
