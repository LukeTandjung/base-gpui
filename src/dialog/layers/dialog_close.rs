use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AnyElement, App, ClickEvent, Div, ElementId, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::dialog::{
    child_wiring::{DialogChildNode, DialogChildWiring},
    scoped_dialog_trigger_id, DialogCloseAction, DialogCloseStyleState, DialogContext,
    DialogOpenAction, DialogOpenChangeReason, DialogOpenChangeSource, DIALOG_POPUP_KEY_CONTEXT,
};

#[derive(derive_setters::Setters, IntoElement)]
pub struct DialogClose<P: Clone + 'static = ()> {
    #[setters(into)]
    id: ElementId,
    #[setters(skip)]
    base: Div,
    #[setters(skip)]
    children: Vec<AnyElement>,
    #[setters(skip)]
    context: Option<DialogContext<P>>,
    #[setters(skip)]
    focus_handle: Option<FocusHandle>,
    #[setters(skip)]
    scoped: bool,
    disabled: bool,
    #[setters(skip)]
    aria_label: Option<SharedString>,
    #[setters(skip)]
    style_with_state: Option<Rc<dyn Fn(DialogCloseStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for DialogClose<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("dialog-close"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            scoped: false,
            disabled: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for DialogClose<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for DialogClose<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogClose<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| close_focus_handle(&self.id, window, cx));
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.close_state(self.disabled, focus_handle.is_focused(window))
                })
            })
            .unwrap_or_else(|| {
                DialogCloseStyleState::new(self.disabled, false, focus_handle.is_focused(window))
            });
        let disabled = state.disabled;
        let click_context = self.context.clone();
        let press_context = self.context.clone();
        let escape_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let mut base = base.id(self.id).role(Role::Button);
        if let Some(aria_label) = self.aria_label {
            base = base.aria_label(aria_label);
        }

        base.track_focus(
            &focus_handle
                .tab_stop(!disabled)
                .tab_index(if disabled { -1 } else { 0 }),
        )
        .focusable()
        .key_context(DIALOG_POPUP_KEY_CONTEXT)
        .on_click(move |event, window, cx| {
            if disabled {
                return;
            }
            // Non-mouse clicks come from AT-dispatched a11y `Click` actions;
            // keyboard activation flows through `DialogOpenAction` below.
            let source = match event {
                ClickEvent::Mouse(_) => DialogOpenChangeSource::Pointer,
                _ => DialogOpenChangeSource::Unknown,
            };
            if let Some(context) = click_context.as_ref() {
                context.close(DialogOpenChangeReason::ClosePress, source, window, cx);
            }
        })
        .on_action(move |_: &DialogOpenAction, window, cx| {
            if disabled {
                return;
            }
            if let Some(context) = press_context.as_ref() {
                context.close(
                    DialogOpenChangeReason::ClosePress,
                    DialogOpenChangeSource::Keyboard,
                    window,
                    cx,
                );
            }
        })
        .on_action(move |_: &DialogCloseAction, window, cx| {
            if let Some(context) = escape_context.as_ref() {
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

impl<P: Clone + 'static> DialogChildNode<P> for DialogClose<P> {
    fn with_dialog_context(mut self, context: DialogContext<P>) -> Self {
        if !self.scoped {
            self.id = scoped_dialog_trigger_id(&context.root_id(), &self.id);
        }
        self.context = Some(context);
        self
    }

    fn wire_dialog_child(
        mut self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.id = wiring.scope_child_id(&self.id);
        let focus_handle = close_focus_handle(&self.id, window, cx);
        wiring.register_popup_focus_handle(focus_handle.clone());
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self
    }
}

impl<P: Clone + 'static> DialogClose<P> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Accessible name for the close button (typically icon-only, e.g. "Close").
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogCloseStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn close_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
