use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::toolbar::{
    child_wiring::toolbar_item_focus_handle, ToolbarActivateFocused, ToolbarContext,
    ToolbarLinkStyleState, ToolbarOrientation, TOOLBAR_ITEM_KEY_CONTEXT,
};

type ToolbarClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A plain focusable styled item with an `on_click` action (no anchor
/// semantics). Links can never be disabled: they ignore the toolbar/group
/// disabled cascade and always occupy a roving slot.
#[derive(IntoElement)]
pub struct ToolbarLink {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    aria_label: Option<SharedString>,
    on_click: Option<ToolbarClickHandler>,
    style_with_state: Option<Rc<dyn Fn(ToolbarLinkStyleState, Div) -> Div + 'static>>,
    toolbar: Option<(ToolbarContext, usize, FocusHandle)>,
}

impl Default for ToolbarLink {
    fn default() -> Self {
        Self {
            id: ElementId::from("toolbar-link"),
            base: div(),
            children: Vec::new(),
            aria_label: None,
            on_click: None,
            style_with_state: None,
            toolbar: None,
        }
    }
}

impl Styled for ToolbarLink {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for ToolbarLink {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToolbarLink {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (context, index, focus_handle) = match self.toolbar {
            Some((context, index, focus_handle)) => (Some(context), Some(index), focus_handle),
            None => (None, None, toolbar_item_focus_handle(&self.id, window, cx)),
        };

        let orientation = context
            .as_ref()
            .map(|context| context.read(cx, |_runtime, props| props.orientation()))
            .unwrap_or(ToolbarOrientation::Horizontal);
        let tab_stop = match (&context, index) {
            (Some(context), Some(index)) => {
                context.read(cx, |runtime, _props| runtime.is_tab_stop(index))
            }
            _ => true,
        };
        let focused = focus_handle.is_focused(window);

        let style_state = ToolbarLinkStyleState::new(orientation, focused, tab_stop);
        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        let keyboard_handler = self.on_click.clone();
        let pointer_handler = self.on_click;
        let pointer_focus_handle = focus_handle.clone();

        // `Role::Link` puts the item in the a11y tree; `Action::Click` and
        // `Action::Focus` are auto-registered by `.on_click` / `.track_focus`
        // below.
        base.id(self.id)
            .role(Role::Link)
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .track_focus(
                &focus_handle
                    .tab_stop(tab_stop)
                    .tab_index(if tab_stop { 0 } else { -1 }),
            )
            .key_context(TOOLBAR_ITEM_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &ToolbarActivateFocused, window, cx| {
                if let Some(on_click) = keyboard_handler.as_ref() {
                    on_click(&ClickEvent::default(), window, cx);
                }
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                pointer_focus_handle.focus(window, cx);

                if let Some(on_click) = pointer_handler.as_ref() {
                    on_click(event, window, cx);
                }
            })
            .children(self.children)
    }
}

impl ToolbarLink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    /// Accessible name for icon-only links. When set alongside a visible text
    /// child, render that text with `Text::new_inaccessible(...)` instead of
    /// `text!(...)` so screen readers do not announce the name twice; without
    /// an `aria_label`, keep `text!(...)` so the child text remains the
    /// accessible name source.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToolbarLinkStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// The link's element id, consumed by the toolbar child wiring to key
    /// the roving focus handle.
    pub fn item_id(&self) -> &ElementId {
        &self.id
    }

    /// Attaches this link to a toolbar as a composite item. Called by the
    /// toolbar child wiring; not intended for direct use.
    pub fn with_toolbar(
        mut self,
        context: ToolbarContext,
        index: usize,
        focus_handle: FocusHandle,
    ) -> Self {
        self.toolbar = Some((context, index, focus_handle));
        self
    }
}
