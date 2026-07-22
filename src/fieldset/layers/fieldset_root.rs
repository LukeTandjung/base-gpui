use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Bounds, Div, Element, ElementId, GlobalElementId, InspectorElementId,
    InteractiveElement, IntoElement, LayoutId, ParentElement, Pixels, RenderOnce, Role,
    SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
};

use crate::fieldset::{
    child_wiring::wire_children, context::with_fieldset_context, current_fieldset_disabled,
    FieldsetChild, FieldsetContext, FieldsetProps, FieldsetRootStyleState,
};

#[derive(IntoElement)]
pub struct FieldsetRoot {
    id: ElementId,
    base: Div,
    children: Vec<FieldsetChild>,
    disabled: bool,
    label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(FieldsetRootStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldsetRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("fieldset"),
            base: div(),
            children: Vec::new(),
            disabled: false,
            label: None,
            style_with_state: None,
        }
    }
}

impl Styled for FieldsetRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldsetRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled || current_fieldset_disabled();
        let context =
            FieldsetContext::new(self.id.clone(), cx, window, FieldsetProps::new(disabled));
        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let children = wire_children(self.children, context.clone());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };
        // AccessKit: a node needs both a stable id and a role to appear in the
        // a11y tree. Base UI's `aria-labelledby` (root -> legend id) has no gpui
        // relationship builder, so the literal `.aria_label(...)` substitutes for
        // it; callers must keep it in sync with the legend text manually. The
        // `disabled` / `aria-disabled` cascade is also not exposed to AT (no
        // `.aria_disabled(...)` builder in this gpui revision) — descendant
        // controls suppress interaction instead.
        let mut base = base.id(self.id.clone()).role(Role::Group);
        if let Some(label) = self.label {
            base = base.aria_label(label);
        }

        FieldsetScopeElement {
            context,
            inner: base.children(children).into_any_element(),
        }
    }
}

impl FieldsetRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<FieldsetChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<FieldsetChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(FieldsetChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Accessible label for the group, announced by screen readers.
    ///
    /// Substitutes for Base UI's `aria-labelledby` -> legend-id wiring, which
    /// gpui cannot express: pass the legend's text here and keep it in sync
    /// manually. When set, build the visible `FieldsetLegend` text with
    /// `Text::new_inaccessible(...)` so it is not announced twice.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldsetRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

struct FieldsetScopeElement {
    context: FieldsetContext,
    inner: AnyElement,
}

impl IntoElement for FieldsetScopeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for FieldsetScopeElement {
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
        let layout_id = with_fieldset_context(self.context.clone(), || {
            self.inner.request_layout(window, cx)
        });

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
        with_fieldset_context(self.context.clone(), || {
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
        with_fieldset_context(self.context.clone(), || {
            self.inner.paint(window, cx);
        });
    }
}
