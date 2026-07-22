use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, ClickEvent, Div, ElementId, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role, SharedString,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::toolbar::{
    child_wiring::toolbar_item_focus_handle, ToolbarActivateFocused, ToolbarButtonStyleState,
    ToolbarContext, ToolbarOrientation, TOOLBAR_ITEM_KEY_CONTEXT,
};

type ToolbarClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// A pressable toolbar item. Designed to later host trigger-style children
/// (menu/select/dialog triggers) without changing the toolbar registration
/// contract.
#[derive(IntoElement)]
pub struct ToolbarButton {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    disabled: bool,
    focusable_when_disabled: bool,
    aria_label: Option<SharedString>,
    on_click: Option<ToolbarClickHandler>,
    style_with_state: Option<Rc<dyn Fn(ToolbarButtonStyleState, Div) -> Div + 'static>>,
    toolbar: Option<(ToolbarContext, usize, FocusHandle, bool)>,
}

impl Default for ToolbarButton {
    fn default() -> Self {
        Self {
            id: ElementId::from("toolbar-button"),
            base: div(),
            children: Vec::new(),
            disabled: false,
            focusable_when_disabled: true,
            aria_label: None,
            on_click: None,
            style_with_state: None,
            toolbar: None,
        }
    }
}

impl Styled for ToolbarButton {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for ToolbarButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ToolbarButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (context, index, focus_handle, cascade_disabled) = match self.toolbar {
            Some((context, index, focus_handle, cascade_disabled)) => {
                (Some(context), Some(index), focus_handle, cascade_disabled)
            }
            None => (
                None,
                None,
                toolbar_item_focus_handle(&self.id, window, cx),
                false,
            ),
        };

        let disabled = self.disabled || cascade_disabled;
        let focusable = self.focusable_when_disabled;
        let orientation = context
            .as_ref()
            .map(|context| context.read(cx, |_runtime, props| props.orientation()))
            .unwrap_or(ToolbarOrientation::Horizontal);
        let highlighted = match (&context, index) {
            (Some(context), Some(index)) => {
                context.read(cx, |runtime, _props| runtime.is_tab_stop(index))
            }
            _ => true,
        };
        let tab_stop = highlighted && (!disabled || focusable);
        let focused = focus_handle.is_focused(window);

        let style_state =
            ToolbarButtonStyleState::new(disabled, orientation, focusable, focused, tab_stop);
        let base = match self.style_with_state {
            Some(style) => style(style_state, self.base),
            None => self.base,
        };

        let keyboard_handler = self.on_click.clone();
        let pointer_handler = self.on_click;
        let pointer_focus_handle = focus_handle.clone();
        let pointer_focusable = !disabled || focusable;

        // `Role::Button` puts the item in the a11y tree; `Action::Click` and
        // `Action::Focus` are auto-registered by `.on_click` / `.track_focus`
        // below. Merged `disabled` cannot be exposed to AT in this gpui
        // revision (no disabled a11y builder); the Click path stays registered
        // but inert while disabled.
        base.id(self.id)
            .role(Role::Button)
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
                activate(disabled, keyboard_handler.as_ref(), window, cx);
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                if pointer_focusable {
                    pointer_focus_handle.focus(window, cx);
                }

                if disabled {
                    return;
                }

                if let Some(on_click) = pointer_handler.as_ref() {
                    on_click(event, window, cx);
                }
            })
            .children(self.children)
    }
}

impl ToolbarButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn focusable_when_disabled(mut self, focusable_when_disabled: bool) -> Self {
        self.focusable_when_disabled = focusable_when_disabled;
        self
    }

    /// Accessible name for icon-only buttons. When set alongside a visible
    /// text child, render that text with `Text::new_inaccessible(...)` instead
    /// of `text!(...)` so screen readers do not announce the name twice;
    /// without an `aria_label`, keep `text!(...)` so the child text remains
    /// the accessible name source.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToolbarButtonStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// The button's own disabled prop, consumed by the toolbar child wiring
    /// when resolving the effective per-item disabled fact.
    pub fn own_disabled(&self) -> bool {
        self.disabled
    }

    /// The button's `focusable_when_disabled` flag, consumed by the toolbar
    /// child wiring for item metadata.
    pub fn own_focusable_when_disabled(&self) -> bool {
        self.focusable_when_disabled
    }

    /// The button's element id, consumed by the toolbar child wiring to key
    /// the roving focus handle.
    pub fn item_id(&self) -> &ElementId {
        &self.id
    }

    /// Attaches this button to a toolbar as a composite item. Called by the
    /// toolbar child wiring; not intended for direct use.
    pub fn with_toolbar(
        mut self,
        context: ToolbarContext,
        index: usize,
        focus_handle: FocusHandle,
        cascade_disabled: bool,
    ) -> Self {
        self.toolbar = Some((context, index, focus_handle, cascade_disabled));
        self
    }
}

/// The single keyboard/pointer activation gate: disabled buttons never fire.
fn activate(
    disabled: bool,
    on_click: Option<&ToolbarClickHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    if disabled {
        return;
    }

    if let Some(on_click) = on_click {
        on_click(&ClickEvent::default(), window, cx);
    }
}
