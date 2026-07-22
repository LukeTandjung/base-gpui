use std::{rc::Rc, time::Duration};

use gpui::{
    div, AnyElement, App, Div, ElementId, Entity, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::tooltip::{
    child_wiring::wire_children, scoped_trigger_id, TooltipChild, TooltipContext,
    TooltipDelayGroup, TooltipFocusChange, TooltipHandle, TooltipOpenChangeCompleteHandler,
    TooltipOpenChangeDetails, TooltipOpenChangeHandler, TooltipOpenChangeReason,
    TooltipOpenChangeSource, TooltipProps, TooltipProviderConfig, TooltipProviderStyleState,
    TooltipRootStyleState, TooltipTrackCursorAxis,
};

type TooltipRootStyle<P> = Rc<dyn Fn(TooltipRootStyleState<P>, Div) -> Div + 'static>;

pub enum TooltipProviderChild<P: Clone + 'static> {
    Root(Box<TooltipRoot<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for TooltipProviderChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Root(root) => (*root).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<TooltipRoot<P>> for TooltipProviderChild<P> {
    fn from(value: TooltipRoot<P>) -> Self {
        Self::Root(Box::new(value))
    }
}

impl<P: Clone + 'static> TooltipProviderChild<P> {
    fn with_provider(self, config: TooltipProviderConfig, delay_group: TooltipDelayGroup) -> Self {
        match self {
            Self::Root(root) => {
                let mut root = *root;
                root.provider_config = config;
                root.delay_group = delay_group;
                Self::Root(Box::new(root))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

#[derive(IntoElement)]
pub struct TooltipProvider<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<TooltipProviderChild<P>>,
    config: TooltipProviderConfig,
    style_with_state: Option<Rc<dyn Fn(TooltipProviderStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for TooltipProvider<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tooltip-provider"),
            base: div(),
            children: Vec::new(),
            config: TooltipProviderConfig::default(),
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for TooltipProvider<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for TooltipProvider<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let delay_group_entity: Entity<TooltipDelayGroup> = window.use_keyed_state(
            ElementId::from((self.id.clone(), "delay-group")),
            cx,
            |_, _| TooltipDelayGroup::new(),
        );
        let delay_group = delay_group_entity.read(cx).clone();
        let state = TooltipProviderStyleState::new(
            self.config.delay(),
            self.config.close_delay(),
            self.config.timeout(),
            delay_group.instant(),
        );
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        base.children(
            self.children
                .into_iter()
                .map(|child| child.with_provider(self.config, delay_group.clone()))
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl<P: Clone + 'static> TooltipProvider<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<TooltipProviderChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(TooltipProviderChild::Any(child.into_any_element()));
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.config = self.config.with_delay(delay);
        self
    }

    pub fn close_delay(mut self, close_delay: Duration) -> Self {
        self.config = self.config.with_close_delay(close_delay);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config = self.config.with_timeout(timeout);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TooltipProviderStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

#[derive(IntoElement)]
pub struct TooltipRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<TooltipChild<P>>,
    default_open: bool,
    open: Option<bool>,
    default_trigger_id: Option<ElementId>,
    trigger_id: Option<Option<ElementId>>,
    disabled: bool,
    disable_hoverable_popup: bool,
    track_cursor_axis: TooltipTrackCursorAxis,
    provider_config: TooltipProviderConfig,
    delay_group: TooltipDelayGroup,
    handle: Option<TooltipHandle<P>>,
    on_open_change: Option<TooltipOpenChangeHandler<P>>,
    on_open_change_complete: Option<TooltipOpenChangeCompleteHandler<P>>,
    style_with_state: Option<TooltipRootStyle<P>>,
}

impl<P: Clone + 'static> Default for TooltipRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tooltip"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            default_trigger_id: None,
            trigger_id: None,
            disabled: false,
            disable_hoverable_popup: false,
            track_cursor_axis: TooltipTrackCursorAxis::None,
            provider_config: TooltipProviderConfig::default(),
            delay_group: TooltipDelayGroup::default(),
            handle: None,
            on_open_change: None,
            on_open_change_complete: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for TooltipRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for TooltipRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let root_id = self.id.clone();
        let controlled_trigger_id = self.trigger_id.clone().map(|trigger_id| {
            trigger_id.map(|trigger_id| scoped_trigger_id(&root_id, &trigger_id))
        });
        let default_trigger_id = self
            .default_trigger_id
            .clone()
            .map(|trigger_id| scoped_trigger_id(&root_id, &trigger_id));
        let context = TooltipContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            controlled_trigger_id,
            default_trigger_id,
            TooltipProps::new(
                self.disabled,
                self.disable_hoverable_popup,
                self.track_cursor_axis,
                self.provider_config,
                self.on_open_change,
                self.on_open_change_complete,
            )
            .with_delay_group(self.delay_group),
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
        let delay_group = context.read(cx, |_runtime, props| props.delay_group());
        let (disabled_close, focus_change, missing_trigger_close, sibling_open_close) = context
            .update(cx, |runtime| {
                let disabled_close = runtime.sync_root_options(
                    self.disabled,
                    self.disable_hoverable_popup,
                    self.track_cursor_axis,
                );
                runtime
                    .sync_provider_delay_group(delay_group.instant(), delay_group.active_root_id());
                runtime.begin_detached_trigger_collection();
                runtime.sync_triggers(triggers);
                let focus_change = runtime.sync_focused_trigger(focused_trigger_id);
                let missing_trigger_close = runtime.take_active_trigger_missing_close_request();
                let sibling_open_close =
                    self.open.is_none() && runtime.should_close_for_provider_handoff(&root_id);
                (
                    disabled_close,
                    focus_change,
                    missing_trigger_close,
                    sibling_open_close,
                )
            });

        if disabled_close {
            context.close(
                TooltipOpenChangeReason::Disabled,
                TooltipOpenChangeSource::None,
                window,
                cx,
            );
        }
        match focus_change {
            TooltipFocusChange::Open(trigger_id) => {
                context.open_trigger(
                    trigger_id,
                    TooltipOpenChangeReason::TriggerFocus,
                    TooltipOpenChangeSource::Focus,
                    window,
                    cx,
                );
            }
            TooltipFocusChange::Close => {
                context.close(
                    TooltipOpenChangeReason::TriggerFocus,
                    TooltipOpenChangeSource::Focus,
                    window,
                    cx,
                );
            }
            TooltipFocusChange::None => {}
        }
        if missing_trigger_close {
            context.close(
                TooltipOpenChangeReason::TriggerHover,
                TooltipOpenChangeSource::None,
                window,
                cx,
            );
        }
        if sibling_open_close {
            context.close(
                TooltipOpenChangeReason::None,
                TooltipOpenChangeSource::None,
                window,
                cx,
            );
        }

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.children(children)
    }
}

impl<P: Clone + 'static> TooltipRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<TooltipChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<TooltipChild<P>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(TooltipChild::Any(child.into_any_element()));
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
        on_open_change: impl Fn(bool, &mut TooltipOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &TooltipOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn disable_hoverable_popup(mut self, disable_hoverable_popup: bool) -> Self {
        self.disable_hoverable_popup = disable_hoverable_popup;
        self
    }

    pub fn track_cursor_axis(mut self, track_cursor_axis: TooltipTrackCursorAxis) -> Self {
        self.track_cursor_axis = track_cursor_axis;
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

    pub fn handle(mut self, handle: TooltipHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TooltipRootStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
