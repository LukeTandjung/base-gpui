use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AccessibleAction, AnyElement, App, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::dialog::{
    child_wiring::{DialogChildNode, DialogChildWiring},
    DialogCloseAction, DialogContext, DialogHandle, DialogOpenAction, DialogOpenChangeReason,
    DialogOpenChangeSource, DialogTriggerMetadata, DialogTriggerStyleState,
    DIALOG_TRIGGER_KEY_CONTEXT,
};

type DialogTriggerStyle<P> = Rc<dyn Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct DialogTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<DialogContext<P>>,
    handle: Option<DialogHandle<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    disabled: bool,
    payload: Option<P>,
    order: usize,
    aria_label: Option<SharedString>,
    style_with_state: Option<DialogTriggerStyle<P>>,
}

impl<P: Clone + 'static> Default for DialogTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("dialog-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            handle: None,
            focus_handle: None,
            scoped: false,
            disabled: false,
            payload: None,
            order: 0,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for DialogTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for DialogTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogTrigger<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .clone()
            .or_else(|| self.handle.as_ref().and_then(DialogHandle::context));
        let source_id = self.id.clone();
        let scoped_id = context
            .as_ref()
            .filter(|_| !self.scoped)
            .map(|context| context.scope_trigger_id(&self.id))
            .unwrap_or_else(|| self.id.clone());
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| trigger_focus_handle(&scoped_id, window, cx));

        if self.context.is_none() {
            if let Some(context) = context.as_ref() {
                let trigger = DialogTriggerMetadata::new(
                    scoped_id.clone(),
                    source_id,
                    focus_handle.clone(),
                    self.disabled,
                    self.payload.clone(),
                    self.order,
                    true,
                );
                context.update(cx, |runtime| runtime.register_detached_trigger(trigger));
            }
        }

        let mut state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.trigger_state(&scoped_id, self.disabled, self.payload.is_some())
                })
            })
            .unwrap_or_else(|| {
                DialogTriggerStyleState::new(
                    self.disabled,
                    false,
                    false,
                    false,
                    self.payload.is_some(),
                    self.payload.clone(),
                )
            });
        state.focused = focus_handle.is_focused(window);
        let disabled = state.disabled;
        let expanded = state.open && state.active_trigger;
        let click_context = context.clone();
        let open_context = context.clone();
        let close_context = context.clone();
        let a11y_context = context.clone();
        let click_id = scoped_id.clone();
        let open_id = scoped_id.clone();
        let a11y_id = scoped_id.clone();
        let click_focus_handle = focus_handle.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let mut base = base
            .id(scoped_id)
            .role(Role::Button)
            .aria_expanded(expanded);
        if let Some(aria_label) = self.aria_label {
            base = base.aria_label(aria_label);
        }

        base.track_focus(
            &focus_handle
                .tab_stop(!disabled)
                .tab_index(if disabled { -1 } else { 0 }),
        )
        .focusable()
        .key_context(DIALOG_TRIGGER_KEY_CONTEXT)
        .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
            cx.stop_propagation();
            if disabled {
                return;
            }
            click_focus_handle.focus(window, cx);
            if let Some(context) = click_context.as_ref() {
                context.open_trigger(
                    click_id.clone(),
                    DialogOpenChangeReason::TriggerPress,
                    DialogOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            }
        })
        .on_action(move |_: &DialogOpenAction, window, cx| {
            if disabled {
                return;
            }
            if let Some(context) = open_context.as_ref() {
                context.open_trigger(
                    open_id.clone(),
                    DialogOpenChangeReason::TriggerPress,
                    DialogOpenChangeSource::Keyboard,
                    window,
                    cx,
                );
            }
        })
        .on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
            if disabled {
                return;
            }
            if let Some(context) = a11y_context.as_ref() {
                context.open_trigger(
                    a11y_id.clone(),
                    DialogOpenChangeReason::TriggerPress,
                    DialogOpenChangeSource::Unknown,
                    window,
                    cx,
                );
            }
        })
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
        .children(self.children)
    }
}

impl<P: Clone + 'static> DialogChildNode<P> for DialogTrigger<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_dialog_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let source_id = self.id.clone();
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = trigger_focus_handle(&scoped_id, window, cx);
        let order = wiring.register_trigger(
            scoped_id.clone(),
            source_id,
            focus_handle.clone(),
            self.disabled,
            self.payload.clone(),
        );
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self.order = order;
        self
    }
}

impl<P: Clone + 'static> DialogTrigger<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn payload(mut self, payload: P) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn handle(mut self, handle: DialogHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogTriggerStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn trigger_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
