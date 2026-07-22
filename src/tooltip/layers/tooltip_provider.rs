use std::{rc::Rc, time::Duration};

use gpui::{
    div, App, Div, ElementId, Entity, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window,
};

use crate::tooltip::{
    TooltipDelayGroup, TooltipProviderChild, TooltipProviderConfig, TooltipProviderStyleState,
};

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
