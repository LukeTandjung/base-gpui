use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Bounds, Div, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, ParentElement, Pixels, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::field::{
    child_wiring::wire_item_children, context::current_field_context,
    item_context::with_field_item_disabled, FieldContext, FieldItemChild, FieldItemStyleState,
};

#[derive(IntoElement)]
pub struct FieldItem {
    base: Div,
    children: Vec<FieldItemChild>,
    context: Option<FieldContext>,
    disabled: bool,
    style_with_state: Option<Rc<dyn Fn(FieldItemStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldItem {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            disabled: false,
            style_with_state: None,
        }
    }
}

impl Styled for FieldItem {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self.context.or_else(current_field_context);
        let root_state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)))
            .unwrap_or_default();
        let disabled = root_state.disabled || self.disabled;
        let state = FieldItemStyleState::new(root_state, disabled);
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        FieldItemScopeElement {
            disabled,
            inner: base.children(self.children).into_any_element(),
        }
    }
}

impl FieldItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<FieldItemChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<FieldItemChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(FieldItemChild::Any(child.into_any_element()));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_field_context(mut self, context: FieldContext) -> Self {
        self.context = Some(context.clone());
        self.children = wire_item_children(self.children, context);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldItemStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

struct FieldItemScopeElement {
    disabled: bool,
    inner: AnyElement,
}

impl IntoElement for FieldItemScopeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for FieldItemScopeElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id =
            with_field_item_disabled(self.disabled, || self.inner.request_layout(window, cx));

        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        with_field_item_disabled(self.disabled, || {
            self.inner.prepaint(window, cx);
        });
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        with_field_item_disabled(self.disabled, || {
            self.inner.paint(window, cx);
        });
    }
}
