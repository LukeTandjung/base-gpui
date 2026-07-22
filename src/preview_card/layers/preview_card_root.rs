use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::preview_card::{
    child_wiring::wire_children,
    layers::preview_card_trigger::{apply_focus_change, evaluate_safe_polygon_move},
    scoped_trigger_id, PreviewCardChild, PreviewCardContext, PreviewCardHandle,
    PreviewCardOpenChangeCompleteHandler, PreviewCardOpenChangeDetails,
    PreviewCardOpenChangeHandler, PreviewCardOpenChangeReason, PreviewCardOpenChangeSource,
    PreviewCardProps, DEFAULT_PREVIEW_CARD_CLOSE_DELAY, DEFAULT_PREVIEW_CARD_DELAY,
};

/// Provider-less hover/focus root (Base UI Preview Card has no provider or
/// delay group). Renders no styleable state of its own: Base UI's root state
/// is empty, so the root stays injection-only.
#[derive(IntoElement)]
pub struct PreviewCardRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<PreviewCardChild<P>>,
    default_open: bool,
    open: Option<bool>,
    default_trigger_id: Option<ElementId>,
    trigger_id: Option<Option<ElementId>>,
    delay: std::time::Duration,
    close_delay: std::time::Duration,
    handle: Option<PreviewCardHandle<P>>,
    on_open_change: Option<PreviewCardOpenChangeHandler<P>>,
    on_open_change_complete: Option<PreviewCardOpenChangeCompleteHandler<P>>,
}

impl<P: Clone + 'static> Default for PreviewCardRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("preview-card"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            default_trigger_id: None,
            trigger_id: None,
            delay: DEFAULT_PREVIEW_CARD_DELAY,
            close_delay: DEFAULT_PREVIEW_CARD_CLOSE_DELAY,
            handle: None,
            on_open_change: None,
            on_open_change_complete: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PreviewCardRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let root_id = self.id.clone();
        let controlled_trigger_id = self.trigger_id.clone().map(|trigger_id| {
            trigger_id.map(|trigger_id| scoped_trigger_id(&root_id, &trigger_id))
        });
        let default_trigger_id = self
            .default_trigger_id
            .clone()
            .map(|trigger_id| scoped_trigger_id(&root_id, &trigger_id));
        let context = PreviewCardContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            controlled_trigger_id,
            default_trigger_id,
            PreviewCardProps::new(
                self.delay,
                self.close_delay,
                self.on_open_change,
                self.on_open_change_complete,
            ),
        );

        if let Some(handle) = self.handle.as_ref() {
            handle.bind(context.clone());
        }

        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let triggers = wired_children.triggers;
        let trigger_focus_handles = wired_children.trigger_focus_handles;
        let children = wired_children.children;

        let focused_trigger_id = trigger_focus_handles
            .iter()
            .find(|(_, focus_handle)| focus_handle.is_focused(window))
            .map(|(id, _)| id.clone());
        let (focus_change, missing_trigger_close) = context.update(cx, |runtime| {
            runtime.begin_detached_trigger_collection();
            runtime.sync_children(triggers);
            let focus_change = runtime.sync_focused_trigger(focused_trigger_id);
            let missing_trigger_close = runtime.take_active_trigger_missing_close_request();
            (focus_change, missing_trigger_close)
        });

        apply_focus_change(&context, focus_change, window, cx);
        if missing_trigger_close {
            context.close(
                PreviewCardOpenChangeReason::TriggerHover,
                PreviewCardOpenChangeSource::None,
                window,
                cx,
            );
        }

        let move_context = context.clone();
        self.base
            .on_mouse_move(move |event, window, cx| {
                evaluate_safe_polygon_move(&move_context, event.position, window, cx);
            })
            .children(children)
    }
}

impl<P: Clone + 'static> PreviewCardRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<PreviewCardChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<PreviewCardChild<P>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PreviewCardChild::Any(child.into_any_element()));
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

    pub fn delay(mut self, delay: std::time::Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn close_delay(mut self, close_delay: std::time::Duration) -> Self {
        self.close_delay = close_delay;
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut PreviewCardOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &PreviewCardOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
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

    pub fn handle(mut self, handle: PreviewCardHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }
}
