use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AnyElement, App, Div, ElementId, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::dialog::{
    child_wiring::{DialogChildNode, DialogChildWiring},
    scoped_dialog_trigger_id, DialogCloseAction, DialogContext, DialogFocusNextAction,
    DialogFocusPreviousAction, DialogOpenChangeReason, DialogOpenChangeSource, DialogPopupChild,
    DialogPopupStyleState, DIALOG_POPUP_KEY_CONTEXT,
};

pub type DialogPayloadContentBuilder<P> =
    Rc<dyn Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static>;

type DialogPopupStyle<P> = Rc<dyn Fn(DialogPopupStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct DialogPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<DialogPopupChild<P>>,
    context: Option<DialogContext<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    keep_mounted: bool,
    role: Role,
    aria_label: Option<SharedString>,
    payload_content: Option<DialogPayloadContentBuilder<P>>,
    style_with_state: Option<DialogPopupStyle<P>>,
}

impl<P: Clone + 'static> Default for DialogPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("dialog-popup"),
            base: div().relative().occlude(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            scoped: false,
            keep_mounted: false,
            role: Role::Dialog,
            aria_label: None,
            payload_content: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DialogPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogPopup<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| popup_focus_handle(&self.id, window, cx));
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.popup_state(self.keep_mounted)))
            .unwrap_or_else(|| {
                DialogPopupStyleState::new(
                    false,
                    self.keep_mounted,
                    false,
                    0,
                    None,
                    None,
                    Default::default(),
                )
            });
        if !state.mounted {
            return div();
        }

        let open = state.open;
        let context = self.context.clone();
        let close_context = context.clone();
        let next_context = context.clone();
        let previous_context = context.clone();
        let trap_focus = state.modal_mode.traps_focus();
        let mut children = Vec::new();
        if let Some(payload_content) = self.payload_content {
            let payload = context
                .as_ref()
                .and_then(|context| context.read(cx, |runtime, _| runtime.active_payload()));
            children.push(payload_content(payload.as_ref(), window, cx));
        }
        children.extend(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // Kept-mounted closed content stays role-less so it is absent from the
        // accessibility tree (there is no hidden/inert builder in this gpui revision).
        let mut base = base.id(self.id);
        if open {
            base = base.role(self.role);
            if let Some(aria_label) = self.aria_label {
                base = base.aria_label(aria_label);
            }
        }

        div().child(
            base.track_focus(&focus_handle.tab_stop(true).tab_index(0))
                .focusable()
                .key_context(DIALOG_POPUP_KEY_CONTEXT)
                .on_action(move |_: &DialogCloseAction, window, cx| {
                    if let Some(context) = close_context.as_ref() {
                        context.close(
                            DialogOpenChangeReason::EscapeKey,
                            DialogOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                })
                .on_action(move |_: &DialogFocusNextAction, window, cx| {
                    if trap_focus {
                        if let Some(context) = next_context.as_ref() {
                            context.focus_popup_neighbor(false, window, cx);
                        }
                    } else {
                        window.focus_next(cx);
                    }
                })
                .on_action(move |_: &DialogFocusPreviousAction, window, cx| {
                    if trap_focus {
                        if let Some(context) = previous_context.as_ref() {
                            context.focus_popup_neighbor(true, window, cx);
                        }
                    } else {
                        window.focus_prev(cx);
                    }
                })
                .children(children),
        )
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogPopup<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        if !self.scoped {
            self.id = scoped_dialog_trigger_id(&context.root_id(), &self.id);
        }
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_dialog_context(context.clone()))
            .collect();
        self
    }

    fn wire_dialog_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.id = wiring.scope_child_id(&self.id);
        let focus_handle = popup_focus_handle(&self.id, window, cx);
        wiring.register_popup_focus_handle(focus_handle.clone());
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self.children = wiring.wire_popup_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> DialogPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<DialogPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DialogPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    /// Overrides the popup's accessibility role. Defaults to [`Role::Dialog`];
    /// Alert Dialog sets [`Role::AlertDialog`].
    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    /// Accessible name for the dialog. gpui has no `aria-labelledby` id-reference
    /// builder, so consumers pass the title string directly.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn payload_content(
        mut self,
        content: impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.payload_content = Some(Rc::new(content));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogPopupStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn popup_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
