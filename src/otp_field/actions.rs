use gpui::{actions, App, KeyBinding};

use crate::primitives::input::InputPaste;

pub const OTP_FIELD_KEY_CONTEXT: &str = "OTPField";

actions!(
    base_gpui_otp_field,
    [
        OTPFieldPrevious,
        OTPFieldNext,
        OTPFieldFirst,
        OTPFieldLast,
        OTPFieldBackspace,
        OTPFieldDelete,
        OTPFieldClear,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(OTP_FIELD_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("left", OTPFieldPrevious, context),
        KeyBinding::new("right", OTPFieldNext, context),
        KeyBinding::new("up", OTPFieldFirst, context),
        KeyBinding::new("down", OTPFieldLast, context),
        KeyBinding::new("home", OTPFieldFirst, context),
        KeyBinding::new("end", OTPFieldLast, context),
        KeyBinding::new("backspace", OTPFieldBackspace, context),
        KeyBinding::new("delete", OTPFieldDelete, context),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", OTPFieldFirst, context),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", OTPFieldLast, context),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-backspace", OTPFieldClear, context),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", InputPaste, context),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-left", OTPFieldFirst, context),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-right", OTPFieldLast, context),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-backspace", OTPFieldClear, context),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", InputPaste, context),
    ]);
}
