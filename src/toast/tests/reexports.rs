//! Compile-time verification of the public toast API surface.

#[test]
fn public_surface_is_reexported() {
    use crate::toast::{
        create_toast_manager, ToastAction, ToastClose, ToastContent, ToastDescription,
        ToastManager, ToastOptions, ToastPortal, ToastPromiseOptions, ToastProvider, ToastRoot,
        ToastTitle, ToastViewport,
    };

    let _provider: ToastProvider = ToastProvider::new();
    let _viewport: ToastViewport = ToastViewport::new();
    let _root: ToastRoot = ToastRoot::new();
    let _content: ToastContent = ToastContent::new();
    let _title: ToastTitle = ToastTitle::new();
    let _description: ToastDescription = ToastDescription::new();
    let _close: ToastClose = ToastClose::new();
    let _action: ToastAction = ToastAction::new();
    let _portal: ToastPortal = ToastPortal::new();
    let manager: ToastManager = create_toast_manager::<()>();
    assert!(!manager.bound());
    let _options: ToastOptions<()> = ToastOptions::new().title("hello");
    let _promise: ToastPromiseOptions<(), i32, String> =
        ToastPromiseOptions::from_text("loading", "done", "failed");
}
