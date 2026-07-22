use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AccessibleAction, AnyElement, App, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::navigation_menu::{
    child_wiring::{scoped_value_id, NavigationMenuChildNode, NavigationMenuChildWiring},
    NavigationMenuContext, NavigationMenuLinkStyleState, NavigationMenuListEntry,
    NavigationMenuValueChangeReason, NavigationMenuValueChangeSource,
};

type NavigationMenuLinkStyle = Rc<dyn Fn(NavigationMenuLinkStyleState, Div) -> Div + 'static>;
type NavigationMenuLinkActivateHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// Link item: GPUI has no href, so activation is a callback. With
/// `close_on_click(true)` a click closes the menu with reason `LinkPress`.
///
/// Accessibility: `Role::Link` with a caller-provided `.aria_label(...)`.
/// AT `Click` routes through the same `on_activate` + `LinkPress` close path
/// as the pointer handler. Base UI's `aria-current="page"` has no gpui
/// builder; `active` stays a visual-only fact (omitted, blocked pending gpui
/// upstream — `aria_selected` is deliberately not abused for it).
#[derive(IntoElement)]
pub struct NavigationMenuLink<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<NavigationMenuContext<T>>,
    active: bool,
    close_on_click: bool,
    focus_handle: Option<FocusHandle>,
    order: usize,
    on_activate: Option<NavigationMenuLinkActivateHandler>,
    aria_label: Option<SharedString>,
    style_with_state: Option<NavigationMenuLinkStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuLink<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            active: false,
            close_on_click: false,
            focus_handle: None,
            order: 0,
            on_activate: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for NavigationMenuLink<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuLink<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuLink<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.link_state(self.active)))
            .unwrap_or_else(|| NavigationMenuLinkStyleState::new(self.active));

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let context = self.context.clone();
        let close_on_click = self.close_on_click;
        let on_activate = self.on_activate.clone();
        let a11y_context = self.context.clone();
        let a11y_on_activate = self.on_activate.clone();
        let mut base = base
            .id(("navigation-menu-link", self.order))
            .role(Role::Link)
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                if let Some(on_activate) = on_activate.as_ref() {
                    on_activate(window, cx);
                }
                if close_on_click {
                    if let Some(context) = context.as_ref() {
                        context.close(
                            NavigationMenuValueChangeReason::LinkPress,
                            NavigationMenuValueChangeSource::Pointer,
                            window,
                            cx,
                        );
                    }
                }
            })
            // AT-dispatched Click: same activate + LinkPress close path as
            // the pointer handler.
            .on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
                if let Some(on_activate) = a11y_on_activate.as_ref() {
                    on_activate(window, cx);
                }
                if close_on_click {
                    if let Some(context) = a11y_context.as_ref() {
                        context.close(
                            NavigationMenuValueChangeReason::LinkPress,
                            NavigationMenuValueChangeSource::Unknown,
                            window,
                            cx,
                        );
                    }
                }
            });
        if let Some(aria_label) = self.aria_label.clone() {
            base = base.aria_label(aria_label);
        }
        if let Some(focus_handle) = self.focus_handle.as_ref() {
            base = base.track_focus(&focus_handle.clone().tab_stop(true).tab_index(0));
        }

        base.children(self.children)
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuLink<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_navigation_menu_child(
        mut self,
        wiring: &mut NavigationMenuChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let order = wiring.register_list_entry(NavigationMenuListEntry::new(None, None, false));
        let focus_handle = link_focus_handle(
            &scoped_value_id(&wiring.root_id(), "navigation-menu-link", order),
            window,
            cx,
        );
        wiring.set_entry_focus_handle(order, focus_handle.clone());
        self.focus_handle = Some(focus_handle);
        self.order = order;
        self
    }
}

fn link_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}

impl<T: Clone + Eq + 'static> NavigationMenuLink<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn close_on_click(mut self, close_on_click: bool) -> Self {
        self.close_on_click = close_on_click;
        self
    }

    /// Accessible name for the link; pass one when the visible caption is not
    /// plain accessible text (no id-reference labelling exists). Render the
    /// visible caption with `Text::new_inaccessible(...)` when set, to avoid
    /// double announcement.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_activate(mut self, on_activate: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_activate = Some(Rc::new(on_activate));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuLinkStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
