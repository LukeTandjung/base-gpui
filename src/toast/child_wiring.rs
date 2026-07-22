use crate::toast::{ToastContext, ToastId};

/// Private wiring trait: provider-scoped parts receive the toast context.
pub trait ToastContextNode<P: Clone + 'static> {
    fn with_toast_context(self, context: ToastContext<P>) -> Self;
}

/// Private wiring trait: per-toast parts receive the context plus the toast
/// id they belong to (assigned by the viewport's content builder).
pub trait ToastPartNode<P: Clone + 'static> {
    fn with_toast(self, context: ToastContext<P>, id: ToastId) -> Self;
}
