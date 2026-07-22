use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::wire_children, layers::popover_trigger::evaluate_safe_polygon_move,
    scoped_trigger_id, PopoverChild, PopoverContext, PopoverHandle,
    PopoverOpenChangeCompleteHandler, PopoverOpenChangeDetails, PopoverOpenChangeHandler,
    PopoverOpenChangeReason, PopoverOpenChangeSource, PopoverProps, PopoverRootStyleState,
};

type PopoverRootStyle<P> = Rc<dyn Fn(PopoverRootStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct PopoverRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<PopoverChild<P>>,
    default_open: bool,
    open: Option<bool>,
    default_trigger_id: Option<ElementId>,
    trigger_id: Option<Option<ElementId>>,
    modal: bool,
    handle: Option<PopoverHandle<P>>,
    on_open_change: Option<PopoverOpenChangeHandler<P>>,
    on_open_change_complete: Option<PopoverOpenChangeCompleteHandler<P>>,
    style_with_state: Option<PopoverRootStyle<P>>,
}

impl<P: Clone + 'static> Default for PopoverRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("popover"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            default_trigger_id: None,
            trigger_id: None,
            modal: false,
            handle: None,
            on_open_change: None,
            on_open_change_complete: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PopoverRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let root_id = self.id.clone();
        let controlled_trigger_id = self.trigger_id.clone().map(|trigger_id| {
            trigger_id.map(|trigger_id| scoped_trigger_id(&root_id, &trigger_id))
        });
        let default_trigger_id = self
            .default_trigger_id
            .clone()
            .map(|trigger_id| scoped_trigger_id(&root_id, &trigger_id));
        let context = PopoverContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            controlled_trigger_id,
            default_trigger_id,
            PopoverProps::new(
                self.modal,
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
        let focus_handles = wired_children.focus_handles;
        let popup_focus_handles = wired_children.popup_focus_handles;
        let children = wired_children.children;

        let focused = focus_handles
            .iter()
            .any(|focus_handle| focus_handle.is_focused(window));
        let close_for_focus_out = context.update(cx, |runtime| {
            runtime.sync_triggers(triggers);
            runtime.clear_label_metadata();
            for title_id in title_ids {
                runtime.register_title(title_id);
            }
            for description_id in description_ids {
                runtime.register_description(description_id);
            }
            runtime.sync_popup_focus_handles(popup_focus_handles);
            runtime.sync_focus(focused);
            runtime.take_focus_out_close_request()
        });
        if close_for_focus_out {
            context.close(
                PopoverOpenChangeReason::FocusOut,
                PopoverOpenChangeSource::None,
                window,
                cx,
            );
        }
        if let Some(focus_handle) = context.update(cx, |runtime| runtime.take_popup_focus_on_open())
        {
            focus_handle.focus(window, cx);
        }

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let move_context = context.clone();
        base.on_mouse_move(move |event, window, cx| {
            evaluate_safe_polygon_move(&move_context, event.position, window, cx);
        })
        .children(children)
    }
}

impl<P: Clone + 'static> PopoverRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<PopoverChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<PopoverChild<P>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PopoverChild::Any(child.into_any_element()));
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
        on_open_change: impl Fn(bool, &mut PopoverOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &PopoverOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
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

    pub fn handle(mut self, handle: PopoverHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverRootStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
