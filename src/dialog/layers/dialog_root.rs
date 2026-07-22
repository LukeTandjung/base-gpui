use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::dialog::{
    child_wiring::wire_children, scoped_dialog_trigger_id, DialogChild, DialogContext,
    DialogHandle, DialogModalMode, DialogOpenChangeCompleteHandler, DialogOpenChangeDetails,
    DialogOpenChangeHandler, DialogProps, DialogRootStyleState,
};

type DialogRootStyle<P> = Rc<dyn Fn(DialogRootStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct DialogRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<DialogChild<P>>,
    default_open: bool,
    open: Option<bool>,
    default_trigger_id: Option<ElementId>,
    trigger_id: Option<Option<ElementId>>,
    modal_mode: DialogModalMode,
    disable_pointer_dismissal: bool,
    handle: Option<DialogHandle<P>>,
    on_open_change: Option<DialogOpenChangeHandler<P>>,
    on_open_change_complete: Option<DialogOpenChangeCompleteHandler<P>>,
    style_with_state: Option<DialogRootStyle<P>>,
}

impl<P: Clone + 'static> Default for DialogRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("dialog"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            default_trigger_id: None,
            trigger_id: None,
            modal_mode: DialogModalMode::Modal,
            disable_pointer_dismissal: false,
            handle: None,
            on_open_change: None,
            on_open_change_complete: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for DialogRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for DialogRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let root_id = self.id.clone();
        let controlled_trigger_id = self.trigger_id.clone().map(|trigger_id| {
            trigger_id.map(|trigger_id| scoped_dialog_trigger_id(&root_id, &trigger_id))
        });
        let default_trigger_id = self
            .default_trigger_id
            .clone()
            .map(|trigger_id| scoped_dialog_trigger_id(&root_id, &trigger_id));
        let context = DialogContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            controlled_trigger_id,
            default_trigger_id,
            DialogProps::new(
                self.modal_mode,
                self.disable_pointer_dismissal,
                self.on_open_change,
                self.on_open_change_complete,
            ),
        );

        if let Some(handle) = self.handle.as_ref() {
            handle.bind(context.clone());
        }

        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let triggers = wired_children.triggers;
        let title_ids = wired_children.title_ids;
        let description_ids = wired_children.description_ids;
        let popup_focus_handles = wired_children.popup_focus_handles;
        let children = wired_children.children;

        context.update(cx, |runtime| {
            runtime.sync_triggers(triggers);
            runtime.sync_title_ids(title_ids);
            runtime.sync_description_ids(description_ids);
            runtime.sync_popup_focus_handles(popup_focus_handles);
        });

        if let Some(focus_handle) = context.update(cx, |runtime| runtime.take_popup_focus_on_open())
        {
            focus_handle.focus(window, cx);
        }

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.children(children)
    }
}

impl<P: Clone + 'static> DialogRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<DialogChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<DialogChild<P>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(DialogChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &DialogOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal_mode = if modal {
            DialogModalMode::Modal
        } else {
            DialogModalMode::NonModal
        };
        self
    }

    pub fn modal_mode(mut self, modal_mode: DialogModalMode) -> Self {
        self.modal_mode = modal_mode;
        self
    }

    pub fn trap_focus(mut self) -> Self {
        self.modal_mode = DialogModalMode::TrapFocus;
        self
    }

    pub fn disable_pointer_dismissal(mut self, disable_pointer_dismissal: bool) -> Self {
        self.disable_pointer_dismissal = disable_pointer_dismissal;
        self
    }

    pub fn trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.trigger_id = Some(Some(trigger_id.into()));
        self
    }

    pub fn no_trigger_id(mut self) -> Self {
        self.trigger_id = Some(None);
        self
    }

    pub fn default_trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.default_trigger_id = Some(trigger_id.into());
        self
    }

    pub fn handle(mut self, handle: DialogHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
