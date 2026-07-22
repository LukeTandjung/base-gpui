use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::toolbar::{
    ToolbarContext, ToolbarGroupChild, ToolbarGroupStyleState, ToolbarOrientation,
};

/// A plain grouping container: it has no focus handle and no roving slot.
/// Its children participate in the toolbar's roving order exactly as if they
/// were direct toolbar children (flattened indices), and its disabled state
/// merges with the toolbar's and cascades to contained buttons and inputs
/// (never links).
#[derive(IntoElement)]
pub struct ToolbarGroup {
    id: ElementId,
    base: Div,
    children: Vec<ToolbarGroupChild>,
    disabled: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(ToolbarGroupStyleState, Div) -> Div + 'static>>,
    toolbar: Option<(ToolbarContext, bool)>,
}

impl Default for ToolbarGroup {
    fn default() -> Self {
        Self {
            id: ElementId::from("toolbar-group"),
            base: div(),
            children: Vec::new(),
            disabled: false,
            aria_label: None,
            style_with_state: None,
            toolbar: None,
        }
    }
}

impl Styled for ToolbarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ToolbarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (orientation, merged_disabled) = match &self.toolbar {
            Some((context, merged_disabled)) => (
                context.read(cx, |_runtime, props| props.orientation()),
                *merged_disabled,
            ),
            None => (ToolbarOrientation::Horizontal, self.disabled),
        };

        let style_state = ToolbarGroupStyleState::new(merged_disabled, orientation);
        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        // The stable id plus `Role::Group` mirror Base UI's `role="group"`.
        // Merged `disabled` stays style-state-only: this gpui revision has no
        // disabled a11y builder.
        base.id(self.id)
            .role(Role::Group)
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .children(self.children)
    }
}

impl ToolbarGroup {
    pub fn new() -> Self {
        Self::default()
    }

    /// Overrides the default `"toolbar-group"` element id. Give each group in
    /// a window a distinct, stable id so assistive technology sees stable
    /// accessibility nodes across frames.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<ToolbarGroupChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<ToolbarGroupChild>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Accessible name for the group, announced by screen readers. There is
    /// no `aria-labelledby` id-reference builder in this gpui revision, so
    /// the name is a literal string.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToolbarGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// The group's own disabled prop, consumed by the toolbar child wiring
    /// when computing the merged cascade for contained items.
    pub fn own_disabled(&self) -> bool {
        self.disabled
    }

    /// Detaches the typed children so the toolbar child wiring can flatten
    /// them into the toolbar's single item order. Called by the toolbar
    /// child wiring; not intended for direct use.
    pub fn split_children(mut self) -> (Self, Vec<ToolbarGroupChild>) {
        let children = std::mem::take(&mut self.children);

        (self, children)
    }

    /// Reattaches the wired children and the toolbar context. Called by the
    /// toolbar child wiring; not intended for direct use.
    pub fn with_toolbar(
        mut self,
        context: ToolbarContext,
        merged_disabled: bool,
        children: Vec<ToolbarGroupChild>,
    ) -> Self {
        self.toolbar = Some((context, merged_disabled));
        self.children = children;
        self
    }
}
